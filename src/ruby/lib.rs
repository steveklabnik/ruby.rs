#[link(name = "ruby",
       vers = "0.1.0",
       uuid = "83cb3400-1e56-11e3-8224-0800200c9a66",
       url = "https://github.com/steveklabnik/ruby.rs")]; 

#[desc = "Ruby-Rust interop."];
#[license = "MIT"];
#[crate_type = "lib"];

pub mod ruby;
