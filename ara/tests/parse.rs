use ara::Ara;
use ara::graph::node::NodeType;

fn setup_ara() -> Ara {
	let mut ara = Ara::new();
	ara.path("../examples/axum");
	ara
}

#[cfg(test)]
mod test_parser {
	use ara::graph::{edge::Edge, node::Node};
	use super::*;

	/// Test that we can parse the examples directory
	#[test]
	fn parsing_example_graph() {
		let mut ara = setup_ara();
		let ara = ara.launch().unwrap();

		let nodes = vec![
			Node{ id: 0, key: "main::main".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Function },
			Node{ id: 1, key: "routes::create_routes".to_string(), local: "app".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 2, key: "format".to_string(), local: "addr".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Macro },
			Node{ id: 3, key: "parse".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 4, key: "expect".to_string(), local: "".to_string(), literal: Some("\"Unable to parse socket address\"".to_string()), doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 5, key: "tokio::net::TcpListener::bind".to_string(), local: "listener".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 6, key: "addr".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Local },
			Node{ id: 7, key: "expect".to_string(), local: "".to_string(), literal: Some("\"Failed to bind listener\"".to_string()), doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 8, key: "axum::serve".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 9, key: "listener".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Local },
			Node{ id: 10, key: "app".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Local },
			Node{ id: 11, key: "into_make_service_with_connect_info".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 12, key: "expect".to_string(), local: "".to_string(), literal: Some("\"Failed to run server.\"".to_string()), doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 13, key: "routes::create_routes".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Function },
			Node{ id: 14, key: "Router::new".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 15, key: "route".to_string(), local: "".to_string(), literal: Some("\"/api/health\"".to_string()), doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 16, key: "get".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 17, key: "get_health".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 18, key: "routes::get_health".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Function },
			Node{ id: 19, key: "(,)".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Tuple },
			Node{ id: 20, key: "StatusCode::OK".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 21, key: "format".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Macro },
			Node{ id: 22, key: "into_response".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method }
		];

		let edges = vec![
			Edge { from: 0, to: 1 },
			Edge { from: 0, to: 2 },
			Edge { from: 2, to: 3 },
			Edge { from: 3, to: 4 },
			Edge { from: 0, to: 5 },
			Edge { from: 5, to: 6 },
			Edge { from: 5, to: 7 },
			Edge { from: 0, to: 8 },
			Edge { from: 8, to: 9 },
			Edge { from: 8, to: 10 },
			Edge { from: 10, to: 11 },
			Edge { from: 8, to: 12 },
			Edge { from: 13, to: 14 },
			Edge { from: 14, to: 15 },
			Edge { from: 15, to: 16 },
			Edge { from: 16, to: 17 },
			Edge { from: 18, to: 19 },
			Edge { from: 19, to: 20 },
			Edge { from: 19, to: 21 },
			Edge { from: 19, to: 22 }
		];

		let ara_nodes: Vec<String> = ara.state.graph.nodes.iter().map(|node| format!("{:?}", node)).collect();
		let data_nodes: Vec<String> = nodes.iter().map(|node| format!("{:?}", node)).collect();
		let ara_edges: Vec<String> = ara.state.graph.edges.iter().map(|edge| format!("{:?}", edge)).collect();
		let data_edges: Vec<String> = edges.iter().map(|edge| format!("{:?}", edge)).collect();

		assert_eq!(ara_nodes, data_nodes);
		assert_eq!(ara_edges, data_edges);
	}
}