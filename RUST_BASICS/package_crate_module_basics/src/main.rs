/*
//////////////////////////////////////////////////////////////////////////////////////////////////////
//--------------------------------------------------------------------------------------------------------------------//
//                               PACKAGE , CRATE , MODULE IN RUST                                                     //
//--------------------------------------------------------------------------------------------------------------------//



    Certainly! Let's use the analogy of organizing a clothes cupboard:
    

        1) Single Clothes Cupboard vs. Multiple Cupboards:

                1.  Initially, you have all your clothes in one big cupboard. As your clothing collection grows, it becomes hard to find a specific item or make changes.
                2.  Organizing into smaller cupboards based on clothing types (shirts, pants) is like splitting your cupboard into sections for better organization.
        
        2) Packages and Crates:

                1.  Your clothing collection is like a package. Inside it, you have different cupboards (crates) dedicated to specific types of clothes.
                2.  Each cupboard (crate) can produce a set of outfits (library) or a complete ensemble (executable) for a specific occasion.
        
        3) Modules and Use:

                1.  Within each cupboard, you organize your clothes further. For example, one cupboard might have casual shirts, another might have formal shirts.
                2.  Modules are like labeled sections within each cupboard. The "use" statement is like deciding which cupboard you want to open and find clothes in.

        4) Paths:

                1.  Each clothing item has a name (like "Blue Jeans"). A path is how you refer to a specific item in a specific cupboard. For example, "Casual Cupboard -> Blue Jeans."
                2.  Paths help you precisely locate and wear a particular item in your clothing collection.

        5) Encapsulating Implementation Details:

                1.  You don't want everyone rearranging your perfectly organized cupboard. You provide a clear set of instructions (public interface) for others to choose from.
                2.  The way you organize your clothes (code) determines what's public (wearable by others) and what's private (internal details for you to change without affecting others).
        
        6) Scope:

                1.  Imagine you're working on different parts of your cupboard in separate rooms. Each room is a scope, and you can't have two clothes with the same name in the same room.
                2.  Rust helps you manage these "rooms" (scopes) and resolves conflicts when you have similar clothes in different places.
        
        7) Rust's Module System:

                1.  In Rust, there are features (like Cargo, crates, modules, and paths) that help you manage your clothes cupboard effectively.
                2.  Understanding and using these features is like becoming a fashionista who knows where each item is, what others can wear, and how to avoid conflicts.

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    SIMPLY....

        1) Package (Clothes Cupboard):
                    Your entire clothes cupboard is like a package in Rust. It's the overarching collection that contains everything.
        
        2) Crates (Sections in Cupboard):
                    Each section within your cupboard, like the T-shirt section, formal shirt section, jeans section, etc., is like a crate in Rust. Each crate is dedicated to a specific type of clothing.
        
        3) Modules (Organization within Sections):
                    Within each section, you can further organize your clothes. For instance, within the T-shirt section, you might have modules for graphic T-shirts, plain T-shirts, etc. These modules represent specific organizational units within each crate.
        
        4) Paths (Locating Specific Clothes):
                    Each individual item of clothing, like a blue jeans or a formal shirt, is like an item in Rust. The path to these items, such as "Jeans Section -> Blue Jeans," helps you precisely locate and use a particular piece of clothing.
        
        5) Encapsulation (Public and Private):
                    You decide what's accessible to others (public) and what's kept private within each section. For example, you might want others to access the graphic T-shirts but not rearrange the internal organization of that section.
        
        6) Scope (Different Sections as Scopes):
                    Imagine each section of your cupboard as a separate room. Each room is a scope, and you manage conflicts by making sure each clothing item has a unique name within its respective section.
        
        7) Rust's Module System:
                    In Rust, features like Cargo, crates, modules, and paths help you effectively manage your clothes cupboard (code). Understanding and using these features allow you to control visibility, organize your code, and avoid conflicts, just like in your cupboard.


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//--------------------------------------------------------------------------------------------------------------------//
//                                                  TYPES OF CRATES                                                   //
//--------------------------------------------------------------------------------------------------------------------//

        # TYPES OF CRATE : 
                            1. BINARY Crate
                            2. LIBRARY Crate


                            1) Binary Crates:
                                            Characteristics:
                                                        1. Contains an executable that can be run.
                                                        2. Must have a function called main as the entry point, defining what happens when the executable is run.
                                            Use Case:
                                                        1. Programs you compile to produce standalone executables, such as command-line tools, servers, or applications.
                            
                            2) Library Crates:
                                            Characteristics:
                                                        1. Does not contain a main function.
                                                        2. Does not compile to an executable on its own.
                                            Use Case:
                                                        1. Provides functionality intended to be shared with multiple projects.
                                                        2. Often used as a dependency in other projects.

        # Ex: 

            1) Binary Crate - Special Outfit:           Consider one section, let's say the formal wear section, as a binary crate. It's like a special outfit that has a main purpose, perhaps for events or formal occasions. This section (binary crate) has a "main" outfit, defining what happens when you wear it.
            2) Library Crate - Essential Accessories:   Other sections, like the accessories drawer, contain items that complement various outfits. This is like a library crate in Rust. It doesn't have a "main" outfit (function) on its own but provides functionalities (accessories) that can be shared across different outfits (sections).
            3) Crate Root - Cupboard Door:              The cupboard door is like the crate root. It's the starting point when you decide what to wear. It lists all the sections (modules) in your cupboard, making it easy to locate specific items.


        #NOTE               

            A crate can be exclusively a binary crate or a library crate. However, a package (which can contain one or more crates) can have both types. For example, a package might have a binary crate for the main application and a library crate for shared functionality.
        
        Here's a summary:

            1) Binary Crates:
                   1. Executable.
                   2. Has a main function.
                   3. Standalone program.

            2) Library Crates:
                   1. Not executable on its own.
                   2. No main function.
                   3. Provides functionality to be used by other programs. 


/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//--------------------------------------------------------------------------------------------------------------------//
//                                               MODULES                                                              //
//--------------------------------------------------------------------------------------------------------------------//

    PACKAGE ------> CRATE ------> MODULES ------> SUB-MODULES
                      |  
                      |
                      |----> 1. Binary
                      |----> 2. Library

                
                            |----->  Kitchen
    HOUSE ------- ROOMS ----|----->  BedRoom
   (crate)      (module)    |----->  LivingRoom   
                                        --------(sub-modules) 


    EX : 
        1) House as Codebase:
                Imagine your entire house as your codebase.

        2) Rooms in the House as Modules:
                Within your codebase (house), think of different rooms like the living room, kitchen, and bedroom. Each room is a module in Rust.
                Modules are like rooms where you group related activities (functions and code) together.
        
        3) Sections in Each Room --------------------- NESTED MODULES:
                Inside each room (module), there are sections representing different purposes. For example, in the kitchen, you might have a section for cooking and another for washing dishes. These sections are like nested modules within the main room.
                These sections (nested modules) keep related tasks organized.
        
        4) Main Areas of the House -------------------- Crate Roots:
                The main areas of your house, like the living room or the backyard, are like crate roots in Rust. These areas serve as starting points for specific activities.
                In code terms, certain files (like src/main.rs or src/lib.rs) act as starting points for the compiler.
        
        5) House Blueprint ----------------------------- Module Tree:
                Visualize the structure of your house as a blueprint. At the top is the entire house crate, and below are different rooms (modules) and their subsections (sections).
                This blueprint helps you understand how different parts of your code are organized.
        
        6) Family Members as Code Users:
                Family members living in your house are like programmers who use your code. They don't need to understand every detail of the house layout; they just want to go to the rooms and sections they are interested in.
                Modules help them navigate through your codebase efficiently.
        
        7) Organization for Efficiency:
                Just as you organize your house into rooms and sections for efficient living, modules help organize your code for readability and maintainability.
                If someone wants to work on bedroom-related tasks, they know to look in the bedroom module.
        
        8) House Blueprint vs. Directory Structure:
                The house blueprint is similar to the directory structure on your computer. It's a way to structure and organize information.
                Each module is like a room in the house, and the hierarchy defines relationships between different parts of your code, just as directories contain files and subdirectories.


*/




fn main()
{

}