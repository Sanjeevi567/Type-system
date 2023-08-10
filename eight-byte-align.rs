fn main(){
   let align = AlignedEight::new(8);
   let align = AlignedEight::new(8f64);
   //This also panic at compile time
   println!("{:?}",<u8 as Verify>::IS_OKAY);
}
struct AlignedEight<A>{
    x : A,
}
impl<A> AlignedEight<A>{
   fn new(x:A)->Self{
    let is_okay = <A as Verify>::IS_OKAY;
       Self{
           x 
       }
   }    
}

trait Verify{
    const IS_OKAY:();
}
use std::mem::align_of;
impl<T> Verify for T{
    const IS_OKAY :() = {
        if align_of::<T>() % 8 != 0{
            panic!("The type is not 8 byte aligned");
        } 
    };
}
