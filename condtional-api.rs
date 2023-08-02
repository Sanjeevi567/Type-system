fn main() {
    let copy = Unit::<i32>(PhantomData);
    let clone = Unit::<String>(PhantomData);
    let copy_clone = Unit::<f64>(PhantomData);

    //i32 is copy and clone
    copy.copy();
    copy.clone();
    copy.both();

    //String is Clone not Copy
    clone.clone();

    //f64 is copy and clone
    copy_clone.copy();
    copy_clone.clone();
    copy_clone.both();
    //the concrete type is f64
    copy_clone.to_bytes();
}
use std::marker::PhantomData;
#[derive(Debug)]
struct Unit<T>(PhantomData<T>);

//Called on both copy and clone types
impl<T: Copy> Unit<T> {
    fn copy(&self) {
        println!("Copy method")
    }
}
//Can't be called on Copy types
impl<T: Clone> Unit<T> {
    fn clone(&self) {
        println!("Clone method")
    }
}
impl<T: Copy + Clone> Unit<T> {
    fn both(&self) {
        println!("Both Copy and Clone method")
    }
}
//only called if the type is f4
impl Unit<f64> {
    fn to_bytes(&self) {
        println!("Method that only called when the type is f64")
    }
}
