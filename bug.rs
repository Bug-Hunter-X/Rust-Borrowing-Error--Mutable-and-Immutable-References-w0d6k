fn main() {
    let mut x = 5;
    let y = &mut x; // Mutable reference
    let z = &x; // Immutable reference

    *y += 1; // Modifying x through y is fine
    println!("x = {}", x); // x is 6

    // This will not compile because z is an immutable reference to x, which has already been mutably borrowed
    //println!("z = {}", *z);
}