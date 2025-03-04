/*-------------
/parse/mod.rs

Parse is the file that does the bulk of the lifting to make a call stack of the functions to be used in the graph connection layer. The actual connecting of the graph is not done here, but the data is collected to here to then
be stitched together in the engine. The main logic is in the visit_expr function, which is the main function that traverses the block. The visit_expr_method_call and visit_expr_call are the other 2 points
of interested followed by visit_expr which contains the routing logic. All other functions are helper functions to make the code more readable and to keep the main logic clean.
-------------*/
use syn::{visit::Visit, ItemFn};
use crate::graph::node::{Node, NodeType};
use crate::state::State;
use crate::log::{self, info};
use crate::config::APP;

mod utils;

impl<'ast> Visit<'ast> for State {
	/// Visit functions and add them to the calls
	fn visit_item_fn(&mut self, func: &'ast ItemFn) {
		if !APP.function_name.contains(&func.sig.ident.to_string()) && !APP.function_name.is_empty() {
			return
		}

		// Create the call stack with the function as the first part
		//let args = utils::get_function_arguments(func.sig.inputs.clone());
		let name = func.sig.ident.to_string();
		let path = self.current_file.clone();
		let comments = utils::get_doc_comments(func);

		let path_name = format!("{}::{}", path, name);
		let mut node = Node::new(&path_name, NodeType::Function);
		let node = node.add_comments(comments);
		self.graph.add_node(node.clone());

		log::info::print_visit_function(&func.sig.ident.to_string(), &self.current_file);
		self.visit_block(&func.block);

		self.graph.clear_calls();

		// Debug
		println!("Graph: {:#?}", self.graph);
    }

