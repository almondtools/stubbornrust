use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub struct TransferItem {
  data:u64
}

pub struct Iter<'s> {
  timestamp:u64,
  from:&'s mut VecDeque<TransferItem>,
  to:&'s mut Vec<TransferItem>
}

impl <'s> Iterator for Iter<'s> {
  type Item=TransferItem;

  fn next(&mut self) -> Option<TransferItem> {
    let front = self.from.pop_front();
    if let Some(item) = front {
       if item.data <= self.timestamp {
         self.to.push(item);
         front
       } else {
         self.from.push_front(item);
         None
       }
    } else {
        None
    }
  }
}

pub struct Transfer {
  from:VecDeque<TransferItem>,
  to:Vec<TransferItem>
}

impl Transfer {
  pub fn start(&mut self, timestamp:u64) -> Iter {
    Iter {
      timestamp,
      from:&mut self.from,
      to:&mut self.to,
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
