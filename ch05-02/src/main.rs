struct Vec2ii(i32, i32);

fn main() {
    let vec1 = Vec2ii(5, 1);
    let vec2 = Vec2ii(-2, 10);

    println!("Area of the parallelogram: {}", area(vec1, vec2));
}

fn area(veca: Vec2ii, vecb: Vec2ii) -> i32 {
    veca.0 * vecb.1 - veca.1 * vecb.0
}
