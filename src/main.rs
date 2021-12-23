use std::{rc::Rc, mem, ops::Deref};

struct Frame<T> {
    value: T,
    previous: Option<Rc<Frame<T>>>
}

impl<T> Deref for Frame<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Clone)]
struct MultiStack<T> {
    head: Option<Rc<Frame<T>>>
}

impl<T: Clone> MultiStack<T> {
    fn new() -> Self {
        Self {head: None}
    }

    fn push(&mut self, elem: T) {
        self.head = Some(Rc::new(Frame {value: elem, previous: mem::take(&mut self.head)}));
    }

    fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let previous_node = self.head.as_ref().unwrap().clone();
            if let Some(previous) = &previous_node.previous {
                return Some(mem::replace(&mut self.head, Some(previous.clone())).unwrap().value.clone());
            } else {
                return Some(mem::replace(&mut self.head, None).unwrap().value.clone());
            };
        }
        None
    }
}

fn main() {
    let mut stack: MultiStack<i8> = MultiStack::new();

    stack.push(1);
    stack.push(2);
    let mut stack2 = stack.clone();
    stack.push(3);
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    stack.push(1);
    println!("{:?}", stack.pop());
    println!();

    stack2.push(5);
    println!("{:?}", stack2.pop());
    println!("{:?}", stack2.pop());
    println!("{:?}", stack2.pop());
    println!("{:?}", stack2.pop());
}
