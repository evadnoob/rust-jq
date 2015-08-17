extern crate libc;

mod ffi_jv;
mod ffi_jq;


pub fn jq_init() -> *mut ffi_jq::Struct_jq_state  {
    unsafe {
        let ptr = ffi_jq::jq_init();
        ptr
    }
}


#[cfg(test)]
mod test {
    use super::jq_init;
    
    #[test]
    fn it_works() {
        println!(".it works.");

        let jq_state = jq_init();
        println!("jq_state: {:?}", jq_state);

        assert!(false);
        
    }
}


