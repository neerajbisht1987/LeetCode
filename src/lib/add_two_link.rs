
use std::rc::Rc;
use std::cell::RefCell;
use link_list::Linklist;
use link_list::Linklist::{Cons,Nil};
/*
pub fn add_two_link(list1:&Linklist, list2:&Linklist)
{
    let mut _l1 = list1;
    let mut _l2 = list2;

    while *_l1 != Linklist::Nil || *_l2 != Linklist::Nil
    {
        let mut _val1=0;
        let mut _val2=0;
        let mut linklst =Nil;
        match  _l1
        {
            &Cons(ref val,ref link) =>
            {
                _val1 = **val;
                let temp=link.borrow();
                 _l1=(*temp).as_ref();
            },
            &Nil=>_l1= &Nil,           
        }
        match _l2
        {
            &Cons(ref val,ref link) =>
            {
                _val2 = **val;
                 _l2=link.borrow().as_ref();
            },
            &Nil=>_l2= &Nil,           
        }
        let _val3= _val1 + _val2;
        let mut list = Cons(Rc::new(_val3),RefCell::new(Rc::new(Nil)));        
        if linklst == Nil
        {
            linklst = list;
        }
        else
        {           
            put_node_at_last(&mut linklst,&mut list);            
        }         
    }

    fn put_node_at_last(list:&mut Linklist,mut node:& mut Linklist)
    {
        match list
        {
            & mut Cons(_,ref link) =>
            {
                if **link.borrow() == Nil
                {
                    *link.borrow_mut()=Rc::new(*node);
                }
                else
                {
                    put_node_at_last(&mut link.borrow_mut(),& mut node);                    
                }               
            },
            & mut Nil=> return,
        }
    }


}*/