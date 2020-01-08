fn main() {
    let mut x = 1;
    let mut y = 2;
    let mut t = 0;
    while y < 4000000 {
      if y % 2 == 0 {
        t += y;
      }
      y += x;
      x = y - x;
    }
    println!("Sum of the even-valued Fibonacci numbers less than 4,000,000 is {}", t);
  }
