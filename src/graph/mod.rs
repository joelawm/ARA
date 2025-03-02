use std::fmt::Debug;
use node::NodeType;

use crate::graph::node::Node;
use crate::graph::edge::Edge;
use crate::log;
use crate::log::debug::{error, warn};

pub mod node;
pub mod edge;

#[derive(Debug)]
pub struct Graph {
	pub nodes: Vec<Node>,
	pub edges: Vec<Edge>,
	pub calls: Vec<Layer>,
	pub layer: i16,
	pub layer_args: i16,
}

#[derive(Debug)]
pub struct Layer {
	id: i16,
	layer: i16,
	args: i16,
}

impl Layer {
	pub fn new(id: i16, layer: i16, args: i16) -> Layer {
		Layer {id, layer, args}
	}
}

impl Graph {
	pub fn new() -> Graph {
		Graph {nodes: Vec::new(), edges: Vec::new(), calls: Vec::new(), layer: 0, layer_args: 0}
	}

	/// This method adds a node to the graph and returns a reference to the node.
	pub fn add_node(&mut self, mut node: Node) -> &Node {
		log::debug::debug(&format!("Calls {:?}", self.calls).to_string());
		log::debug::debug(&format!("Adding node: {:?}", node).to_string());

		// Update the id of the node
		let id = self.get_node_len();
		node.update_id(id);

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

		log::debug::debug(&format!("Adding edge from: {} to: {}", from.id, to));

		self.edges.push(Edge::new(from.id, to));
	}
	pub fn add_literal(&mut self, literal: &str) {
		let layer_id = self.get_last().unwrap().id;
		if let Some(node) = self.get_node_mut(layer_id) {
			node.add_literal(literal);
		}
	}
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
		let most_recent_call = self.get_node(self.calls.last().unwrap().id).unwrap();

		// If the most recent call is a method we need to attach to most recent call or Local
		if most_recent_call.node_type == NodeType::Method {
			for calls in self.calls.iter().rev() {
				let node_type = &self.get_node(calls.id).unwrap().node_type;
				if (node_type == &NodeType::Local ||  node_type == &NodeType::Call) 
				&& calls.args == self.calls.last().unwrap().args 
				&& calls.layer == self.calls.last().unwrap().layer {
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
	pub fn get_last(&self) -> Option<&Layer> {
		self.calls.last()
	}
	pub fn get_first(&self) -> Option<&i16> {
		for calls in self.calls.iter() {
			if calls.layer == self.layer {
				return Some(&calls.id);
			}
		}
		None
	}
	pub fn get_node_mut(&mut self, id: i16) -> Option<&mut Node> {
		self.nodes.iter_mut().find(|node| node.id == id)
	}
	pub fn get_node(&self, id: i16) -> Option<&Node> {
		self.nodes.iter().find(|node| node.id == id)
	}
	pub fn get_depth(&self) -> i16 {
		self.calls.len() as i16
	}
	pub fn get_node_len(&self) -> i16 {
		self.nodes.len() as i16
	}
	pub fn local_exists(&self, local: &str) -> bool {
		for node in &self.nodes {
			if node.local == local {
				return true;
			}
		}
		false
	}
	pub fn clear_calls_layer(&mut self) {
		self.calls.retain(|v| v.layer != self.layer);
	}
	pub fn clear_calls(&mut self) {
		self.calls.clear();
	}
	pub fn increase_layer(&mut self) {
		self.layer += 1;
	}
	pub fn decrease_layer(&mut self) {
		self.layer -= 1;
	}
	pub fn increase_layer_args(&mut self) {
		self.layer_args += 1;
	}
	pub fn decrease_layer_args(&mut self) {
		self.layer_args -= 1;
	}
} 