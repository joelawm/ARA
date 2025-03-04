use std::fmt::Debug;
use crate::graph::{node::{Node, NodeType}, edge::Edge, layer::Layer};
use crate::log::debug::{error, warn, debug};

pub mod node;
pub mod edge;
pub mod layer;

#[derive(Debug)]
pub struct Graph {
	pub nodes: Vec<Node>,
	pub edges: Vec<Edge>,
	pub calls: Vec<Layer>,
	pub layer: i16,
	pub layer_args: i16,
}

impl Graph {
	pub fn new() -> Graph {
		Graph {nodes: Vec::new(), edges: Vec::new(), calls: Vec::new(), layer: 0, layer_args: 0}
	}
	/// This method adds a node to the graph and returns a reference to the node.
	pub fn add_node(&mut self, mut node: Node) -> &Node {
		// Update the id of the node
		let id = self.get_node_len();
		node.update_id(id);

		debug(&format!("Calls {:?}", self.calls).to_string());
		debug(&format!("Adding node: {:?}", node).to_string());

		self.calls.push(Layer::new(id, self.layer, self.layer_args));
		self.nodes.push(node);
		match self.nodes.last() {
			Some(node) => node,
			None => {
				error(&"Node not found".to_string());
				panic!("Exiting...");
			},
		}
	}
	/// This method adds an edge to the graph.
	pub fn add_edge(&mut self, to: i16) {
		let from = match self.get_offset() {
			Some(offset) => offset,
			None => {
				warn(&format!("Failed to get offset for to {}", to));
				return
			},
		};

		debug(&format!("Adding edge from: {} to: {}", from.id, to));

		self.edges.push(Edge::new(from.id, to));
	}
	/// This method adds a literal a node.
	pub fn add_literal(&mut self, literal: &str) {
		let layer_id = match self.get_last_layer(){
			Some(layer) => layer.id,
			None => {
				warn(&format!("Failed to get last layer and couldn't add {}", literal));
				return
			},
		};

		if let Some(node) = self.get_node_mut(layer_id) {
			node.add_literal(literal);
		}
	}
	/// This method adds a local to a node.
	pub fn add_local(&mut self, local: &str) {
		let node_id = match self.get_first() {
			Some(id) => id,
			None => {
				warn(&format!("Failed to get first node and couldn't add {}", local));
				return
			},
		};

		if let Some(node) = self.get_node_mut(*node_id) {
			node.add_local(local);
		}
	}
	/// This function is used to grab the offset of the last layer.
	/// Example:
	/// function {
	/// 	call(arg1, arg2)
	/// }
	/// Function is layer 0, Call is layer 1, arg1 and arg2 is layer 2
	pub fn get_offset(&self) -> Option<&Layer> {
		let last_call = match self.get_last_layer() {
			Some(call) => call,
			None => {
				warn(&"Failed to get last call".to_string());
				return None
			},
		};

		let most_recent_node = match self.get_node(last_call.id) {
			Some(node) => node,
			None => {
				warn(&"Failed to get most recent call".to_string());
				return None
			},
		};

		// If the most recent call is a method we need to attach to most recent call or Local
		if most_recent_node.node_type == NodeType::Method {
			for calls in self.calls.iter().rev() {
				let node_type = match self.get_node(calls.id) {
					Some(node) => &node.node_type,
					None => {
						warn(&"Failed to get node type".to_string());
						return None
					}
				};
		
				if (node_type == &NodeType::Local ||  node_type == &NodeType::Call) 
				&& calls.args == last_call.args && calls.layer == last_call.layer {
					return Some(&calls);
				}
			}
		}
    
		// If its not a method we can iterate through the calls and check layers
		for (i, calls) in self.calls.iter().enumerate() {
			let offset = i + 1;

			if offset == self.calls.len() {
				return Some(&self.calls[self.calls.len() - 2])
			}
			if calls.args != self.calls[offset].args && calls.layer == self.calls[offset].layer {
				return Some(&calls);
			}
		}
		None
	}
	/// Get the last call in the call stack.
	pub fn get_last_layer(&self) -> Option<&Layer> {
		self.calls.last()
	}
	/// Get the first call in the call stack where the layer is the current layer.
	pub fn get_first(&self) -> Option<&i16> {
		for calls in self.calls.iter() {
			if calls.layer == self.layer {
				return Some(&calls.id);
			}
		}
		None
	}
	/// Get a node by its id.
	pub fn get_node_mut(&mut self, id: i16) -> Option<&mut Node> {
		self.nodes.iter_mut().find(|node| node.id == id)
	}
	/// Get a node by its id.
	pub fn get_node(&self, id: i16) -> Option<&Node> {
		self.nodes.iter().find(|node| node.id == id)
	}
	/// Get the depth of the call stack in the graph.
	pub fn get_depth(&self) -> i16 {
		self.calls.len() as i16
	}
	/// Get the length of the nodes in the graph.
	pub fn get_node_len(&self) -> i16 {
		self.nodes.len() as i16
	}
	/// Check to see if a local exists in the graph.
	pub fn local_exists(&self, local: &str) -> bool {
		for node in &self.nodes {
			if node.local == local {
				return true;
			}
		}
		false
	}
	/// Clear all the calls from the current layer.
	pub fn clear_calls_layer(&mut self) {
		self.calls.retain(|v| v.layer != self.layer);
	}
	/// Clear all of the calls and retain nothing.
	pub fn clear_calls(&mut self) {
		self.calls.clear();
	}
	/// Increase the layer of the graph.
	pub fn increase_layer(&mut self) {
		self.layer += 1;
	}
	/// Decrease the layer of the graph.
	pub fn decrease_layer(&mut self) {
		self.layer -= 1;
	}
	/// Increase the layer arguments of the graph.
	pub fn increase_layer_args(&mut self) {
		self.layer_args += 1;
	}
	/// Decrease the layer arguments of the graph.
	pub fn decrease_layer_args(&mut self) {
		self.layer_args -= 1;
	}
} 