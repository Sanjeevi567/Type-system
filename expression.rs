fn main() {
    //Empty block returns ()
    let block_expr = {};
    println!("{:?}", block_expr);
    let a = 10;
    let b = { a + 1 };
    let hash = {
        //Read some file form disk then store them in hash map
        // and then returns it
        //let mut hashmap = HashMap::new();
        //hashmap
    };

    //Here the return type is optional
    fn expr(x: bool) -> () {
        if x {
            //Note there is no semicoln here
            println!("This variant is Bool::True")
        } else {
            println!("This variant is Bool::False")
        }
    }

    //we can use external variable
    let a = 10;
    //We can put any expression that evaluates to bool
    expr({
        //Also local variable
        let b = 15;
        a < b
    });

    expr(1 > 6);
    use self::Bool::{False, True};
    //Because of PartialEq
    expr(True == False);
    //Because of PartialOrd
    expr(False > True);
    
    enum Empty{
       Nonne,
       Nonejk,
       Noned,
       jhk,
    }
    println!("{}",std::mem::size_of::<Empty>());
}

//Field ordering from zero if not specified
#[derive(PartialEq, PartialOrd)]
enum Bool {
    True,
    False,
}
