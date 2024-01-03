
//use std::ops::Add;

//-------------------------------------------------------------------------------------------------//
//                                   STRUCT INTRODUCTION
//-------------------------------------------------------------------------------------------------//
/*
    
    1)  In Rust, a struct (short for structure) is a custom data type that allows you to group together related pieces of data under a single name. 
        It's a way to create complex data structures with different fields, each having its own data type

    2)  ex : 
                struct Person
                {
                    name: String,
                    age:u32,
                    is_adult:bool
                }    

    3)  Structure types :   
                            1. Named Structure
                            2. Tupes Structure
                            3. Unit-Like structure
*/              

//-------------------------------------------------------------------------------------------------//
//                                   DEFINING STRUCTURE
//-------------------------------------------------------------------------------------------------//
/*
    1) When i create struct .i define what type of information it will hold and give name to each feild or data
    2) A user account is a collection of various details, such as whether the account is active, the username, email, and the number of times the user has signed in.
*/

struct User
{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}


//-------------------------------------------------------------------------------------------------//
//                             CREATING INSTANCE OR OBJECT OF STRUCTURE
//-------------------------------------------------------------------------------------------------//
/*
    1) Once you have a struct template, you can create an instance/object by filling in the actual data for a specific user.
    2) ex : Filling out user record form with specific values for each feild  ex : Entring username,active,email,sign_in_count
*/

fn user_instance_immutable()
{
    let user_obj_1 = User{
        active:true,
        username:String::from("ABC"),
        email:String::from("abc1@gmail.com"),
        sign_in_count:1
    };

    println!("USER NAME : {}",user_obj_1.username);
    println!("USER EMAIL : {}",user_obj_1.email);
    println!("SIGN IN COUNT : {}",user_obj_1.sign_in_count);
    println!("STATUS: {}",user_obj_1.active);

    // user_obj_1.active = false;                                               error occure because this is immutable instance or object
}


//-------------------------------------------------------------------------------------------------//
//                          MUTABLE INSTANCE OR OBJECT
//-------------------------------------------------------------------------------------------------//
/*  
    1)  let's consider a real-life scenario where we have a user profile, and we want to update some information about that user. 
        The scenario is that a user changes their email address, and we need to update their profile with the new email.
*/
fn user_instance_mutable()
{
    let mut user_obj_2 = User
    {
        active:false,
        username:String::from("XYZ"),
        email:String::from("---------"),
        sign_in_count:1
    };

    println!("USER NAME : {}",user_obj_2.username);
    println!("USER MAIL : {}",user_obj_2.email);
    println!("USER ACCOUNT STATUS: {}",user_obj_2.active);
    println!("USER SIGN IN COUNT : {}",user_obj_2.sign_in_count);

    user_obj_2.email = String::from("xyz123@gmail.com");

    println!("USER MAIL AFTER UPDATE : {}",user_obj_2.email);
}

//-------------------------------------------------------------------------------------------------//
//                              FUNCTION WITH STRCUTURE .... which return object
//-------------------------------------------------------------------------------------------------//
/*
    1) Create functions that construct and return instances of the struct with specific values
*/

fn build_user(username:String,email:String)->User                               // here shorthand init is used see parameter of function and struct feild are same soo don't use again again that .....
{                                                                               //  Rust understands that you want to map the function parameters directly to the struct fields. 
    User{
        username,
        email,
        active:true,
        sign_in_count:1
    }
}

//-------------------------------------------------------------------------------------------------//
//                                  STRUCT UPDATE SYNTAX                      ... means use another object and make some change and make new object
//-------------------------------------------------------------------------------------------------//
/*
    1)  Imagine you work for a car manufacturing company, and you have a standard model of a car with certain features. 
        Now, you want to create a new variant of the car with a few modifications.

    2)  Standard Car Model (car1):
                                    Color: Red
                                    Engine: 2.0L
                                    Airbags: Yes
                                    Sunroof: No

        Updated Car Variant (car2):
                                    Color: Blue (change from the standard model)
                                    Engine: 2.0L
                                    Airbags: Yes
                                    Sunroof: Yes (addition to the standard model)  

    OR one more example

    3)  Imagine you have a standard model of a smartphone, and you want to create a personalized version with a few modifications to suit your preferences.  

            Standard Smartphone Model (phone1):
                                            Brand: XYZ
                                            Storage: 64GB
                                            Color: Black
                                            Face Recognition: Yes

            Personalized Smartphone (phone2):   
                                            Brand: XYZ
                                            Storage: 128GB           (upgrade from the standard)
                                            Color: White             (change from the standard)
                                            Face Recognition: Yes                                 
*/

struct Car 
{
    color:String,
    engine:f64,
    airbags:bool,
    sunroof:bool
}

