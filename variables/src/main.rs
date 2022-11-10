const Y: u32 = 33;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}, {Y}, {{z}}");

    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let mut n = 1_000;
    n += 1;
    println!("The value of n is: {n}");
 
    // let mut o: u8 = 0;
    // o -=1;
    // println!("The value of o is: {o}");

    let a = [1; 5];
    println!("The first a: {}",a[0]);
}
