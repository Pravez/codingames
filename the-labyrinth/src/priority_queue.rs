use std::collections::{HashMap, VecDeque};

pub trait PriorityQueueTrait<Element> {
    fn new() -> Self;
    fn insert(&mut self, element: Element, p: u8);
    fn peek(&self) -> Option<&Element>;
    fn pop(&mut self) -> Option<Element>;
    fn is_empty(&self) -> bool;
}

pub struct PriorityQueue<T>(HashMap<u8, VecDeque<T>>, u8);

impl<T: Clone> PriorityQueueTrait<T> for PriorityQueue<T> {
    fn new() -> Self {
        Self(HashMap::new(), 0)
    }

    fn insert(&mut self, element: T, p: u8) {
        self.0.entry(p).and_modify(|e| e.push_back(element.clone())).or_insert(VecDeque::from(vec![element.clone()]));
        if p > self.1 {
            self.1 = p
        }
    }

    fn peek(&self) -> Option<&T> {
        self.0.get(&self.1).map_or(None, |v|v.front())
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.0.get_mut(&self.1).map(|v| (v.pop_front()).unwrap());
        if self.0.contains_key(&self.1) {
            if self.0.get(&self.1).unwrap().is_empty() {
                self.0.remove(&self.1);
                self.1 = *self.0.keys().max().unwrap_or(&0);
            }
        }
        val
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
