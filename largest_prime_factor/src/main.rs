fn main() {
    let mut l: u64 = 600851475143;
    let mut largest_prime = 1;
    let mut memoized_primes = vec![2];
    while l > 1 {
        for (_, &i) in memoized_primes.iter().enumerate() {
            if l % i == 0 && i > largest_prime {
                largest_prime = i;
                l = l / i;
            }
        }
        memoized_primes.push(
            find_next_prime(
                *memoized_primes.last().unwrap()
            )
        );
    }
    println!("Largest Prime: {}", largest_prime);
}

fn find_next_prime(mut n: u64) -> u64 {
    loop {
        n+=1;
        if is_prime(n) {
            return n;
        }
    }
}

fn is_prime(n: u64) -> bool {
    if n == 1 || n == 2 {
        return true;
    } else {
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}
