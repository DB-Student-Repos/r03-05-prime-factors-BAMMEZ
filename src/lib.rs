pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut factors = Vec::new();
    let mut divisor = 2;

    while num > 1 {
        while num % divisor == 0 {
            factors.push(divisor);
            num /= divisor;
        }
        divisor += 1;
        // If divisor reaches the square root of num and num is still greater than 1,
        // then num must be a prime number greater than the divisor and should be added to the factors
        if divisor * divisor > num && num > 1 {
            factors.push(num);
            break;
        }
    }

    factors
}


