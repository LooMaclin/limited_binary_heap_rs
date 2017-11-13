

use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering};

#[derive(Debug)]
pub struct LimitedBinaryHeap<T>
    where
        T: Ord,
{
    binary_heap: BinaryHeap<T>,
    limit: usize,
    filled: bool,
}

impl<T> LimitedBinaryHeap<T>
    where
        T: Ord,
{
    pub fn new(limit: usize) -> LimitedBinaryHeap<T> {
        LimitedBinaryHeap {
            binary_heap: BinaryHeap::with_capacity(limit as usize),
            limit,
            filled: false,
        }
    }


    pub fn insert(&mut self, value: T) {
        if self.filled {
            if value.cmp(self.binary_heap.peek().expect("Binary heap empty")) == Ordering::Less {
                self.binary_heap.pop();
                self.binary_heap.push(value);
            }
        } else {
            if self.binary_heap.len() + 1 == self.limit {
                self.filled = true;
            }
            self.binary_heap.push(value);
        }
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        self.binary_heap.into_sorted_vec()
    }
}

#[cfg(test)]
pub mod limited_binary_tests {

    use super::LimitedBinaryHeap;
    use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};

    #[derive(Debug)]
    pub struct RawResponseRecord {
        pub price: u32,
        pub product_name: String,
    }

    impl Ord for RawResponseRecord {
        fn cmp(&self, other: &Self) -> Ordering {
            self.price.cmp(&other.price)
        }
    }

    impl Eq for RawResponseRecord {}

    impl PartialOrd for RawResponseRecord {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.price.partial_cmp(&other.price)
        }
    }

    impl PartialEq for RawResponseRecord {
        fn eq(&self, other: &Self) -> bool {
            self.price == other.price
        }
    }


    #[test]
    fn test_limit() {
        let mut limited_binary_heap: LimitedBinaryHeap<RawResponseRecord> =
            LimitedBinaryHeap::new(3);
        limited_binary_heap.insert(RawResponseRecord {
            price: 10,
            product_name: "Seal".to_owned(),
        });
        limited_binary_heap.insert(RawResponseRecord {
            price: 20,
            product_name: "Sugar".to_owned(),
        });
        limited_binary_heap.insert(RawResponseRecord {
            price: 30,
            product_name: "Pepper".to_owned(),
        });
        limited_binary_heap.insert(RawResponseRecord {
            price: 5,
            product_name: "Potato".to_owned(),
        });
        limited_binary_heap.insert(RawResponseRecord {
            price: 40,
            product_name: "Milk".to_owned(),
        });
        limited_binary_heap.insert(RawResponseRecord {
            price: 15,
            product_name: "Nut".to_owned(),
        });
        assert_eq!(
            limited_binary_heap.into_sorted_vec(),
            vec![
                RawResponseRecord {
                    price: 5,
                    product_name: "Potato".to_owned(),
                },
                RawResponseRecord {
                    price: 10,
                    product_name: "Seal".to_owned()
                },
                RawResponseRecord {
                    price: 15,
                    product_name: "Nut".to_owned(),
                },
            ]
        );
    }
}