fn update_car()
{
    let car_obj_1:Car = Car{
        color:String::from("RED"),
        engine:2.0,
        airbags:true,
        sunroof:false
    };

    println!("STANDARD CAR COLOR: {}",car_obj_1.color);
    println!("STANDARD CAR ENGINE: {}",car_obj_1.engine);
    println!("STANDARD CAR AIRBAG: {}",car_obj_1.airbags);
    println!("STANDARD CAR SUNROOF: {}",car_obj_1.sunroof);
    println!("--------------------------------------------");

    
    let car_obj_2:Car = Car{
        color:String::from("BLUE"),
        sunroof:true,
        ..car_obj_1
    };

    println!("UPDATED CAR COLOR : {}",car_obj_2.color);
    println!("UPDATED CAR ENGINE : {}",car_obj_2.engine);
    println!("UPDATED CAR AIRBAG : {}",car_obj_2.airbags);
    println!("UPDATED CAR SUNROOF : {}",car_obj_2.sunroof);


}

//-------------------------------------------------------------------------------------------------//
//                                   TUPLE STRUCT     
//-------------------------------------------------------------------------------------------------//
/*
        # Difference between normal tuple and struct tuple

    1) NORMAL TUPLE : 
                    Regular tuples are collections of values with different types.
                    Elements are accessed using numerical indices (e.g., person.0, person.1).
                    Tuple elements don't have names; their order determines their meaning.

                    ex:   let person = ("DEMO",30,true);

                          let name = person.0;
                          let age  = person.1;
                          let active = person.2;

    2) TUPLE STRUCT : 
                    Tuple structs are similar to regular tuples but defined as distinct types using struct.
                    Elements are accessed using numerical indices (e.g., person.0, person.1).
                    Tuple structs allow you to give a name and a type to the whole tuple, providing a bit more clarity compared to regular tuples.
                       
                    ex :        struct Person(&str,u8,bool);

                                let person = Person("demo",30,false);

                                let name = person.0;
                                let age  = person.1;
                                let active = person.2;
*/
fn normal_struct()
{
    let person = ("abc",30,true);

    let name = person.0;
    let age = person.1;
    let active = person.2;

    println!("NAME : {}",name);
    println!("AGE : {}",age);
    println!("ACTIVE : {}",active);

}

struct Person(String,u8,bool);

fn struct_tuple()
{
    let person = Person(String::from("xyz"),32,false);

    let name = person.0;
    let age = person.1;
    let active = person.2;

    println!("NAME : {}",name);
    println!("AGE : {}",age);
    println!("ACTIVE : {}",active);
}

//-------------------------------------------------------------------------------------------------//
//                                  UNIT LIKE STRUCT 
//-------------------------------------------------------------------------------------------------//
/*
        1) unit like struct in rust is struct without any feilds     ..... simply empty structure
        2) ex : 
                struct Person;
        3) It's specifically used when you want to create a type that doesn't carry any data and is often used as a marker or identifier.         

        4) Imagine you work in a software development team, and every morning, your team has a daily meeting to discuss progress and plans. 
            To help team members remember the daily standup meeting, a simple reminder system is implemented using a Reminder unit-like struct.
*/

struct Reminder;

fn set_reminder(reminder:&Reminder,message:&str)
{
    println!("MESSAGE : {}",message);
}

fn unit_like_struct()
{
    let morning_meet:Reminder = Reminder;

    set_reminder(&morning_meet,"MORNING PLAN MEETING...");

    let afternoon_meet:Reminder = Reminder;

    set_reminder(&afternoon_meet,"AFTERNOON MEET PROGESS CHECK..");

}

//-------------------------------------------------------------------------------------------------//
//                                    METHODS 
//-------------------------------------------------------------------------------------------------//
/*
    # DIFFERENCE BETWEEN METHODS AND FUCNTION :

        FUNCTION :  This is standalone piece of code which takes input process it and generate output

        METHOD   :  Function which is associated with particular type like structure or enum and we call that function using object or instance of that type

        ex : 
            1)  fn addition(ino_1:i8,ino_2:i8)             .....................> THIS IS STANDALONE FUCNTION 
                {
                    let add:i8 = ino_1 + ino_2 ;
                    println("ADDITION IS : {}",add);
                }

            2) 
                struct Calculator
                {
                    value_1:i8,
                    value_2:i8
                }   

                impl Calculator
                {
                    fn add(&self)->i8                   ........................> THIS IS FUNCTION IS ASSOCIATED WITH STRUCTURE CALCULATOR
                    {
                        self.value_1 + self.value_2
                    }
                }

                1]  self is a special parameter in methods, representing the instance the method is called on.
                    It allows methods to access and manipulate the data within the instance.

                2]  Methods are called using dot notation: object.method();

                3]  Each struct can have multiple impl blocks.

                4]  methods can take different forms of self as their first parameter: 
                    1.  &self for immutable borrowing, 
                    2.  &mut self for mutable borrowing, and 
                    3.  self for taking ownership.

                5] TYPES OF METHOD : 
                                    1.Instance Method :
                                                      *  Associated with an instance/object of a type (structure)
                                                      *  Defined inside an impl block for a specific type.
                                                      *  Access instance data USING SELF parameter.                     ......imp

                                    2.Associcated Function (STATIC METHOD):
                                                      *  Associated with a type(structure) itself, not with instances/object.
                                                      *  Often used for creating instances or type-related operations.
                                                      *  Defined inside an impl block for a type but WITHOUT a SELF parameter.      .... imp                                               
                                    
                                    3.

*/
// 1st Example: 
/*
            1)  Imagine you are working on a graphical user interface (GUI) application where you have a Window struct representing the application window.
                You want to provide a method to resize the window:
*/

