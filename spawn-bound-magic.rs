use std::thread::spawn;
fn main() {
    //Inside the closure the m is independent of value
    let copy_type = 10;

    //Even though m cloned inside the closure ,
    //But shorter_lifetime depend on the variable copy_type
    //Rust compiler not that smart enough to make short_liftime is tied to clone of copy_type inside of closure
    //So this is compile error but the error not say anything about this variable.
    //The code will compile if any reference doesn't depend on the above i32 value i.e in this case this &i32
    let short_lifetime: &i32 = &copy_type;

    //This reference has static lifetime
    let static_ref: &'static i32 = &89;

    let owned_data = String::from("Owned data are Moved");

    spawn(move || {
        println!(
            "We are in spawned thread: {copy_type}\n{static_ref}\n{short_lifetime}\n{owned_data}"
        );
    })
    .join();

    //Copy types are cloned when Moved
    //So we can use after the closure
    println!("We are in main thread: {copy_type}\n{static_ref} ");
    //But Move types are moved inside the closure
    //Thus we can't use here once passed to closure
    //println!("{owned_data}");
}
