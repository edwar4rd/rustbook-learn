fn main() {
    let vec1 = (5, 1);
    let vec2 = (-2, 10);
    
    println!("Area of the parallelogram: {}", area(vec1, vec2));
}

fn area(veca: (i32, i32), vecb: (i32, i32)) -> i32{
    veca.0*vecb.1-veca.1*vecb.0
}