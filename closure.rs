fn main() {  

  let mut str = String::from("Hello");
  
  //let iter = vec![1,2,4].iter_mut().map(|a|{str;*a+*a});
  // movee(||{str;});
  //movee(||{str;});
  //println!("{str}");
  
    let count = 0;
    let print_count_closure: i32 = (|| {return count;})();
    //Note there is no parantheses around the function name otherwise
    //the type is just i32
    let fn_pointer:fn(i32)->i32 = move1;
    

}
fn move1(i:i32)->i32{78}
//We can only used with Clone types i.e we can't pass i32 and similar copy primitives
fn movee<T,F:Fn(i32) + Clone>(x:F)->i32{
//closure is also a type that implements clone
//This is bit confusing than other types
//Where cloning is matter
  //  x.clone()();
//    x();
       67
}

