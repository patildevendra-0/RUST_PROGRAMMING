

fn main()
{
//--------------------------------------------------------------------------------------------------//
    let teampreture :i8 = 40;

    if teampreture > 30
    {
        println!("It is hot day....");
    }
    else if teampreture <= 20 {
        println!("cool day...")
    }
    else{
        println!("normal day")
    }

//--------------------------------------------------------------------------------------------------//

//--------------------------------------------------------------------------------------------------//

    let number = 40;

    match number {
        0 => println!("zero"),
        10..=20 => println!("Between 10 to 20"),
        21..=30 => println!("Between 20 to 30"),
        31..=40 => println!("Between 30 to 40"),
        _ =>println!("some thing else")
    }

//--------------------------------------------------------------------------------------------------//

}