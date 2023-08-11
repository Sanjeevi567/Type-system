fn main() {
    let mut udt_str = Udt::new();
    udt_str.len();
    //Result's in error
    println!("{:?}", udt_str.push_str("Adding More than fiteen Elements"));
    println!("{:?} {}", udt_str, udt_str.capacity());
    //but we can bypass the inherent push_str with deref operator
    //only if DerefMut is implemented
    (*udt_str).push_str("Adding More than fiteen Elements");
    println!("{:?} {}", udt_str, udt_str.capacity());

    //but different from String
    //this is not compile
    // let add = udt_str + "str_slice";
}
use std::ops::{Deref, DerefMut};
#[derive(Debug)]
struct Udt(String);
impl Udt {
    fn new() -> Self {
        //The type is inferred
        Self("".into())
    }
    //We have a constrain that we can't push more than
    //a 15 characters at a time
    fn push_str(&mut self, str: &str) -> Result<(), &'static str> {
        if str.len() > 15 {
            Err("The length must be less than or equal to fifteen")
        } else {
            self.0.reserve(20);
            self.0.push_str(str);
            Ok(())
        }
    }
    //If name collision happen as in this case then
    //rust first call the methods defined on it's impl
    //block
}

impl Deref for Udt {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl DerefMut for Udt {
    //since Deref is super trait we can use Target of Deref
    //or returning &mut String directly but must the Target
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
