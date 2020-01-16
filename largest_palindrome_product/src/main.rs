fn main() {
    println!("Largest Palindrom Product");
    let mut l = 0;
    let mut a = 999;
    while a >= 100 {
        let mut b = 999;
        while b >= a {
            let p = a * b;
            if p <= l {
                break;
            } else if is_palindrome(p) {
                l = p;
            }
            b = b - 1;
        }
        a = a - 1;
    }
    println!("{}", l);
}

fn is_palindrome(n: i32) -> bool {
    n == reverse(n)
}

fn reverse(mut n: i32) -> i32 {
    let mut rev = 0;
    while n > 0 {
        rev = 10 * rev + n % 10;
        n = n / 10;
    }
    return rev;
}