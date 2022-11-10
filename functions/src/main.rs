fn main() {
    println!("Hello, world!");

    another_function();

    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");    

    if x != 5 {
        println!("true shit");
    } else {
        println!("false shit");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    
    wtf();
    println!("-----");
    vfromloop();
    println!("-----");
    let a = [10, 20, 30, 40, 50];
    let b = [100;5];
    println!("length: {}",  a.len());
    let mut n = 0;
    let mut idx = 0;
    while idx < a.len() {
        println!("value: {}", a[idx]);
        n += b[idx];
        idx += 1;
    }
    println!("the n: {n}, {b:?}");
    println!("-----");
    println!("length of strings? {}", "length of strings? {}".len());
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    6
}

fn wtf() {
    let x = true;
    let y = if x {};
    println!("wtf is y? {y:?}");
}

fn vfromloop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}