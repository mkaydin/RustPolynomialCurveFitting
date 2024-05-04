use rand::Rng;
use rand::rngs::ThreadRng;

pub fn vector() -> (Vec<i32>, Vec<i32>) {
    // create a vector x with random values between 0 and 200
    let mut x: Vec<i32> = vec![];
    let mut rng: ThreadRng = rand::thread_rng();
    for _ in 0..1000 {
        let value: i32 = rng.gen_range(0..=20);
        x.push(value);
    }

    // create a vector y with random values between 0 and 200
    let mut y: Vec<i32> = vec![];
    for _ in 0..1000 {
        let value: i32 = rng.gen_range(-20..=0);
        y.push(value);
    }

    // print the vectors x and y
    println!("Vector x: {:?}", x);
    println!("Vector y: {:?}", y);

    (x,y)
}