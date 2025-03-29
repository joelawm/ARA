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
			Node{ id: 22, key: "into_response".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 23, key: "routes::get_list".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Function },
			Node{ id: 24, key: "headers".to_string(), local: "user_agent".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 25, key: "get".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 26, key: "USER_AGENT".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 27, key: "context".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 28, key: "to_str".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 29, key: "unwrap_or_default".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 30, key: "to_string".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 31, key: "Err".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 32, key: "CreateOauthUser::MissingUserAgent".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 33, key: "into_response".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 34, key: "pool".to_string(), local: "conn".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 35, key: "acquire".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method },
			Node{ id: 36, key: "context".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 37, key: "Err".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 38, key: "Error::PoolConnection".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Call },
			Node{ id: 39, key: "into_response".to_string(), local: "".to_string(), literal: None, doc: Vec::new(), node_type: NodeType::Method }
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
			Edge { from: 19, to: 22 },
			Edge { from: 23, to: 24 },
			Edge { from: 24, to: 25 },
			Edge { from: 25, to: 26 },
			Edge { from: 24, to: 27 },
			Edge { from: 27, to: 28 },
			Edge { from: 28, to: 29 },
			Edge { from: 29, to: 30 },
			Edge { from: 24, to: 31 },
			Edge { from: 31, to: 32 },
			Edge { from: 32, to: 33 },
			Edge { from: 23, to: 34 },
			Edge { from: 34, to: 35 },
			Edge { from: 34, to: 36 },
			Edge { from: 34, to: 37 },
			Edge { from: 37, to: 38 },
			Edge { from: 38, to: 39 },
		];

		let ara_nodes: Vec<String> = ara.state.graph.nodes.iter().map(|node| format!("{:?}", node)).collect();
		let data_nodes: Vec<String> = nodes.iter().map(|node| format!("{:?}", node)).collect();
		let ara_edges: Vec<String> = ara.state.graph.edges.iter().map(|edge| format!("{:?}", edge)).collect();
		let data_edges: Vec<String> = edges.iter().map(|edge| format!("{:?}", edge)).collect();

		pretty_assertions::assert_eq!(ara_nodes, data_nodes);
		pretty_assertions::assert_eq!(ara_edges, data_edges);
	}
}