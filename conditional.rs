use std::fmt::{Debug, Display};
fn main() {
    //The static function new used by any instance
    //Instance of this can't call display method
    let constructor1 = ConditionalApi::new(vec![1, 3, 4], vec![3, 45, 67]);
    constructor1.debug();
    //constructor1.display();
    //Both debug and diplay can be called since &str implements both
    let constructor2 = ConditionalApi::new("First", "Second");
    constructor2.debug();
    constructor2.display();
}
struct ConditionalApi<T> {
    x: T,
    y: T,
}
impl<T> ConditionalApi<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Debug> ConditionalApi<T> {
    fn debug(&self) {
        println!("Field x : {:?}\n Field y:{:?}", self.x, self.y)
    }
}
impl<T: Display> ConditionalApi<T> {
    fn display(&self) {
        println!("Field x: {}\n Field y: {}", self.x, self.y);
    }
}
