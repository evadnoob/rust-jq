pub mod jq {

    pub fn x() {
        println!("in x");
    }
}



#[cfg(test)]
mod test {
    use super::jq::x;
   
    #[test]
    fn it_works() {
        println!(".it works.");

        x();

        assert!(false);
        
    }
}


