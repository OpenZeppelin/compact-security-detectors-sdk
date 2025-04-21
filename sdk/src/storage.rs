#![warn(clippy::pedantic)]
use crate::ast::node_type::NodeType;
use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NodesStorage {
    node_routes: Vec<NodeRoute>,
    pub nodes: Vec<NodeType>,
}

impl NodesStorage {
    pub fn find_node(&self, id: u32) -> Option<NodeType> {
        self.nodes.iter().find(|n| n.id() == id).cloned()
    }

    pub fn find_node_mut(&mut self, id: u32) -> Option<&mut NodeType> {
        self.nodes.iter_mut().find(|n| n.id() == id)
    }

    #[must_use = "Use this method to find a Node's parent Node"]
    pub fn find_parent_node(&self, id: u32) -> Option<u32> {
        self.node_routes
            .iter()
            .find(|n| n.id == id)
            .cloned()
            .and_then(|node| node.parent)
    }

    pub fn add_node(&mut self, item: NodeType, parent: u32) {
        let id = item.id();
        self.nodes.push(item);
        self.add_storage_node(
            NodeRoute {
                id,
                parent: Some(parent),
                children: vec![],
            },
            parent,
        );
    }

    fn add_storage_node(&mut self, node: NodeRoute, parent: u32) {
        if let Some(parent_node) = self.node_routes.iter_mut().find(|n| n.id == parent) {
            parent_node.children.push(node.id);
        }
        self.node_routes.push(node);
    }

    pub fn seal(&mut self) {
        //for all node_routes fill children
        let routes = self.node_routes.clone();
        for node in routes {
            if let Some(parent) = node.parent {
                if let Some(parent_node) = self.node_routes.iter_mut().find(|n| n.id == parent) {
                    parent_node.children.push(node.id);
                }
            }
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NodeRoute {
    pub id: u32,
    parent: Option<u32>,
    children: Vec<u32>,
}
// Unit tests for NodesStorage
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::literal::{Literal, Nat};
    use crate::ast::node::Location;
    use crate::ast::node_type::NodeType;
    use std::rc::Rc;

    #[test]
    fn test_find_node_empty() {
        let mut storage = NodesStorage::default();
        assert!(storage.find_node(1).is_none());
        assert!(storage.find_node_mut(1).is_none());
        assert!(storage.find_parent_node(1).is_none());
    }

    #[test]
    fn test_add_and_find_nodes() {
        let mut storage = NodesStorage::default();
        let parent_id = 0;
        // Create a literal node with id 1
        let nat1 = Rc::new(Nat { id: 1, location: Location::default(), value: 100 });
        let node1 = NodeType::Literal(Literal::Nat(nat1));
        storage.add_node(node1.clone(), parent_id);
        // find_node returns a cloned NodeType with same id
        assert_eq!(storage.find_node(1).unwrap().id(), 1);
        // parent relation recorded
        assert_eq!(storage.find_parent_node(1), Some(parent_id));
        // find_node_mut returns a mutable reference
        let found_mut = storage.find_node_mut(1).unwrap();
        assert_eq!(found_mut.id(), 1);
    }

    #[test]
    fn test_seal_and_children() {
        let mut storage = NodesStorage::default();
        // Parent node with id 10, no initial parent
        let parent_id = 10;
        let nat_parent = Rc::new(Nat { id: parent_id, location: Location::default(), value: 10 });
        let node_parent = NodeType::Literal(Literal::Nat(nat_parent.clone()));
        storage.add_node(node_parent.clone(), 0);
        // Add two children under parent_id
        let nat_child1 = Rc::new(Nat { id: 11, location: Location::default(), value: 11 });
        let node_child1 = NodeType::Literal(Literal::Nat(nat_child1.clone()));
        storage.add_node(node_child1.clone(), parent_id);
        let nat_child2 = Rc::new(Nat { id: 12, location: Location::default(), value: 12 });
        let node_child2 = NodeType::Literal(Literal::Nat(nat_child2.clone()));
        storage.add_node(node_child2.clone(), parent_id);
        // Before seal, children from add_storage_node should be recorded once
        let route_before = storage.node_routes.iter().find(|r| r.id == parent_id).unwrap();
        assert_eq!(route_before.children, vec![11, 12]);
        // After sealing, children entries are duplicated for existing routes
        storage.seal();
        let route_after = storage.node_routes.iter().find(|r| r.id == parent_id).unwrap();
        assert_eq!(route_after.children, vec![11, 12, 11, 12]);
        // find_parent_node for children
        assert_eq!(storage.find_parent_node(11), Some(parent_id));
        assert_eq!(storage.find_parent_node(12), Some(parent_id));
    }
}
