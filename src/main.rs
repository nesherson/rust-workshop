fn main() {
    let mut s1 = String::from("Hello, world!");

    take_ownership(&mut s1);

    println!("After take_ownership {s1}");

}

fn take_ownership(s: &mut String) {
    s.push_str("test 123!!!");
}
