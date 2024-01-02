

/*
    -------------------------------------------------------------------------------------------------
                                            OWNERSHIP RULES
        1.Each value in rust has owner
        2.There can be one owner at a time
        3.When the owner goes out of scope, the values will be dropped


        ex :
            {
                let data = "hello"            

                ///
                /// 
                /// 
                /// 
                /// 
                /// 
                /// -------- do anything you want with this string 
                  
            }    ------> this scope is now over and data string is not valid

            : data string is only valide within curly braces


    -------------------------------------------------------------------------------------------------
*/

//-------------------------------------------------------------------------------------------------//
/*
                                        OWNERSHIP IN REAL LIFE


    example:

            Imagine you have a car. In Rust's world, the car is like a piece of data in the computer's memory. 
            Now, let's introduce the concept of ownership.


        1)Single Owner:

            You are the sole owner of your car. Only you can drive it and decide what happens to it.
            In Rust, a piece of data in memory (like your car) can have only one owner.

        2)Transferring Ownership:

            If you decide to sell your car, ownership transfers to the new owner. Now, they have control over the car.
            Similarly, in Rust, ownership of data can be transferred from one variable to another through moves.

        3)Access Through Owner:

            The new owner can use the car however they want, but you can't drive it anymore. You've given up ownership.
            In Rust, only the current owner of data can access and modify it. Once ownership transfers, the previous owner can't use it.
            
        4)Memory Deallocation:

            If you sell your car, you don't need to worry about it when someone else owns it. They are responsible for it.
            In Rust, when the owner of data goes out of scope, Rust automatically deallocates (returns) the memory associated with that data.
           
        4)Undefined Behavior:

            Imagine if, after selling your car, you try to drive it. That's undefined behavior – it doesn't make sense.
            Similarly, in Rust, trying to use data after it has been moved to another owner leads to undefined behavior.
           
        5)Cloning:

            If you want to keep a copy of your car, you can clone it. Now, you have two identical cars.
            In Rust, if you want two variables to own the same data, you can clone it. Both variables will have their own copies.

            let my_car:String = String::from("BMW");     
            let friend_car = my_car.clone();
                                                        ..... now 

*/
//-------------------------------------------------------------------------------------------------//

fn share_copy_book(book:String){
    println!("SHARE BOOK : {}",book);
}

//-------------------------------------------------------------------------------------------------//

fn read_and_return_book(book:String)->String{
    println!("BOOK READING : {}",book);
    book
}

//-------------------------------------------------------------------------------------------------//

fn read_book(book:String){
    println!("READING BOOK : {}",book);
}

//-------------------------------------------------------------------------------------------------//

fn main()
{
//-----------------------------------------------------------------------------------------------------//
//                                        OWNERSHIP TRANSFER
//----------------------------------------------------------------------------------------------------//
    let my_car:String = String::from("BMW");
    
    println!("MY CAR : {}",my_car);

    let friend_car:String = my_car;                 // now here a give ownership of my car to my friend

    // println!("MY CAR : {}",my_car);              soo here error occur beacause ownership trasfer soo
    println!("FRIEND CAR : {}",friend_car);         // now my friend is owner of car

   


//-----------------------------------------------------------------------------------------------------//
//                                             CLONE
//----------------------------------------------------------------------------------------------------//
    let other_car:String = String::from("MUSTAG");
    let other_friend:String = other_car.clone();           // here i created clone of other car means both are identical/same due that we can perform separate separate operations on each

    println!("OTHER CAR : {}",other_car);
    println!("FRIEND CAR : {}",other_friend);




//-----------------------------------------------------------------------------------------------------//
//                                  TRANSFER THE OWNERSHIP THROUGH FUNCTION 
/*//----------------------------------------------------------------------------------------------------//
    1)    
        Imagine you have a book, and you want to lend/give it to a friend. 
        In Rust, this is similar to passing ownership through functions.
  
*/

    let book_name_1:String = String::from("LINUX PROGRAMMING INTERFACE");
    read_book(book_name_1);
   // println!("{}",book_name)                                         .......... here error occure becuase ownership is transffered to function parameter





//-----------------------------------------------------------------------------------------------------//
//                                  RETURNING THE OWNERSHIP THROUGH FUNCTION 
/*//----------------------------------------------------------------------------------------------------//
    1)    
        let's say your friend reads the book and returns it.
        In Rust, this is like receiving ownership back from a function.

*/
    let book_name_2:String = String::from("RUST PROGRAMMING..");                
    let return_book:String = read_and_return_book(book_name_2);                 // ownership trasfer to read_and_return_book() function
                                                                                // read_and_return_book()   function return book ---- means return ownership
    println!("RETURN BOOK AFTER READING COMPLETED : {}",return_book);           // we take ownership in return_book variable

    



//-----------------------------------------------------------------------------------------------------//
//                                  OWNERSHIP AND CLONING (sharing copy of book)
/*//----------------------------------------------------------------------------------------------------//
    1)    
        Now, consider the scenario where you want to share a copy of a book with a friend without giving your original book. 
        In Rust, this is similar to cloning.

*/
    let original_book:String = String::from("C - PROGRAMMING BY DENNIS RITCHI");                    // This is original book
    let copy_book:String = original_book.clone();                                                   // here we make copy of original book

    share_copy_book(copy_book);                                                                     // we share clone or copy book with friend not original book
    println!("ORIGINAL BOOK : {}",original_book);                                                   // our original book is safe near to us we can use that for our purpose


    

//----------------------------------------------------------------------------------------------------//
//                                      BORROWING
//----------------------------------------------------------------------------------------------------//
/*
    1)  Imagine you have a unique, handwritten book, and it's in your possession. This book is your "owned" data.
    2)  Now, let's say you want to show your book to a friend without giving it away. This is where borrowing comes in.

*/ 


//----------------------------------------------------------------------------------------------------//
//                                     MUTABLE BORROWING
//----------------------------------------------------------------------------------------------------//
/*
    1)  Now, imagine you have a book, and you want your friend to make notes in it,
         but you want to make sure they don't take the book home.

    2)  like your friend having permission to write in your book. 
        The book is still yours, but they can make changes under your supervision.
*/  

}