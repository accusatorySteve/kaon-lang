use crate::data::Data;

#[derive(Debug, Clone)]
pub struct Slot(Data);

impl Slot {
    pub fn new(data: Data) -> Slot {
        Slot(data)
    }
}

#[derive(Debug)]
pub struct Stack {
    stack: Vec<Slot>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: vec![] }
    }

    pub fn pop(&mut self) -> Data {
        self.stack.pop().expect("Stack should not be empty").0
    }

    pub fn push(&mut self, slot: Slot) {
        self.stack.push(slot);
    }

    pub fn peek(&mut self) -> Data {
        self.stack[self.stack.len() - 1].0.clone()
    }
}
