

//----------------------------------------------------------------------------//

fn normal_function()
{
    println!("JAY GANESH ....");
}

//----------------------------------------------------------------------------//

fn single_parameter_function(ivalue:i8)
{
    println!("Value : {}",ivalue);
}    

//----------------------------------------------------------------------------//

fn multiple_paramter_function(ino_1:i8,ino_2:i8)
{
    let ans:i8 = ino_1+ino_2;
    println!("ADDITION IS : {}",ans);
}

//----------------------------------------------------------------------------//

fn function_with_return(ino_1:i8,ino_2:i8)->i8
{
    ino_1+ino_2
}

//----------------------------------------------------------------------------//

fn function_multiple_return(ino_1:i8,ino_2:i8)->(i8,i8)
{
    let add:i8  = ino_1+ino_2;
    let sub:i8 = ino_1-ino_2;

    (add,sub)
}

//----------------------------------------------------------------------------//



fn main()
{   
    normal_function();
    single_parameter_function(10);
    multiple_paramter_function(10, 20);
    
    let iret = function_with_return(20, 20);
    println!("ADDITION IS : {}",iret);

    let (iret_1,iret_2) = function_multiple_return(30, 20);

    println!("ADDITION : {} AND SUBSTRACTION : {}",iret_1,iret_2);

}