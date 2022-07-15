fn main() {
    let mut s = String::from("Hello, slice");

    let hello = &mut s[..5];

    println!("hello = {hello}");

    let world = &mut s[7..];
    println!("world = {world}");

    // let full = &s[..];
    // println!("full = {full}");
}
