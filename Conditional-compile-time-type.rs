fn main() {
    use either::{Left, Right};
    use std::{
        any::Any,
        iter::{empty, once, Once},
    };

    let empty_or_once: Box<dyn Any> = if 3 < 2 {
        Box::new(empty::<()>())
    } else {
        Box::new(once("Hello"))
    };
    if let Ok(once_str) = empty_or_once.downcast::<Once<&str>>() {
        println!("{:?}", once_str);
    }

    let empty_or_once = if 1 < 2 {
        Left(empty::<()>())
    } else {
        Right(once("Hello"))
    };

    println!("{:?}", empty_or_once);
}
