use garden::vegetables::Asparagus;
use garden::fruits::apples;
use garden as my_garden;
use room as my_room;

pub mod garden;
pub mod kitchen;

pub mod room {
    pub fn who_am_i(){
        println!("I am the room!");
    }
    pub fn who_am_i_again(){
        println!("I am the room, again!");
    }
}

fn main() {
   let plant = Asparagus {};
   println!("I am a growing {plant:?}");
   my_room::who_am_i();
   my_room::who_am_i_again();
   println!("Name: {}",apples::NAME);
   my_garden::public_function();
}
