

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

fn book_immutable(book:&String)
{
    println!("IMMUTABLE BOOK : {}",book);
}

//-------------------------------------------------------------------------------------------------//

fn book_mutable(book:&mut String)
{
    book.push_str("......");
    println!("MUTABLE BOOK :{:?}",book);
}
//-------------------------------------------------------------------------------------------------//

fn return_reference(book_title:&String,length:usize)->&str{              // &str and &String are related but different types
                                                                        // &str ---> this is string slice (portion of string)   and  &String--->This is entire string
    &book_title[..length]                                               
                                                                        // [..length]   this make slice of string from start to given length
}

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















/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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
    let my_book_3:String = String::from("C++ PROGRAMMING BY Bjarne Stroustrup");                    // This is my book 
    let friend_reads_book:&String = &my_book_3;                                                     // here by book is borrow by my friend (NO TRANSFER OF OWNERSHIP)

    println!("MY BOOK : {}",my_book_3);                                                             // Still i can access my book becuase my friend not take book it just looking that
    println!("MY BOOK READ BY FRIEND : {}",friend_reads_book);                                          


//----------------------------------------------------------------------------------------------------//
//                                     MUTABLE BORROWING
//----------------------------------------------------------------------------------------------------//
/*
    1)  Now, imagine you have a book, and you want your friend to make notes in it,
         but you want to make sure they don't take the book home.

    2)  like your friend having permission to write in your book. 
        The book is still yours, but they can make changes under your supervision.
*/  

    let mut my_book_4:String = String::from("PYTHON BOOK BY Guido van Rossum");                     // this is mt book 

    let friend_write_in_book:&mut String = &mut my_book_4;                                          // my friend having permission to write in my book 

    friend_write_in_book.push_str("....");                                                  // my friend make changes or write something in book under my supervision

    println!("IN MY BOOK MY FRIEND WRITE SOMETHING : {}",my_book_4);                                // book is still my book

//----------------------------------------------------------------------------------------------------//
//                                  BORROW CHECKER
//----------------------------------------------------------------------------------------------------//
/*  
    1)  consider the Rust borrow checker as a library assistant 
        who ensures that only one person can read or write in the book at a time.
        In Rust : It ensures that either you have ownership or a reference, but not both at the same time.

    2)  The borrow checker prevents the simultaneous borrowing of the book for reading and writing, 
        ensuring that your book remains safe and sound.

    3)  It ensures that you're not violating the rules, such as trying to use a book that has been moved or modified.    
*/

    let mut my_book_5:String = String::from("GOLANG BY ROBERT GRISERMER");      // this is my book
    let friend_read:&String = &my_book_5;                                       // here one friend looking my friend 

    // let friend_write:&mut String = &mut my_book_5;                            // here error occured beacuse simultaneous borrow not allowed 
                                                                                // we can't perform read write simultaneously on single book  

    println!("MY BOOK IS READ BY MY FRIEND : {}",friend_read);
    println!("MY BOOK : {}",my_book_5);                                         //  i still have access to my book ... beacuase no transfer of ownership





//----------------------------------------------------------------------------------------------------//
//                               RUILES OF :  READ, WRITE, OWN 
//----------------------------------------------------------------------------------------------------//
/* 
    Imagine you have a book, and you want to let your friend read it without giving away your ownership
    The book represents your data, and your friend reading it is akin to using a reference. 
    Now, there are three levels of permissions related to this book:

    1) Read (R) :  Your friend can read the content of the book but can't make any changes or annotations.
    2) Write (W) : Your friend can write notes, highlight paragraphs, or make changes in the book. 
                    This is only possible if you explicitly allow it.
    3) Own (O) :  You have complete ownership of the book, and you can lend it or keep it as you wish.
*/  

    let mut book:String = String::from("RUST BOOK");


    
    let reader:&String = &book;                                                 // Borrow for reading (immutable reference)  READER has read permission
    //let writer:&mut String = &mut book;                                       // if i write this line here means simultaneous mutable and unmutable reference is occur hence error occur 
    println!("READER : {}",reader);                                             // beacuse of this line there is no simultaneous borrowing means first reading is done then writing start

                                                                                // aapan ekach veles 2 pepoles la reading writing sathi single book nahi deu shakat soo ty sathi pahile konte tari ek kaam adhi honar then nanantar dusre kaam
// After the reader is done, now you can borrow for writing


    let writer:&mut String = &mut book;                                         // Borrow for writing (mutable reference) WRITER has writing permission
    println!("WRITER : {}",writer);            

    writer.push_str(".....");                                           // here writer changes in book
    println!("WRITER : {}",writer);
    
    println!("ORIGINAL BOOK : {}",book);                                        // we have access of our original book  but this time book is edited (by writer)


    read_book(book);                                                            // here ownership transfer now function parameter owns the book




/*
    When you pass a variable to a function in Rust, you can either 
       1. TRANSFER OWNERSHIP, 
       2. BORROW IT IMMUTABLE ,
       3. BORROW AS MUTABLE
       
       Let's start with passing by reference
*/    

//----------------------------------------------------------------------------------------------------//
//                          PASSING BY REFERENCE TO THE FUNCTION 
//----------------------------------------------------------------------------------------------------//
/*
    1)  Imagine you have a book, let's call it my_book, which is a physical object representing a story. 
        Now, you want to share information about this book with a friend, without actually giving them the book.

    2)  book_immutable() is like a friend who wants to know the title of your book but doesn't want to own or change anything about it. 
        You pass a reference &my_book to the function, which is similar to letting your friend borrow the book temporarily. 

    3)  The friend can read the title println!("Book Title: {}", book);) 
        but can't keep or modify the original book (my_book) because they only have a reference.

    4)  After the function call, you can still use and own your original book (my_book) in the main function.  
*/

    let my_book_6:String = String::from("JAVASCRIPT BOOK");                     // This is my book             

    book_immutable(&my_book_6);                                           // here i passing my book reference to fucntion .....means here ownership is not transfer




//----------------------------------------------------------------------------------------------------//
//                          MUTABLE BORROWING IN FUNCTION 
//----------------------------------------------------------------------------------------------------//
/*
    1)  Imagine you have a book title, and you want to add a subtitle to it. However, you want to do this in a way that the original title is modified directly, 
        without making a new copy of the book. or without transfer ownership

    2)  The function modifies the borrowed book directly using book.push_str("......");

    3)  These changes are reflected in the original my_book in the main function. 
    
*/

    let mut my_book_7:String = String::from("HTML CSS BOOK");

    println!("BOOK BEFORE MUTABLE FUNCTION : {}",my_book_7);

    book_mutable(&mut my_book_7);
    
    println!("ORIGINAL BOOK AFTER MUTABLE FUNCTION : {}",my_book_7);




//----------------------------------------------------------------------------------------------------//
//                          RETURN REFERENCE FROM FUNCTION
//----------------------------------------------------------------------------------------------------//
/*
    1)  Imagine you have a long book title, and you want to get the prefix/short name of it, maybe for a short summary or display purposes. 
        However, you don't want to create a new string for the prefix; instead, you want to borrow a reference to a part of the original title.

    2)  In this example, the return_reference() function takes a reference to the book title and a length, and it returns a reference to a substring of the original title. 
        It allows you to borrow a part of the title without transferring ownership.

    3)  When you call the function with return_reference(&book_title, 16), you get a reference to the first 16 characters of the book title. 
        This reference (return_reference) is then printed in the main function    
*/

    let book_title:String = String::from("Rust Programming Adventures");

    let return_reference:&str =  return_reference(&book_title,16);

    println!("RETURN REFERNCE : {:?}",return_reference);

}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


