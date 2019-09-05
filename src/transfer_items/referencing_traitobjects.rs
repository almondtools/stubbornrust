use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub struct TransferItem {
  data:u64
}

pub trait Source {
  fn read(&mut self) -> Option<TransferItem>;
}

pub trait Sink {
  fn write(&mut self, item:TransferItem);
}

pub struct Iter<F, T> {
  from:F,
  to:T
}

impl <F,T> Iterator for Iter<F, T> where F:Source, T:Sink{
  type Item=TransferItem;

  fn next(&mut self) -> Option<TransferItem> {
    let next = self.from.read();
    if let Some(item) = next {
       self.to.write(item);
       next
    } else {
        None
    }
  }
}

pub struct TransferSource<'s> {
  timestamp:u64,
  source:&'s mut VecDeque<TransferItem>,
}

impl <'s> Source for TransferSource<'s> {
  fn read(&mut self) -> Option<TransferItem> {
    let next = self.source.pop_front();
    if let Some(item) = next {
       if item.data <= self.timestamp {
         return next;
       } else {
         self.source.push_front(item);
       }
    }
    None
  }
}

pub struct TransferSink<'s> {
  sink:&'s mut Vec<TransferItem>
}

impl <'s> Sink for TransferSink<'s> {
  fn write(&mut self, item:TransferItem) {
    self.sink.push(item);
  }
}

pub struct Transfer {
  from:VecDeque<TransferItem>,
  to:Vec<TransferItem>
}

impl Transfer {
  pub fn start(&mut self, timestamp:u64) -> Iter<TransferSource, TransferSink> {
    Iter {
      from:TransferSource {source:&mut self.from, timestamp},
      to:TransferSink {sink:&mut self.to},
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_transfer() {
    let to = Vec::new();
    let from = [1u64,2u64,3u64,4u64].iter().map(|&d| TransferItem {
      data:d
    }).collect::<VecDeque<_>>();
    let mut transfer = Transfer {
      from,
      to
    };
    let mut iterator = transfer.start(2u64);

    let e1 = iterator.next().unwrap();
    assert_eq!(e1.data, 1u64);

    let e2 = iterator.next().unwrap();
    assert_eq!(e2.data, 2u64);

    let e3 = iterator.next();
    assert!(e3.is_none());

    assert_eq!(transfer.to.get(0).unwrap().data,1u64);
    assert_eq!(transfer.to.get(1).unwrap().data,2u64);
  }
}
