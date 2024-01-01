use std::{io, env};



fn main()
{

//-------------------------------------------------------------------------------------------------//    

    let mut buffer:String = String::new();

    println!("ENTER THE DATA AS INPUT : ");

    io::stdin().read_line(&mut buffer).expect("ERROR WHILE ACCEPTING INPUT..");

    println!("DATA FROM USER AS INPUT : {}",buffer);

//-------------------------------------------------------------------------------------------------//  

//-------------------------------------------------------------------------------------------------//    

    let mut data:String = String::new();

    println!("ENTER THE NUMBER : ");
    
    io::stdin().read_line(&mut data).expect("EORROR");

    let ino:i8 = data.trim().parse().expect("ERROR");

    println!("NUMBER AS INPUT : {}",ino);

//-------------------------------------------------------------------------------------------------//    

    let argv:Vec<String> = env::args().collect();

    if argv.len() > 1
    {
        println!("FILE NAME : {}",argv[0]);
        println!("COMMAND LINE INPUT : {}",argv[1])
    }
    else {
        println!("PLS PROVIDE ARGUMNETS ............");
    }


//-------------------------------------------------------------------------------------------------//    




//-------------------------------------------------------------------------------------------------//    

}