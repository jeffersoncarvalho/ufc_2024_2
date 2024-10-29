#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    version_3();
}

fn version_3() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(scale * 20),
        height: 50,
    };
    dbg!(&rect1);
}

/*fn version_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    //println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
}*/

/*fn version_1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );
}*/

/*fn version_0() {

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1,height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}*/

/*fn area_struct(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}*/
