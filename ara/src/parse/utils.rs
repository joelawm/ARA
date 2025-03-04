/*-------------
/parse/utils.rs

Utility functions for helping remove boilerplate code from the main logic of the parser.
-------------*/
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{ExprPath, FnArg, ItemFn};

/// Get the arguments of a function and return them as a vector of strings
pub fn get_function_arguments(input: Punctuated<FnArg, Comma>) -> Vec<String> {
	let mut args = Vec::new();

	for arg in &input {
		if let syn::FnArg::Typed(arg) = arg {
			// if let syn::Type::Path(ref path) = *arg.ty {
			// 	println!("TYPE: {}", quote! { #path }.to_string());
			// }

			if let syn::Pat::TupleStruct(ref pat) = *arg.pat {
				pat.elems.iter().for_each(|el| {
					if let syn::Pat::Ident(ref pat) = el {
						args.push(pat.ident.to_string());
					}
				});
				println!("{:?}", &pat.path.get_ident());
			}

			if let syn::Pat::Ident(ref pat) = *arg.pat {
				args.push(pat.ident.to_string());
			}
		}
	}

	args
}

/// Get document comments from an attr
pub fn get_doc_comments(function: &ItemFn) -> Vec<String> {
	let mut comments = Vec::new();
	for attr in &function.attrs {
		let comment = if attr.path().is_ident("doc") {
			Some(attr.meta.require_name_value().unwrap().value.clone())
		} else {
			None
		};

		let comment = match comment {
			Some(comment) => comment,
			None => continue,
		}; 

		if let syn::Expr::Lit(expr_lit) = comment {
			if let syn::Lit::Str(lit) = &expr_lit.lit {
				comments.push(lit.value());
			}
		}
	}
	comments
}

/// Get the local variable name
pub fn get_local(i: &syn::Local) -> Option<String> {
	match i.pat {
		syn::Pat::Ident(ref pat) => {
			return Some(pat.ident.to_string())
		},
		syn::Pat::Type(ref pat) => {
			if let syn::Pat::Ident(ref pat) = *pat.pat {
				return Some(pat.ident.to_string())
			}
			None
		},
		syn::Pat::Tuple(ref pat) => {
			for el in pat.elems.iter() {
				if let syn::Pat::Ident(ref pat) = el {
					return Some(pat.ident.to_string())
				}
			}
			None
		},
		_ => None
	}
}

/// Create a path from an expression path
pub fn create_path(path: &ExprPath) -> String {
	path.path.segments.iter().map(|p| p.ident.to_string()).collect::<Vec<_>>().join("::")
}