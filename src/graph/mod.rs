use std::fmt::Debug;
use node::NodeType;

use crate::graph::node::Node;
use crate::graph::edge::Edge;

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
	pub fn add_node(&mut self, mut node: Node) -> &Node {
		// Update the id of the node
		let id = self.get_node_len();
		node.update_id(id);

		self.calls.push(Layer::new(id, self.layer, self.layer_args));
		println!("Calls {:?}", self.calls);

		println!("Adding node: {:?}", node);
		self.nodes.push(node);
		self.nodes.last().unwrap()
	}
	pub fn add_edge(&mut self, to: i16) {
		let from = self.get_offset().unwrap();

		println!("Adding edge from: {} to: {}", from.id, to);

		self.edges.push(Edge::new(from.id, to));
	}
	pub fn add_literal(&mut self, literal: &str) {
		let layer_id = self.get_last().unwrap().id;
		if let Some(node) = self.get_node_mut(layer_id) {
			node.add_literal(literal);
		}
	}
	pub fn add_local(&mut self, local: &str) {
		let node_id = self.get_first().unwrap();
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
		println!("Clearing calls layer: {}", self.layer);
		// Remove where the hash map value is equal to the current layer
		self.calls.retain(|v| v.layer != self.layer);
	}
	pub fn clear_calls(&mut self) {
		println!("Clearing calls");
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