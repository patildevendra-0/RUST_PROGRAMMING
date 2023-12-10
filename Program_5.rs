
fn call_me(num:i8)
{
    for i in 0..num
    {
        println!("{}",i+1);
    }
}

fn main()
{
    call_me(3);
}