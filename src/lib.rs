extern crate libc;

mod ffi_jv;
mod ffi_jq;

pub fn jq_init() -> *mut ffi_jq::Struct_jq_state  {
    unsafe {
        ffi_jq::jq_init()
    }
}

pub fn jq_teardown(state: *mut ffi_jq::Struct_jq_state) {
    unsafe {
        ffi_jq::jq_teardown(state)
    }
}

#[cfg(test)]
mod test {
    use super::jq_init;
    use super::jq_teardown;
    
    #[test]
    fn it_works() {
        println!(".it works.");

        let mut jq_state = jq_init();
        println!("jq_state: {:?}", jq_state);

        jq_teardown(jq_state);

        assert!(false);
    }
}


