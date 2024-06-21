fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6; //this will cause an error
    println!("The value of x is {x}");

    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

    let y = 5; 

    let y = y + 1; 

    {
        let y = y * 2; 
        println!("The value of y in the scope is {y}");
    }

    println!("The value of y outside the scope is {y}");

    // allowed via shadowing
    let spaces = "     "; 
    let spaces = spaces.length(); 

    // let mut spaces = "      "; 
    // spaces = spaces.length(); <- this will panic because we're trying to mutate a variable's
    // type. 
}
