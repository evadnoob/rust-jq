extern crate libc;

mod ffi_jq;
mod ffi_jv;

mod jq {

    use super::ffi_jq::jq_state;
    use super::ffi_jq;
    use super::ffi_jv;
    use std::ffi::CString;

    pub fn init() -> *mut jq_state {
        unsafe {
            super::ffi_jq::jq_init()
        }
    }

    pub fn teardown(state: *mut *mut jq_state) {
        unsafe {
            super::ffi_jq::jq_teardown(state)
        }
    }

    pub fn compile_args(state:  *mut jq_state, expression: String) {
        unsafe {
            let arg2 = CString::new(expression).unwrap();
            let mut ptr = super::ffi_jq::jq_compile_args(
                state,
                arg2.as_ptr(),
                ffi_jq::jv_array());
        }
    }
}

#[cfg(test)]
mod test {
    use super::jq;
    
    #[test]
    fn it_works() {
        println!(".it works.");

        let mut jq_state = jq::init();
        println!("jq_state: {:?}", jq_state);

        let mut ptr = jq::compile_args(jq_state, ".".to_string());
        println!("ptr: {:?}", ptr);

        jq::teardown(&mut jq_state);

        assert!(false);
    }
}


