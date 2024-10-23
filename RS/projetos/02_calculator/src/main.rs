fn main() {
    sum(10,5);
    sub(10,5);
    mul(10,5);
    div(10,3);
}

fn sum(x: i32, y: i32) {
    println!("The sum of {} and {} is {}.",x,y,x+y);
}

fn sub(x: i32, y: i32) {
    println!("The subtraction of {} and {} is {}.",x,y,x-y);
}

fn mul(x: i32, y: i32) {
    println!("The multiplation of {} and {} is {}.",x,y,x*y);
}

fn div(x: i32, y: i32) {
    let res: f32 = (x as f32)/(y as f32);
    println!("The division of {} and {} is {:.3}.",x,y,res);
}