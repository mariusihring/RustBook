fn main() {
    let x = 5;
    let x = x + 1;

    {
        let tup = (500, 420, 69);
        let (x, y, z) = tup;
        println!("x: {x} y: {y}, z: {z}");
        let first_entry = tup.0;
        println!("x is: {first_entry}");
    }

    println!("the value of x is: {x}");
}
