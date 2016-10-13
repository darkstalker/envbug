use std::env;

fn main()
{
    println!("{:?}", env::var("LD_LIBRARY_PATH"));
}
