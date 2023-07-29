use std::sync::{Arc, Mutex};
use std::thread::spawn;
fn main() {
    let shared_data = vec![67, 54, 98];
    let thread_safe_instance = Arc::new(Mutex::new(shared_data));

    spawn(move || {
        let mut vector = thread_safe_instance.lock().unwrap();
        vector.push(768);
        println!("{:?}", vector);
    })
    .join()
    .unwrap();

//we are using the moved data inside another thread
    let clone = thread_safe_instance.clone();
    spawn(move || {
        let mut vector = clone.lock().unwrap();
        vector.push(89);
    })
    .join()
    .unwrap();
}
