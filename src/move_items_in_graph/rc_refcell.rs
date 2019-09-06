use std::rc::Rc;
use core::cell::RefCell;



#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TransferItem {
  data:u64
}

impl From<u64> for TransferItem {
  fn from(data:u64) -> TransferItem {
    TransferItem {
      data
    }
  }
}

pub type NodeRef = Rc<RefCell<Node>>;

pub struct Node {
  successors:Vec<NodeRef>,
  items:Vec<TransferItem>
}

impl Node {
  pub fn trigger(&mut self) {
    while self.items.len() > 0 { 
      let item = self.items.remove(0);
      for successor in self.successors.iter() {
        successor.borrow_mut().items.push(item);
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_acyclic() {
    let node_ab = Node {
      successors:vec![],
      items:vec![]
    };
    let node_ab_ref = Rc::from(RefCell::from(node_ab));
    let node_bc = Node {
      successors:vec![],
      items:vec![]
    };
    let node_bc_ref = Rc::from(RefCell::from(node_bc));
    let mut node_a =  Node {
      successors:vec![node_ab_ref.clone()],
      items:vec![1u64.into(),2u64.into()]
    };
    let mut node_b =  Node {
      successors:vec![node_ab_ref.clone(), node_bc_ref.clone()],
      items:vec![3u64.into()]
    };
    let mut node_c =  Node {
      successors:vec![node_bc_ref.clone()],
      items:vec![4u64.into()]
    };

    node_a.trigger();
    node_b.trigger();
    node_c.trigger();

    assert!(node_a.items.is_empty());
    assert!(node_b.items.is_empty());
    assert!(node_c.items.is_empty());

    assert_eq!(node_ab_ref.borrow().items, vec![1u64.into(), 2u64.into(), 3u64.into()]);
    assert_eq!(node_bc_ref.borrow().items, vec![3u64.into(), 4u64.into()]);
  }

  #[test]
  fn test_cyclic() {
    let node_ab = Node {
      successors:vec![],
      items:vec![]
    };
    let node_ab_ref = Rc::from(RefCell::from(node_ab));
    let node_bc = Node {
      successors:vec![],
      items:vec![]
    };
    let node_bc_ref = Rc::from(RefCell::from(node_bc));
    let node_a =  Node {
      successors:vec![node_ab_ref.clone()],
      items:vec![1u64.into(),2u64.into()]
    };
    let node_a_ref = Rc::from(RefCell::from(node_a));
    let node_b =  Node {
      successors:vec![node_ab_ref.clone(), node_bc_ref.clone()],
      items:vec![3u64.into()]
    };
    let node_b_ref = Rc::from(RefCell::from(node_b));
    let node_c =  Node {
      successors:vec![node_bc_ref.clone()],
      items:vec![4u64.into()]
    };
    let node_c_ref = Rc::from(RefCell::from(node_c));

    node_bc_ref.borrow_mut().successors.push(node_b_ref.clone());
    node_ab_ref.borrow_mut().successors.push(node_b_ref.clone());

    let nodes = [node_a_ref.clone(), node_b_ref.clone(), node_c_ref.clone(), node_ab_ref.clone(), node_bc_ref.clone()];
    for _ in 0..1 {
      for node in nodes.iter() {
        node.borrow_mut().trigger();
      }
    }
    assert!(node_b_ref.borrow().items.contains(&1u64.into()));
    assert!(node_b_ref.borrow().items.contains(&2u64.into()));
    assert!(node_b_ref.borrow().items.contains(&3u64.into()));
    assert!(node_b_ref.borrow().items.contains(&4u64.into()));
  }
}
