pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut primes = vec![2];
    let mut i = 3;
    while primes.len() <= n as usize {
        if is_prime(i) {
            primes.push(i);
        }
        i += 2;
    }
    primes[n as usize]
}

pub fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
