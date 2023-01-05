use add_one;  //clippy提示这条语句是多余的
use add_three::add_three;
use add_two::add_two;
fn main() {
    println!("{}", add_one::add_one(8));
    println!("{}", add_two(66));
    println!("{}", add_three(82));
}
