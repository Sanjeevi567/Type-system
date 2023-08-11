use std::fmt::Debug;
fn main(){
    let mut iter = vec![1,2,4];
    
    //the item type is depend on the type of 
    //Iterator 
    accept(iter.iter_mut(),&mut 23);
    //But also Item must be debuggable and Eq
    //So we can't pass float value
    accept(iter.iter(),&23);
    
    accept(iter.into_iter(),34);
    
}
fn accept<I>(x:I,item:I::Item) where I:Iterator , I::Item : Eq + Debug{
}
