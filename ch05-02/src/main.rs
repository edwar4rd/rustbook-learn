#[derive(Debug)]
struct Vec(f32, f32);

impl Vec {
    fn dist(veca: &Vec, vecb: &Vec)->f32 {
        ((veca.0-vecb.0)*(veca.0-vecb.0)+(veca.1-vecb.1)*(veca.1-vecb.1)).sqrt()
    }
}


#[derive(Debug)]
struct Circle {
    center: Vec,
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        (self.radius*self.radius) as f32 *3.1415926535898f32
    }

    fn short_dist(&self, point: &Vec) -> f32{
        Vec::dist(&self.center, &point)-self.radius
    }
}

fn main() {
    let vec1 = Vec(5.0, 1.0);
    let vec2 = Vec(-2.0, 10.0);

    println!("First vector of the parallelogram: {:?}", vec1);
    println!("Second vector of the parallelogram: {:?}", vec2);
    println!("Area of the parallelogram: {}\n", area(&vec1, &vec2));


    let ccl1 = Circle{
        center:vec1, 
        radius:5.0
    };
    // note after here vec1 is no longer available

    println!("circle: {:#?}", ccl1);
    println!("Area of the circle: {}", ccl1.area());
    println!("Distance from vec2 to circle: {}", ccl1.short_dist(&vec2));
    
}

fn area(veca: &Vec, vecb: &Vec) -> f32 {
    veca.0 * vecb.1 - veca.1 * vecb.0
}
