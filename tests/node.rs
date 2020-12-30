mod common;
use pea2pea::{connect_nodes, Node, NodeConfig, Topology};

#[tokio::test]
async fn node_creation_any_port_works() {
    let _node = Node::new(None).await.unwrap();
}

#[should_panic]
#[tokio::test]
async fn node_creation_bad_params() {
    let mut config = NodeConfig::default();
    config.allow_random_port = false;
    let _node = Node::new(Some(config)).await.unwrap();
}

#[tokio::test]
async fn node_creation_used_port_fails() {
    let mut config = NodeConfig::default();
    config.desired_listening_port = Some(9); // the official Discard Protocol port
    config.allow_random_port = false;
    assert!(Node::new(Some(config)).await.is_err());
}

#[tokio::test]
async fn node_connect_and_disconnect() {
    let nodes = common::start_inert_nodes(2, None).await;
    connect_nodes(&nodes, Topology::Line).await.unwrap();

    assert!(nodes[0].disconnect(nodes[1].listening_addr));
    assert!(!nodes[0].is_connected(nodes[1].listening_addr));
}
