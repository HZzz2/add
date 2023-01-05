use add_one;
use add_two::add_two;
use add_three::add_three;
fn main() {
    println!("{}",add_one::add_one(8));
    println!("{}",add_two(66));
    println!("{}",add_three(82));
}
