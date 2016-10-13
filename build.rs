extern crate pkg_config;

fn main()
{
    println!("cargo:warning=running build.rs");
    pkg_config::find_library("x11").unwrap();
}
