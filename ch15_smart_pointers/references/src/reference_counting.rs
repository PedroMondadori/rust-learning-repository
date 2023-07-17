use std::rc::Rc;

pub enum RCList {
    RCCons(i32, Rc<RCList>),
    RCNil,
}
