use std::collections::HashMap;

#[derive(Debug)]
pub struct Stack {
	pub root: String,
	pub stack: HashMap<i16, String>,
	pub group_layer: i16,
}

impl Stack {
	/// Create a new Use Stack which allows use to keep the current stack of use statements
	pub fn new() -> Stack {
		Stack {
			root: String::new(),
			stack: HashMap::new(),
			group_layer: 1,
		}
	}
	/// Get the current stack with the last element that would be added
	pub fn get_with_last(&self, last_element: &str) -> String {
		let mut data = self.root.clone();
		for stack in &self.stack {
			data.push_str("::");
			data.push_str(stack.1);
		}
		if data.is_empty() {
			data.push_str(last_element);
		} else {
			data.push_str(&format!("::{}", last_element));
		}
		data
	}
	/// Insert a new element into the stack
	pub fn insert(&mut self, value: String) {
		if self.root.is_empty() {
			self.root = value;
			return; 
		} else {
			let search_layer = self.group_layer;
			if self.stack.contains_key(&search_layer) {
				let el = self.stack.get(&search_layer).unwrap();
				self.stack.insert(search_layer, format!("{}::{}", el, value));
			} else {
				self.stack.insert(search_layer, value);
			}
		}
	}
	/// Adds a group layer
	pub fn add_layer(&mut self) {
		self.group_layer += 1;
	}
	/// Clear the last group layer and element of stack
	pub fn clear_layer(&mut self) {
		self.stack.remove(&self.group_layer);
		self.group_layer -= 1;
	}
	///  Remove the last element of the stack
	pub fn pop_stack(&mut self) {
		self.stack.remove(&(&self.group_layer));
	}
	/// Clear the Use Stack
	pub fn clear(&mut self) {
		self.root.clear();
		self.stack.clear();
		self.group_layer = 0;
	}
}