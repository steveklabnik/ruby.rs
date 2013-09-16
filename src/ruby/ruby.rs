use std::libc::c_int;
use std::c_str::CString;

#[link_args = "-lruby"]
extern {
    fn ruby_init();
    fn ruby_options(argc: c_int, argv: *CString);
    fn ruby_run();
    fn ruby_script(name: CString);
    fn ruby_cleanup(ex: c_int);
}

pub fn start_ruby() {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe {
        ruby_init();
        let argc : c_int = 0;
        let argv = &("".to_c_str());
        ruby_options(argc, argv);
        ruby_run();
    }
}

pub fn run_ruby() {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe {
        let source = "40 + 2".to_c_str();
        ruby_script(source);
    }
}

pub fn stop_ruby() {
    #[fixed_stack_segment]; #[inline(never)];
    unsafe{ 
        let ex: c_int = 0;
        ruby_cleanup(ex);
    }
}
