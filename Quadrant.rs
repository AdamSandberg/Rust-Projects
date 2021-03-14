use std::io;
//This was used to place a set of cooardinates on a graph.

fn main() {
    let mut x = String::new();
    let mut y = String::new();

println!("Type a number you would like placed in a qaudrant.");

io::stdin().read_line(&mut x);
io::stdin().read_line(&mut y);

println!("the value of x is {}", x);
println!("the value of y is {}", y);

let x: i16 = x.trim().parse().expect("");
let y: i16 = y.trim().parse().expect("");

if x > 0 && y > 0 {
    println!("Quadrant 1");
}
else if x < 0 && y > 0{
    println!("Quadrant 2");
}
else if x < 0 && y < 0{
    println!("Quadrant 3");
}
else {
    println!("Quadrant 4");
}

}
