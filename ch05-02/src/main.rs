fn main() {
    let vec1x = 5;
    let vec1y = 1;
    let vec2x = -2;
    let vec2y = 10;
    
    println!("Area of the parallelogram: {}", area(vec1x, vec1y, vec2x, vec2y));
}

fn area(vecax: i32, vecay: i32, vecbx: i32, vecby: i32) -> i32{
    vecax*vecby-vecay*vecbx
}