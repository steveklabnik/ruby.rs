#[link(name = "ruby",
       vers = "0.1.0",
       uuid = "83cb3400-1e56-11e3-8224-0800200c9a66",
       url = "https://github.com/steveklabnik/ruby.rs")]; 

#[desc = "Ruby-Rust interop."];
#[license = "MIT"];
#[crate_type = "lib"];

use std::libc::c_int;
use std::libc::uintptr_t;
use std::c_str::CString;
use std::str;

#[link_args = "-lruby -lgmp -lcrypt"]
extern {
    fn ruby_init();
    fn ruby_script(name: CString);
    fn ruby_cleanup(ex: c_int);
    fn rb_eval_string(str: CString) -> uintptr_t;
    fn rb_string_value_cstr(value: uintptr_t) -> CString;
}

pub fn start_ruby() {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe {
        ruby_init();
        ruby_script("rustlang".to_c_str());
    }
}

pub fn run_ruby() {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe {
        let source = "40 + 2".to_c_str();
        let result = rb_eval_string(source);
        let cstr = rb_string_value_cstr(result);
        let bytes = cstr.as_bytes();
        println(str::from_utf8(bytes));
    }
}

pub fn stop_ruby() {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe{ 
        let ex: c_int = 0;
        ruby_cleanup(ex);
    }
}
