fn main() {
}
use std::ops::{Add,Mul};
//silly example
fn bounds<'a,A,B,C,T:'a>(x:A,y:B,c:C,d:T) ->impl FnOnce()->T
where A : Add<Output=A> + Copy,
      B : Mul<Output=B> + Clone ,
      C : Eq + Ord + PartialOrd,
{
  //Without the Add bound we can't use + or add method
    let add = x + x;
  //We can't multiply same move type since they moved to left operand 
    let mul = y.clone() * y;
    //We can use methods or operators like == ,<=,>
    println!("{} {}",c==c, c < c);
    //last expression
    || d
}
