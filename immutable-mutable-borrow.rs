fn main(){
        //Single threaded

    //A type of T
    let owner = true;
    let srd_brw1: &bool = &owner;

    //A type of &T
    //shared borrow is alaised i.e both point to owner
    let srd_brw2 = &srd_brw1;
    let srd_brw3 = &owner;
    // multiple reads are possible
    println!("{srd_brw1} {srd_brw2} {srd_brw3}");

    //For a copy type this will create independent copy implicitly,
    //but for move types this will move the owner to new_owner and
    //Invalidate the references of the old owner thus can't be after this assignment
    let new_owner = owner;

    //without mut infront of variable we can't get mutable reference ,it's called inherent mutability
    let mut owner = true;

    //A type of &mut T
    let uqe_brw1: &mut bool = &mut owner;
    //uqe_brw1 moved to uqe_brw2
    let uqe_brw2 = uqe_brw1;
    //Throw compile error.
    //only one mutable borrow exist

    //*uqe_brw1 =false;

    //We can't access the previous mutable borrow after the
    //another mutable borrow

    //println!("{}",uqe_brw1);

    //re borrowing is allowed

    //Getting mutable reference from another mutable reference
    let uqe_brw3 = &mut *uqe_brw2;
    *uqe_brw3 = false;
    //Non lexical lifetime(NLL) would allow this code
    //but previous old Rust version won't allow
    println!("{uqe_brw2}");
}
