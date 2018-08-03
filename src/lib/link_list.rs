use std::rc::Rc;
 use std::cell::RefCell;

#[derive(Debug,PartialEq)]
pub enum Linklist {
    Cons(Rc<i32>, RefCell<Rc<Linklist>>),
    Nil,
}
