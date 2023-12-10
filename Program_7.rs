

fn main()
{
    let logical :bool = true;
    println!("{}",logical);

    let float_variable :f32 = 23.56; // regular 
    println!("{}",float_variable);
 
    let number = 10i16;          // suffix
    println!("{}",number);

    let num = 10;                // default
    println!("{}",num);


    let mut ino = 11;
    println!("{}",ino);

   // ino  = true ----------error
   
    ino = 45;
    println!("{}",ino);

    let ino  = true;                //shadowing
    println!("{}",ino);

}