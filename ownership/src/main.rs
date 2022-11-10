fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = s1.clone();
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    show_calc_len();
    println!("-----");
    cal_len_w_ref();

    let mut s11 = String::from("hello");
    change(&mut s11);
    println!("s11: {:?}", s11.as_bytes());
    println!("s11: {s11}");
    println!("-----");
    let mut n = 1;
    incr(&mut n);
    println!("n: {n}");
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
      );
}

fn incr(n: &mut i32) {
    *n += 1;  // dereference n to work with it's value
}
  
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn show_calc_len() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// -----
fn cal_len_w_ref() {
    let s1 = String::from("hello");

    let len = calculate_length_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}