	/// Visits the block of the code and determines for each statement where the block is called to
	fn visit_block(&mut self, block: &'ast syn::Block) {
		self.graph.increase_layer();
		for stmt in &block.stmts {
			if self.graph.get_depth() == 1 {
			 	info::print_loc(&quote! { #stmt }.to_string());
			}

			if let syn::Stmt::Local(local) = stmt {
				self.visit_local(local);
			}

			if let syn::Stmt::Expr(expr, _) = stmt {
				self.visit_expr(expr);
			}
			self.graph.clear_calls_layer();
		}
		self.graph.decrease_layer();
	}

	/// Main Logic for visiting and traversing the syntax tree
	fn visit_expr(&mut self, i: &'ast syn::Expr) {
		log::debug::print_expr(&quote! { #i }.to_string());
		
		if let syn::Expr::Match(ref call) = &*i {
			self.visit_expr_match(call);
		}

		if let syn::Expr::Try(ref call) = &*i {
			self.visit_expr(&call.expr);
		}

		if let syn::Expr::ForLoop(call) = &i {
			self.visit_expr(&call.expr);
		}

		if let syn::Expr::While(call) = &i {
			self.visit_expr(&call.cond);
		}

		if let syn::Expr::Block(ref call) = &*i {
			self.visit_block(&call.block);
		}

		if let syn::Expr::Reference(ref call) = &*i {
			self.visit_expr(&call.expr);
		}

		if let syn::Expr::Binary(ref call) = &*i {
			self.visit_expr(&call.left);
			self.visit_expr(&call.right);
		}

		if let syn::Expr::Return(ref call) = &*i {
			self.visit_expr_return(call);
		}

		if let syn::Expr::Await(ref call) = &*i {
			self.visit_expr(&call.base);
		}

		if let syn::Expr::If(ref call) = &*i {
			self.visit_expr_if(call);
		}

		if let syn::Expr::Tuple(ref _call) = &*i {
			let node = Node::new("(,)", NodeType::Tuple);
			let node_id = self.graph.add_node(node).id;
			self.graph.add_edge(node_id);

			self.graph.increase_layer_args();
			for el in _call.elems.iter() {
				self.visit_expr(el);
			}
			self.graph.decrease_layer_args();
		}

		if let syn::Expr::Call(ref call) = &*i {
			self.visit_expr_call(call);
		}

		if let syn::Expr::Paren(ref call) = &*i {
			let node = Node::new("()", NodeType::Paren);
			let node_id = self.graph.add_node(node).id;
			self.graph.add_edge(node_id);

			self.graph.increase_layer_args();
			self.visit_expr(&call.expr);
			self.graph.decrease_layer_args();
		}
 
		if let syn::Expr::MethodCall(ref call) = &*i {
			self.visit_expr_method_call(call);
		}


		// if self.calls.get_biggest_depth() == 0 {
		// 	if let syn::Expr::Field(ref call) = &*i {
		// 		if let syn::Expr::Path(ref path) = &*call.base {
		// 			self.calls.add_call(&utils::create_path(path), CallType::Field);
		// 		}
		// 		self.visit_expr(&call.base);
		// 		if let syn::Member::Named(ident) = &call.member {
		// 			println!("Field: {}", ident.to_string());
		// 			self.calls.add_field(ident.to_string());
		// 		}
		// 	}
		// }

		if let syn::Expr::Struct(ref call) = &*i {
			let path = call.path.segments.iter().map(|f| f.ident.to_string()).collect::<Vec<_>>().join("");
			let node = Node::new(&path, NodeType::Struct);
			self.graph.add_node(node);

			for field in &call.fields {
				self.visit_expr(&field.expr);
			}
		}

		if let syn::Expr::Macro(ref call) = &*i {
			let path = &call.mac.path;
			let expr_path = syn::ExprPath { attrs: Vec::new(), qself: None, path: path.clone() };
			let node = Node::new(&utils::create_path(&expr_path), NodeType::Macro);
			let node_id = self.graph.add_node(node).id;
			self.graph.add_edge(node_id);
		}

		if let syn::Expr::Path(ref path) = &*i {
			// Check if the path is a local variable
			let node = if self.graph.local_exists(&utils::create_path(path)) {
				Node::new(&utils::create_path(path), NodeType::Local)
			} else {
				Node::new(&utils::create_path(path), NodeType::Call)
			};
			let node_id = self.graph.add_node(node).id;
			self.graph.add_edge(node_id);
		}

		if let syn::Expr::Lit(ref call) = &*i {
			self.graph.add_literal(&quote!{#call}.to_string())
		}
	}

	/// Visit a method call and add it to the nodes
	fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
		self.visit_expr(&i.receiver);

		let node = Node::new(&i.method.to_string(), NodeType::Method);
		let node_id = self.graph.add_node(node).id;
		self.graph.add_edge(node_id);

		self.graph.increase_layer_args();
		for arg in &i.args {
			self.visit_expr(arg);
		}
		self.graph.decrease_layer_args();
	}

	/// Visit a function call and add it to the nodes
	fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
		self.visit_expr(&i.func);
		self.graph.increase_layer_args();
		for arg in &i.args {
			self.visit_expr(arg);
		}
		self.graph.decrease_layer_args();
	}

	///
	fn visit_arm(&mut self, arm: &'ast syn::Arm) {
		self.visit_expr(&arm.body);
	}

	/// 
	fn visit_expr_return(&mut self, i: &'ast syn::ExprReturn) {
		if let Some(expr) = &i.expr {
			self.visit_expr(expr);
		}
	}

	///
	fn visit_expr_match(&mut self, call: &'ast syn::ExprMatch) {
		if let syn::Expr::Call(call) = &*call.expr {
			self.visit_expr_call(call);
		}

		if let syn::Expr::MethodCall(call) = &*call.expr {
			self.visit_expr_method_call(call);
		}

		if let syn::Expr::Await(call) = &*call.expr {
			self.visit_expr(&call.base);
		}

		if let syn::Expr::Block(call) = &*call.expr {
			self.visit_block(&call.block);
		}

		for arm in &call.arms {
			self.visit_arm(arm);
		}
	}

	///
	fn visit_expr_if(&mut self, call: &'ast syn::ExprIf) {
		self.visit_expr(&call.cond);

		if let syn::Expr::Unary(unary) = &*call.cond {
			self.visit_expr(&unary.expr);
		}

		if let syn::Expr::Let(let_expr) = &*call.cond {
			self.visit_expr(&let_expr.expr);
		}
		
		if let syn::Expr::Call(call) = &*call.cond {
			self.visit_expr_call(call);
		}

		if let syn::Expr::MethodCall(call) = &*call.cond {
			self.visit_expr_method_call(call);
		}

		if let Some((_, else_expr)) = &call.else_branch {
			self.visit_expr(else_expr);
		}

		self.visit_block(&call.then_branch);
	}

	/// Visit a local variable and get the expression
	fn visit_local(&mut self, i: &'ast syn::Local) {
		if let Some(init) = &i.init {
			self.visit_expr(&*init.expr);
		}

		if let Some(local) = utils::get_local(i) {
			self.graph.add_local(&local);
		}
	}

	/// Visit a struct and add it to the calls
	fn visit_item_struct(&mut self, _i: &'ast syn::ItemStruct) {
		//let route = format!("{}::{}", self.current_file, &i.ident.to_string());
		//let node = Node::new(self.graph.get_node_len(), route, NodeType::Struct);
		//self.graph.add_node(node);
	}

	/// Visit an enum and add it to the calls
	fn visit_item_enum(&mut self, _i: &'ast syn::ItemEnum) {
		//let route = format!("{}::{}", self.current_file, &i.ident.to_string());
		//let node = Node::new(self.graph.get_node_len(), route, NodeType::Enum);
		//self.graph.add_node(node);
	}

	///
	fn visit_item_impl(&mut self, _i: &'ast syn::ItemImpl) {
		//println!("Impl: {}", quote! { #i });
	}

	///
	fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
		let module = &i.ident.to_string();
		self.add_new_lib(&module);
	}

	///
	fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
		self.visit_use_tree(&i.tree);
		self.use_stack.clear();
	}

	///
	fn visit_use_tree(&mut self, i: &'ast syn::UseTree) {
		match &i {
			syn::UseTree::Path(path) => {
				self.use_stack.insert(path.ident.to_string());
				self.visit_use_tree(&path.tree);
				self.use_stack.pop_stack();
			},
			syn::UseTree::Group(group) => {
				self.use_stack.add_layer();
				for tree in &group.items {
					self.visit_use_tree(tree);
				}
				self.use_stack.clear_layer();
			}
			syn::UseTree::Name(name) => self.add_new_lib(&quote!{#name}.to_string()),
			_ => {}
		}
	}
}