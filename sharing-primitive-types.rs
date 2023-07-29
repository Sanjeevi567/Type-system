use std::collections::HashMap;
use std::thread::spawn;
fn main() {
    let Integer: u64 = 4560;
    let mut string: String = String::from("Not thread safe");
    let mut hashmap: HashMap<&str, Vec<i32>> = HashMap::new();
    hashmap.insert("Key", vec![90, 456]);
    spawn(|| {
        //We can't reference a variable outside of closure
        println!("{}", Integer);
        //nor mutate them inside closure
        string.push_str("won't compile");
    });
    for i in 0..10 {
        //try to add 1 to loop local variable in
        //different threads
        spawn(|| i + 1);
    }

    //Mutatating string/hashmap in different threads
    //without any synchronization
    spawn(|| {
        string.push_str("Some thread");
        hashmap.insert("Spawned", vec![45, 23]);
    });

    //Reading concurrently
    spawn(|| {
        println!("String :{}\n HashMap: {:?}", string, hashmap);
    });
}
