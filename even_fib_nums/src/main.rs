fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut c = 2;
    let mut sum = 0;
    let l = 4000000;
    while c < l {
      sum += c;
      a = c + b;
      b = c + a;
      c = a + b;
      // println!("{}", c);
    }
    println!("Sum of the even-valued Fibonacci numbers less than 4,000,000 is {}", sum);
  }