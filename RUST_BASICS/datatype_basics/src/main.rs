use std::collections::HashMap;





fn main()
{

//-------------------------------------------------------------------------------------------------//

    let signed_integer:i8 = 127;          // -127 to 127
    let unsigned_integer:u8 = 255;        // 0 to 255

    println!("SIGNED INTEGER : {}",signed_integer);
    println!("UNSIGNED INTEGER : {}",unsigned_integer);

//-------------------------------------------------------------------------------------------------//

    let float_value_1:f32 = 23.23;

    let float_value_2:f64 = 56.56;

    println!("FLOAT VALUE : {}",float_value_1);
    println!("FLOAT VALUE : {}",float_value_2);

    //------------------------------------------------//

    let float_value_1:f32 = f32::MIN;

    let float_value_2:f32 = f32::MAX;

    println!("FLOAT VALUE MINIMUM : {}",float_value_1);
    println!("FLOAT VALUE MAXIMUM : {}",float_value_2);

//-------------------------------------------------------------------------------------------------//

    let bool_value_1:bool = true;
    let bool_value_2:bool = false;

    println!("BOOLEAN VALUE : {}",bool_value_1);
    println!("BOOLEAN VALUE : {}",bool_value_2);

//-------------------------------------------------------------------------------------------------//

    let string_data:String = String::from("SHREE GANESH");
    let charachter_value:char = 'D';

    println!("STRING DATA : {}",string_data);
    println!("CHARACHTER VALUE : {}",charachter_value);

//-------------------------------------------------------------------------------------------------//

    let array_arr:[i8;5] = [10,20,30,40,50];
    println!("ARRAY DATA : {:?}",array_arr);

//-------------------------------------------------------------------------------------------------//
    
    let slice_crr:&[i8] = &array_arr[1..5];

    println!("SLICE OF ARRAY : {:?}",slice_crr);

//-------------------------------------------------------------------------------------------------//

    let tuple_brr:(i8,f32,char,bool) = (10,12.3,'D',true);

    println!("TUPLE DATA : {:?}",tuple_brr);

//-------------------------------------------------------------------------------------------------//

    let mut vector = vec![10,20,30];

    vector.insert(2, 80);
    vector.push(90);

    println!("VECTOR DATA : {:?}",vector);

//-------------------------------------------------------------------------------------------------//
    let mut hash_map = HashMap::new();

    hash_map.insert("DEMO_1", 10);

    hash_map.insert("DEMO_2", 20);  

    println!("MAP DATA : {:?}",hash_map.get("DEMO_1"));

//-------------------------------------------------------------------------------------------------//

    let x:i8 = 10;
    let y:&i8 = &x;
    let z:&&i8 = &y;

    println!("VALUE OF Z : {}",z);

//-------------------------------------------------------------------------------------------------//

}