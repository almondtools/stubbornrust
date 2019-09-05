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