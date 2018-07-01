extern crate leetCodeLib;



fn main() {
    leetCodeLib::two_sum::printHelloWorld();
    let arr=[1,2,3,4];
    let vec = create_vector(&arr);
    println!("{:?}",vec );
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