
/*
//-------------------------------------------------------------------------------------------------//
                                    ENUM - Enumeration
//-------------------------------------------------------------------------------------------------//
#    An "enum" in programming, short for "enumeration," 
     is a user-defined data type that consists of a set of named values, known as variants

     ex : 

        enum Fruit{
            Apple,
            Banana,
            Orange
        }

        or 

        enum FoodItem{
            maincourse{name:String,price:u32};
            dessert{name:String,price:u32};
            beverage{name:String,price:u32};
        }

    Why Enums are Useful:
                       1. Clarity: 
                                    Enums make your code more readable and expressive. When you see Fruit::Apple, it's clear that you're talking about an apple.

                       2. Grouping: 
                                    Enums group related values together. In our example, all fruits are part of the Fruit enum.

                       3. Type Safety: 
                                    Enums help prevent mistakes. If you're expecting a fruit, you know it can only be one of the defined variants.

                       4. Pattern Matching: 
                                    Enums are often used with "pattern matching," allowing your code to behave differently based on the specific variant.
*/    
//  Example -1 ]  Imagine you're developing a program to handle food orders at a restaurant. Each order can consist of a main course, a dessert, and a beverage. 
//  However, each of these categories can have various options. Let's explore how you might implement this using structs and enums.
//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------//
// USING STRUCT : 


struct maincourse
{
    name:String,
    portion_size:String,
    price:i16
}

struct Dessert {
    name:String,
    sweetness_level:String,
    price:i16
}

struct Berverage {
    name:String,
    size:String,
    price:i16
}
//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------//
// USING ENUM :
/*
        Advantages of Enums in this Scenario:

            Unified Representation  : All types of food items are now of the same enum type (FoodItem). This allows for a more consistent and unified way to handle different categories.

            Flexibility             : Adding a new type of food item, such as an appetizer or a side dish, is straightforward. You can simply add a new variant to the FoodItem enum.

            Pattern Matching        : Enums are well-suited for pattern matching, allowing you to write code that behaves differently based on the type of food item received in an order.

            Compact Code            : Enums provide a more concise way to represent different categories of items compared to using separate structs for each category.

        Key Differences:
                        1) Single vs. Multiple Types:
                                        Structs : Represent a single type with a fixed set of named fields.
                                        Enums   : Represent a type that can be one of several possible variants, each with potentially different structures.
                                        
                        2) Data vs. Variation:
                                        Structs : Emphasize grouping related data together.
                                        Enums   : Emphasize representing different variations or states of a concept.
                                        
                        3) Mutability:
                                        Structs : Fields are mutable by default.
                                        Enums   : Mutability is variant-specific; each variant can have its own mutability.

                        4) Pattern Matching:
                                        Structs : Not typically used for pattern matching.
                                        Enums   : Well-suited for pattern matching, allowing code to behave differently based on the variant.

                        5) Extensibility:
                                        Structs : Generally less extensible; changing the structure might affect multiple parts of the code.
                                        Enums   : More extensible; adding a new variant doesn't necessarily impact existing code.

*/

enum FoodItem {

    MainCourse{
        name:String,
        portion_size:String,
        price:i32
    },

    Dessert{
        name:String,
        sweetness_level:String,
        price:i16
    },

    Berverage{
        name:String,
        size:String,
        price:i16
    }
}

fn enum_demonstration()
{
    let palak_panir:FoodItem = FoodItem::MainCourse {
        name:String::from("PALAK-PANIR"),
        portion_size:String::from("FULL"), 
        price:350 
    };

    let cake:FoodItem = FoodItem::Dessert {
        name:String::from("CAKE"), 
        sweetness_level:String::from("1 Piece"), 
        price: 120 
    };

    let soda:FoodItem = FoodItem::Berverage {
        name:String::from("SODA"), 
        size:String::from("1 Liter Bottle"), 
        price:120 
    };

    // println!("{:?},{:?},{:?},",palak_panir,cake,soda);
}


fn main()
{
    println!("SHREE GANESH...");

    enum_demonstration();
}