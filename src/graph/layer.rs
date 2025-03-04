#[derive(Debug)]
pub struct Layer {
	pub id: i16,
	pub layer: i16,
	pub args: i16,
}

impl Layer {
	pub fn new(id: i16, layer: i16, args: i16) -> Layer {
		Layer {id, layer, args}
	}
}