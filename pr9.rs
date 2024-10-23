fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    for i in (5..).step_by(6).take_while(|i| i * i <= n) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let число = 29;
    if is_prime(число) {
        println!("Число {} є простим.", число);
    } else {
        println!("Число {} не є простим.", число);
    }
}