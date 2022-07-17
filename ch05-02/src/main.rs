
#[derive(Debug)]
struct Vec2ii(i32, i32);

#[derive(Debug)]
struct Circle {
    center: Vec2ii,
    radius: u32,
}

impl Circle {
    fn area(&self) -> f32 {
        (self.radius*self.radius) as f32 *3.1415926535898f32
    }
}

fn main() {
    let vec1 = Vec2ii(5, 1);
    let vec2 = Vec2ii(-2, 10);

    println!("First vector of the parallelogram: {:?}", vec1);
    println!("Second vector of the parallelogram: {:?}", vec2);
    println!("Area of the parallelogram: {}\n", area(&vec1, &vec2));


    let ccl1 = Circle{
        center:vec1, 
        radius:5
    };
    // note after here vec1 is no longer available

    println!("circle: {:#?}", ccl1);
    println!("Area of the circle: {}", ccl1.area());

    
}

fn area(veca: &Vec2ii, vecb: &Vec2ii) -> i32 {
    veca.0 * vecb.1 - veca.1 * vecb.0
}
