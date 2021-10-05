use std::ptr;

use indextree::{Arena, NodeId};

use crate::node::Node;

pub struct Router<'a> {
    arena: Arena<&'a Box<dyn Node>>
}

impl Router<'a> {
    pub fn connect<'a>(&mut self, from: &Box<dyn Node>, to: &Box<dyn Node>) {
        let from_possibly = self.search_node_id(from);
        let to_possibly = self.search_node_id(to);

        let from_id:Option::<NodeId, 'a> = match from_possibly {
            Some(from) => from,
            None => {
                self.arena.new_node(&'a from)
            }
        };
    }

    fn search_node_id(&self, node: &Box<dyn Node>) -> Option<NodeId> {
        let mut result = None;

        for node_to_check in self.arena.iter() {
            if ptr::eq(node_to_check.get(), node) && !node_to_check.is_removed() {
                result = Some(node_to_check);
            }
        }

        match result {
            Some(node) => self.arena.get_node_id(node),
            None => None
        }
    }
}
