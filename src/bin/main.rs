extern crate leetCodeLib;
extern crate ndarray;
use std::rc::Rc;
use std::cell::RefCell;
use leetCodeLib::link_list::Linklist::{Cons,Nil};
use leetCodeLib::link_list::Linklist;
use leetCodeLib::dijkstra;

fn main() {
    let path = [
        [<u32>::max_value(),5,<u32>::max_value(),3,<u32>::max_value(),<u32>::max_value()],
        [<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value()],
        [<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value()],
        [<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value()],
        [<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value()],
        [<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value(),<u32>::max_value()],
        ];
    dijkstra::dijkstra_shortest_path(path,0);

    //leetCodeLib::print_hello_world();
    //let _arr=[1,2,3,4];
    // let vec = create_vector(&_arr);
    // let pair = leetCodeLib::two_sum::two_sum(&vec, 5);
    // println!("{:?}",pair );
    // let _arr1=[1,2,3];
    // let _list1 = create_linklist(&_arr1);
    // print_linklist(&_list1);  
    // let _arr2=[3,2,1];
    // let _list2 = create_linklist(&_arr2);
    // print_linklist(&_list2);  
     //let x_vec = RefCell::new(vec![5u32, 8u32, 2u32]);
    
    // borrow `x_vec` mutably in the while-head
    //while let Some(x) = {let mut b=x_vec.borrow_mut();b.pop()} {
    //while let Some(x) = x_vec.borrow_mut().pop() {
        // borrow it again in the body. 
        // ERROR: apparently the cell was not released
        // let rem_count = x_vec.borrow().len();
        // println!("x = {}, remaining = {}", x, rem_count);

    //     let v = vec![1,2,3];
    //     for val in v.iter()
    //     {
    //         println!("{}",val );
    //     }
        
    //  for val in v.iter()
    //     {
    //         println!("{}",val );
    //     }

}


fn create_vector(array: &[i32])->Vec<i32>
{
    let mut vect= Vec::new();
    for ele in  array.iter()
    {
        let _val=*ele;
        vect.push(*ele);
    }
    vect
}



fn create_linklist(array: &[i32])->Linklist
{
    let mut linklst=Linklist::Nil;
    for ele in array.iter()
    {
        if linklst == Nil
        {
            linklst = Cons(Rc::new(*ele),RefCell::new(Rc::new(Nil)));
        }
        else
        {
            linklst = Cons(Rc::new(*ele),RefCell::new(Rc::new(linklst)));
        }         
    }
    linklst
}

fn print_linklist(list: &Linklist)
{
    match list
    {
        &Cons(ref x, ref y)=>{
            println!("{}", *x);        
            print_linklist(&**y.borrow());
        },
        &Nil=>return,
    }
}