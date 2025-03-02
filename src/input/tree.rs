#[derive(Debug)]
pub struct BTree {
    root: Option<Node>,
}

impl BTree {
    pub fn new(root: Option<Node>) -> BTree {
        BTree{ root }
    }
    pub fn get_root(&mut self) -> &mut Option<Node> {
        &mut self.root
    }
    pub fn get_all_leafs(&mut self) -> Vec<String> {
        let mut leafs = Vec::new();
        if let Some(ref mut root) = self.root {
            root.collect_leafs(&mut leafs);
        } else {
            return Vec::new();
        }
        leafs 
    }
}

#[derive(Debug)]
pub struct Node {
    keys: Vec<String>,
    children: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            keys: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn add_key(&mut self, key: &str) {
        self.keys.push(key.to_string());
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
    pub fn to_string(&self) -> String {
        self.keys.clone()[0].clone()
    }
    pub fn collect_leafs(&self, leafs: &mut Vec<String>) {
        leafs.extend(self.keys.clone());
        for child in &self.children {
            child.collect_leafs(leafs);
        }
    }
}