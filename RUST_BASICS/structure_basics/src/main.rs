
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
    
}
