const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (missles, ready): (i32, i32) = (STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {} of my {} missles...", ready, missles);
    println!("{} missles left", missles - ready);
}
