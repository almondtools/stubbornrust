use std::rc::Rc;
use std::rc::Weak;
use core::cell::RefCell;


pub type NodeRef = Rc<RefCell<Node>>;
pub type WeakNodeRef = Weak<RefCell<Node>>;

pub trait Connected<T> {
  fn add_edge(&mut self, node:&T);
}

#[derive(Debug)]
pub struct Node {
  successors:Vec<WeakNodeRef>,
  data:String
}

impl Connected<NodeRef> for Node {
  fn add_edge(&mut self, node:&NodeRef) {
    self.successors.push(Rc::downgrade(&node));
  }
}

impl Connected<NodeRef> for NodeRef {
  fn add_edge(&mut self, node:&NodeRef) {
    self.borrow_mut().add_edge(node);
  }
}

pub struct Graph {
  nodes:Vec<NodeRef>,
}

impl Graph {

  pub fn new() -> Graph {
    Graph{
      nodes:Vec::new()
    }
  }

  pub fn node<T>(&mut self, data:T) -> NodeRef where T:Into<String> {
    let node = Node {
      successors:Vec::new(),
      data:data.into()
    };
    let node_ref = Rc::from(RefCell::from(node));
    self.nodes.push(node_ref.clone());
    node_ref
  }

}

#[cfg(test)]
mod tests {

  use super::*;

  static mut DROPPED: Option<Vec<String>> = None;

  fn add_dropped(data:String) {
    unsafe {
      let dropped = DROPPED.get_or_insert_with(|| Vec::new());
      dropped.push(data);
    }
  }

  fn get_dropped() -> Vec<String> {
    unsafe {
      let dropped = DROPPED.get_or_insert_with(|| Vec::new());
      dropped.clone()
    }
  }

  impl Drop for Node {
    fn drop(&mut self) {
      add_dropped(self.data.clone());
    }
  }

  #[test]
  fn test_drop() {
    {
      let mut graph = Graph::new();

      let mut node_ab = graph.node("ab");
      let mut node_bc = graph.node("bc");
      let mut node_a = graph.node("a");
      let mut node_b = graph.node("b");
      let mut node_c = graph.node("c");

      node_a.add_edge(&node_ab);
      node_b.add_edge(&node_ab);
      node_b.add_edge(&node_bc);
      node_c.add_edge(&node_bc);
      node_ab.add_edge(&node_c);
      node_bc.add_edge(&node_a);
    }

    let dropped = get_dropped();
    assert_eq!(dropped.len(), 5);
    assert!(dropped.contains(&"a".to_string()));
    assert!(dropped.contains(&"b".to_string()));
    assert!(dropped.contains(&"c".to_string()));
    assert!(dropped.contains(&"ab".to_string()));
    assert!(dropped.contains(&"bc".to_string()));
  }
}
