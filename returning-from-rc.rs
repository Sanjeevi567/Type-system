fn iterate<'a,T:Copy + Clone >(data: &'a Rc<Vec<T>>) -> impl Iterator<Item = T> + 'a {
    data.iter().map(|item|*item)
}
use std::rc::Rc;
fn main() {
      let rc =  Rc::new(vec![34u32,56,89]);
      iterate(&rcc);
}
