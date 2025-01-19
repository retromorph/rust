#![forbid(unsafe_code)]

use std::cmp;

#[derive(Default, Debug)]
pub struct QueueElement {
    value: i32,
    min: i32,
    max: i32,
}

#[derive(Default, Debug)]
pub struct MinMaxQueue {
    inbox: Vec<QueueElement>,
    outbox: Vec<QueueElement>,
}

impl MinMaxQueue {
    pub fn new() -> Self {
        Self {
            inbox: Vec::new(),
            outbox: Vec::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        let mut max = value;
        let mut min = value;
        if !self.inbox.is_empty() {
            max = cmp::max(max, self.inbox.last().unwrap().max);
            min = cmp::min(min, self.inbox.last().unwrap().min);
        }

        if !self.outbox.is_empty() {
            max = cmp::max(max, self.outbox.last().unwrap().max);
            min = cmp::min(min, self.outbox.last().unwrap().min);
        }

        self.inbox.push(QueueElement { value, min, max });
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        if self.outbox.is_empty() {
            while let Some(element) = self.inbox.pop() {
                if self.outbox.is_empty() {
                    self.outbox.push(QueueElement {
                        value: element.value,
                        min: element.value,
                        max: element.value,
                    });
                    continue;
                }

                let last = self.outbox.last().unwrap();
                self.outbox.push(QueueElement {
                    value: element.value,
                    min: cmp::min(element.value, last.min),
                    max: cmp::max(element.value, last.max),
                });
            }
        }

        Some(self.outbox.pop()?.value)
    }

    pub fn first(&self) -> Option<i32> {
        if !self.outbox.is_empty() {
            Some(self.outbox.last().unwrap().value)
        } else if !self.inbox.is_empty() {
            Some(self.inbox.first().unwrap().value)
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<i32> {
        if !self.inbox.is_empty() {
            Some(self.inbox.last().unwrap().value)
        } else if !self.outbox.is_empty() {
            Some(self.outbox.first().unwrap().value)
        } else {
            None
        }
    }

    pub fn min(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else if !self.inbox.is_empty() {
            Some(self.inbox.last().unwrap().min)
        } else if !self.outbox.is_empty() {
            Some(self.outbox.last().unwrap().min)
        } else {
            Some(cmp::min(
                self.inbox.last().unwrap().min,
                self.outbox.last().unwrap().min,
            ))
        }
    }

    pub fn max(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else if !self.inbox.is_empty() {
            Some(self.inbox.last().unwrap().max)
        } else if !self.outbox.is_empty() {
            Some(self.outbox.last().unwrap().max)
        } else {
            Some(cmp::max(
                self.inbox.last().unwrap().max,
                self.outbox.last().unwrap().max,
            ))
        }
    }

    pub fn len(&self) -> usize {
        self.inbox.len() + self.outbox.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }
}
