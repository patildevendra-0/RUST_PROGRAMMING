

fn main()
{
    //----------------------------------------------------------------------------------------------//    
    //                       Immutability and Mutability


    let numutable_number:i16 = 30;
 
    println!("Number : {}",numutable_number);

    /*
        if 
            number = 35      .... we try to change the value of number then error occur
                                beacuse we can not change the value of immutable variables

        # if we declare variable like this     let data =  
            then this variable is bydefault non mutable

        # if we want to make mutable variable then we want use explecitely mut keyword
            let mut number =     
     */
    
    let mut mutable_number:i16 = 89;

    println!("BEFORE CHANGING VALUE : {}",mutable_number);

    mutable_number = 56;

    println!("AFTER CHANGING VALUE : {}",mutable_number);

        
    //----------------------------------------------------------------------------------------------//    
    //                              Constants

    const MAX_SCORE:i32 = 56;

    println!("CONSTANT  : {}",MAX_SCORE);

    /*  
        # We can not chage the value of constant variable  after declaration through out the program
    */

    //----------------------------------------------------------------------------------------------//
    //                              Shadowing
    /*
        Think of having a notebook where you first write a note, and then you use the same notebook for a different note. 
        The second note "shadows" of the first one. 
        In Rust, you can reuse a variable name for different things within the same area of your code.
    */

    let x = 5;           // original variable    
    let x = x+10;       // same variable name but with differernt thing
    let x  = x*20;      // same variable with different things

    println!("VALUE OF X : {}",x);


    let x = 89;
    println!("VALUE OF X : {}",x);


    let data = "hello rust";           // this string but becuse of shadowing 
    let data = data.len();              // we can use that same variable for another purpose

    println!("DATA : {}",data);

    //----------------------------------------------------------------------------------------------//
       
}   