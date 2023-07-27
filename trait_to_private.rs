use new::{Slice, Username};
fn main() {}
impl Slice for i32 {
    fn cant_be_impl() {}
}

mod new {
    #[derive(Debug)]
    pub struct Username(String);
    trait Sealed {}

    pub trait Slice: Sealed {
        fn cant_be_impl();
    }
    //Some of the internal types can implement
    //this trait but outside can't
}
