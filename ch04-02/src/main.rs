fn main() {
    let mut s = String::from("Hello");
    
    
    let s_1 = &s;
    println!("s = {s_1}");
    // s_1 is dropped afterwards
    
    s.push_str(", References!");
    // so the mutable borrow happen here is fine

    println!("s = {s}"); 
}
