use std::{io, env};



fn main()
{
    ///////////////////////////////////////////////////////////////////////////
    //          1. Take Input From User as String

    let mut buffer = String::new();
    println!("ENTER THE DATA : ");
    io::stdin().read_line(&mut buffer).expect("ERROR");
    println!("INPUT DATA : {}",buffer);


    ///////////////////////////////////////////////////////////////////////////

    ///////////////////////////////////////////////////////////////////////////
    //          2. Take Input Parse in Integer

    let mut data:String = String::new();

    println!("ENTER THE NUMBER : ");

    io::stdin().read_line(&mut data).expect("error");

    let number:i32 = data.trim().parse().expect("error");

    println!("PARSE NUMBER : {}",number); 

    ///////////////////////////////////////////////////////////////////////////
    
    ////////////////////////////////////////////////////////////////////////////
    //         3.Command line argumnet

    let argv: Vec<String> = env::args().collect();

    if argv.len() > 1
    {
        println!("ARGUMENT : {}",argv[1]);
    }
    else {
        println!("PROVIDE ARGUMENT");
    }   

}