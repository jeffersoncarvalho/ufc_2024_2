use crate::garden::PI;
use crate::garden::Tree;
use crate::garden::vegetables::Asparagus;

pub mod garden {

    #[derive(Debug)]
    pub struct Tree {}
    pub const PI:f32 = 3.14;

    pub mod vegetables {
        #[derive(Debug)]
        pub struct Asparagus {}
    }
}

fn main() {
    let plant = Asparagus {};
    let tree = Tree {};
    println!("I'm growing {tree:?}!");
    println!("I'm growing {plant:?}!");
    println!("PI : {PI}");

}
