#![warn(clippy::pedantic)]
use crate::ast::node_type::NodeType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NodesStorage {
    node_routes: Vec<NodeRoute>,
    pub nodes: Vec<NodeType>,
    file_content_map: HashMap<u128, String>,
}

impl NodesStorage {
    // pub fn find_node(&self, id: u128) -> Option<NodeType> {
    //     self.nodes.iter().find(|n| n.id() == id).cloned()
    // }

    // pub fn find_node_file(&self, id: u128) -> Option<Rc<Program>> {
    //     if self.file_content_map.contains_key(&id) {
    //         let file = self.find_node(id).unwrap();
    //         match file {
    //             NodeType::Program(f) => Some(f),
    //             _ => None,
    //         }
    //     } else {
    //         let mut node_id = id;
    //         while let Some(parent) = self.find_parent_node(node_id) {
    //             if parent.parent.unwrap() == 0 {
    //                 let file = self.find_node(parent.id).unwrap();
    //                 match file {
    //                     NodeType::Program(f) => return Some(f),
    //                     _ => return None,
    //                 }
    //             }
    //             node_id = parent.id;
    //         }
    //         None
    //     }
    // }

    // #[must_use = "Use this method to find a Node's parent Node"]
    // pub fn find_parent_node(&self, id: u128) -> Option<NodeRoute> {
    //     self.node_routes.iter().find(|n| n.id == id).cloned()
    // }

    // //TODO test this function and remove source_code attr from nodes
    // #[must_use = "Use this method to get a Node's source code"]
    // /// # Panics
    // ///
    // /// This function will panic if the node with the given id is not found.
    // pub fn get_node_source_code(&self, id: u128) -> Option<String> {
    //     todo!("Implement this function")
    // }

    pub fn add_node(&mut self, item: NodeType, parent: u128) {
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

    fn add_storage_node(&mut self, node: NodeRoute, parent: u128) {
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
    id: u128,
    parent: Option<u128>,
    children: Vec<u128>,
}
