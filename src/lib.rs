extern crate libc;

mod ffi_jq;
mod ffi_jv;

// TODO: remove the lazy unwraps, replace with proper error handling.

mod jq {

    use super::ffi_jq::jq_state;
    use super::ffi_jq;
    use std::ffi::CString;
    use libc;
    
    pub fn init() -> *mut jq_state {
        unsafe {
            ffi_jq::jq_init()
        }
    }

    pub fn teardown(state: *mut *mut jq_state) {
        unsafe {
            ffi_jq::jq_teardown(state)
        }
    }

    pub fn compile_args(state: *mut jq_state, expression: String) {
        unsafe {
            let expression_cstr = CString::new(expression).unwrap();
            let mut ptr = ffi_jq::jq_compile_args(
                state,
                expression_cstr.as_ptr(),
                ffi_jq::jv_array());
            // ?? free expression_cstr ??
        }
    }

    pub fn start(state: *mut jq_state, input: String) {
        unsafe {
            let input_ptr = CString::new(input).unwrap();
            let mut ptr = ffi_jq::jv_parse(input_ptr.as_ptr());
        
            // void jq_start(jq_state *jq, jv input, int flags) {
            //ffi_jq::jq_start(state, ptr, ffi_jq::JV_PRINT_PRETTY as libc::c_uint)  
            ffi_jq::jq_start(state, ptr, 0)  
        }
    }

    pub fn next(state: *mut jq_state) {
        unsafe {
            ffi_jq::jq_next(state);
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

        let mut started = jq::start(jq_state, "{}".to_string()); 
        println!("started {:?}", started);

        let mut next = jq::next(jq_state); 
        println!("next {:?}", next);


        jq::teardown(&mut jq_state);

        assert!(false);
    }
}


