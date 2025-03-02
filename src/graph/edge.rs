use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct Edge {
	pub from: i16,
	pub to: i16,
}

impl Edge {
	pub fn new(from: i16, to: i16) -> Edge {
		Edge {from, to}
	}
}

impl Debug for Edge {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{:?} -> {:?}", self.from, self.to)
	}
}