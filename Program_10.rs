
fn main()
{


    let tuple_ex = (10,10.2,true,"hello");
    println!("{:?}",tuple_ex);



    let array_x = [1,2,3,4,5];
    println!("{:?}",array_x);

    let array_y:[i8;5] = [12,45,78,56,45];
    println!("{:?}",array_y);

    let array_z:[i8;100] = [10;100];
    println!("{:?}",array_z);
}