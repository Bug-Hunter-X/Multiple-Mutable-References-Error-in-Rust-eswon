fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modifying x through y
    } // y goes out of scope here
    {
        let z = &mut x; // z is a mutable reference to x
        *z = 100; // Modifying x through z
    } // z goes out of scope here
    println!("x = {}", x); // This will print 100
} 