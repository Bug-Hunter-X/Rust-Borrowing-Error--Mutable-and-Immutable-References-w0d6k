fn main() {
    let mut x = 5;
    {
        let y = &mut x; // Mutable reference
        *y += 1; // Modify x using y
    } 
    let z = &x; // Immutable reference, now safe after mutable borrow ends
    println!("x = {}", x); // Prints 6
    println!("z = {}", *z); // Prints 6
} 