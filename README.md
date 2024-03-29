Stubborn Rust
=============

In this repository you can find some problems I had with development in Rust together with the solutions I found:

## Transfer Items

We start with an object that is responsible for transferring data items from one data source to a target.

```rust
pub struct Transfer {
  from:FromType, // provides items
  to:ToType      // accepts items
}
```

Each data item provides a timestamp, e.g.

```rust
struct TransferItem {
  timestamp:u64
}
```

The transfer source `from` can be assumed to provide transfer items in timestamp order (from low to high).

### Task

Write a method `start(to:u64) -> TransferIter` returning an iterator (`TransferIter`) with following properties:
* it returns each data item with timestamp lesser or equal to `to`
* it removes each returned item from the datasource
* it add each returned item to the target

### Solutions

* [Passing mutable references to data source and target to the iterator](https://github.com/almondtools/stubbornrust/blob/master/src/transfer_items/referencing_collections.rs)
* [Passing trait adaptors of data source and target to the iterator](https://github.com/almondtools/stubbornrust/blob/master/src/transfer_items/referencing_traits.rs)

## Move Items through Graph

Consider a directed graph with nodes containing items, e.g.

```rust
pub struct Node {
  items:CollectionOfItems,    // contains items
  successors:ListOfNodes      // contains reachable nodes
}
```

The items may be assumed to be copyable (implement the Copy trait).

Note: A Node might reach multiple nodes and multiple nodes might reach a single node

### Task

Write a method `trigger(&mut self)` which performs the following tasks:

* it moves all items from it self to all successors
* it works in cyclic and acyclic graphs

### Solutions

* [static variant using node references of type Rc<RefCell<Node>>](https://github.com/almondtools/stubbornrust/blob/master/src/move_items_in_graph/rc_refcell.rs)
* [thread safe variant using node references of type Arc<Mutex<Node>>](https://github.com/almondtools/stubbornrust/blob/master/src/move_items_in_graph/arc_mutex.rs)

## Non-Leaking Mutable Graphs

Consider a directed strongly connected graph, i.e. there is a path from every node to each other (this implies that there is a cycle). 

### Task

Create a graph of strongly connected nodes and then drop it. Ensure that each graph node is dropped after the graph is dropped. 

### Solutions

* [a graph serving as factory for node references, where nodes only contain weak references to prevent memory leaks](https://github.com/almondtools/stubbornrust/blob/master/src/mutable_graphs/node_factory.rs)
