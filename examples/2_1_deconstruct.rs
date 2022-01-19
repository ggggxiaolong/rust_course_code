fn main() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {a}, b = {b}");
    b = true;
    assert_eq!(a, b);
}