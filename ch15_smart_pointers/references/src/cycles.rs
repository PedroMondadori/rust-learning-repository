use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use CList::{CCons, CNil};

#[derive(Debug)]
pub enum CList {
    CCons(i32, RefCell<Rc<CList>>),
    CNil,
}

impl CList {
    pub fn tail(&self) -> Option<&RefCell<Rc<CList>>> {
        match self {
            CCons(_, item) => Some(item),
            CNil => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}
