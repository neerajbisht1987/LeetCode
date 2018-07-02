extern crate leetCodeLib;



fn main() {
    leetCodeLib::print_hello_world();
    let arr=[1,2,3,4];
    let vec = create_vector(&arr);
    let pair = leetCodeLib::two_sum::two_sum(&vec, 5);
    println!("{:?}",pair );
}


fn create_vector(sar:&[i32])->Vec<i32>
{

    let mut vect= Vec::new();
    for ele in  sar.iter()
    {
        let _val=*ele;
        vect.push(*ele);
    }
    vect
}