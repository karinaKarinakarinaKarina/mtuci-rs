

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: u32) -> u32 {
    num * num
}