struct Window {
    width:u32,
    height:u32
}

impl Window{
    fn resize(&mut self,new_width:u32,new_height:u32)                           // instance method
    {
        self.width = new_width;
        self.height = new_height;
    }
    
}

fn method_demonstration_1()
{   
    let mut w_obj = Window{
        width:800,
        height:600
    };

    println!("WINDOW WIDTH : {} & WINDOW HEIGHT : {}",w_obj.width,w_obj.height);

    w_obj.resize(1024, 800);

    println!("NEW WINDOW WIDTH : {} & NEW WINDOW HEIGHT : {}",w_obj.width,w_obj.height);

}

// 2nd Example
/*
        1)  Certainly! Let's consider a scenario involving a library management system where we have a Book struct, 
            and we want to implement methods for borrowing and returning books.

        2)  Static Methods or Associated Functions:
                In Rust, a static method is a function associated with a type(structure) itself, not with instances/object of the type(structure).
                These functions are often used for operations that are related to the type but don't require an instance of the type.

                In Rust, a static method is like a helper function connected to a type itself, not to specific objects of that type. 
                This kind of function is often used for tasks that involve the type as a whole, rather than something specific to an individual instance of the type.
            
            Syntax for Calling Associated Functions:
                Associated functions are called on the type itself, not on an instance. 
                Use the syntax    TypeName::function_name().  

*/

struct Book {

    title:String,
    author:String,
    is_avilable:bool
    
}

impl Book {
                                                                                // Static Method / Associated Function:            calling   ..... let my_book = Book::new_book("The Title", "The Author");
    fn new_book(title:&str,author:&str)->Book                                   // The function new_book() is associated with the Book type, not with a specific instance of Book.
    {                                                                           // It's used for creating a new instance of Book.   They're like actions that make sense for the entire group of things rather than something you do to a single thing. 
        Book{

            title:title.to_string(),
            author:author.to_string(),
            is_avilable:true
        }
    }
                                                                                // The function borrow_book() in your code is an instance method. In Rust, an instance method is a function associated with a specific instance of a type. In this case, it's associated with a particular Book instance.
    fn borrow_book(&mut self)                                                   // calling  :  my_book.borrow_book();
    {                                                                           // It operates on the particular instance (the self parameter).
        if self.is_avilable{                                                    
            println!("YOU BORROWED  : {}  by {} ",self.title,self.author);
            self.is_avilable = false;
        }else {
            println!("SORRY BOOK IS CURRENTLY UNAVILABLE...");
        }
    }

    fn return_book(&mut self)
    {
        println!("YOU RETURNED : {} by {}",self.title,self.author);
        self.is_avilable = true;
    }

    fn is_avilable(&self)->bool
    {
        self.is_avilable
    }
    
}

fn method_demonstration_2()
{
    let mut b_obj = Book :: new_book("C Programming","Dennis Ritchi.." );            //The new_book associated function is used to create a new book without the need for an existing instance.
                                                                                                        //You can directly call Book::new_book(...) to create instances of the Book struct.
    
    b_obj.borrow_book();                                                                                // here we instance method for that we . operator    

    if b_obj.is_avilable()
    {
        println!("The book is available for borrowing.");
    }else {
        println!("The book is currently unavailable.");
    }



    b_obj.return_book();

    if b_obj.is_avilable()
    {
        println!("The book is available for borrowing.");
    }else {
        println!("The book is currently unavailable.");
    }}



//-------------------------------------------------------------------------------------------------//
//                                     OPERATOR OVERLOADING
//-------------------------------------------------------------------------------------------------//


//-------------------------------------------------------------------------------------------------//
//
//-------------------------------------------------------------------------------------------------//


//-------------------------------------------------------------------------------------------------//
//
//-------------------------------------------------------------------------------------------------//


//-------------------------------------------------------------------------------------------------//
//
//-------------------------------------------------------------------------------------------------//

fn main()
{
    user_instance_immutable();
    println!("--------------------------------------------");
    user_instance_mutable();
    println!("--------------------------------------------");

    let name:String = String::from("DEMO");
    let email:String = String::from("demo123@gmail.com");

    let user_obj_3:User = build_user(name, email);

    println!("USER 3: {}",user_obj_3.username);

    println!("--------------------------------------------");

    update_car();
    println!("--------------------------------------------");

    normal_struct();
    println!("--------------------------------------------");

    struct_tuple();
    
    println!("--------------------------------------------");

    unit_like_struct();

    println!("--------------------------------------------");

    method_demonstration_1();

    println!("--------------------------------------------");

    method_demonstration_2();

    println!("--------------------------------------------");

    
}
