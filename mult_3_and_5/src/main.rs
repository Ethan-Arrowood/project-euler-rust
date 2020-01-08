fn main() {
    let t = sum_divisible_by(3) + sum_divisible_by(5) - sum_divisible_by(15);
    println!("The sum of the multiples of 3 or 5 below 1000 is: {}", t);
}

fn sum_divisible_by(n: i32) -> i32 {
    let p = 1000 / n;
    return n * ( p * ( p + 1 ) ) / 2;
}