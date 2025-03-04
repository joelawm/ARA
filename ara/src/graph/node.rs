/*-------------
/node.rs

This file is for the node operations in the graph.
-------------*/
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct Node {
	pub id: i16,
	pub key: String,
	pub local: String,
	pub literal: Option<String>,
	pub doc: Vec<String>,
	pub node_type: NodeType,
}

impl Node {
	pub fn new(key: &str, node_type: NodeType) -> Node {
		Node {id: 0, local: String::new(), literal: None, doc: Vec::new(), key: key.to_string(), node_type}
	}
	pub fn update_id(&mut self, id: i16) {
		self.id = id;
	}
	pub fn add_local(&mut self, local: &str) {
		self.local = local.to_string();
	}
	pub fn add_literal(&mut self, literal: &str) {
		self.literal = Some(literal.to_string());
	}
	pub fn add_comments(&mut self, doc: Vec<String>) -> &mut Node {
		self.doc = doc;
		self
	}
}

impl Debug for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Id: {} Key: {} Local: {} Literal: {:?} Type: {:?}", self.id, self.key, self.local, self.literal, self.node_type)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
	Function,
	Method,
	Call,
	Macro,
	Local,
	Struct,
	Paren,
	Tuple,
}