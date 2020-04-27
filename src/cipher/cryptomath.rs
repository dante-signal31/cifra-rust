/// Cryptomatrh module.
/// Rust port of code at:
/// https://www.nostarch.com/crackingcodes/ (BSD Licensed)

/// Return the GCD of a and b using Euclid's algorithm.
///
/// # Parameters:
/// * a: First integer.
/// * b: Second integer.
///
/// # Returns:
/// * The Greatest Common Divisor between two given numbers.
pub fn gcd(mut a: isize, mut b: isize)-> isize {
    while a != 0 {
        let previous_a = a;
        a = b % a;
        b = previous_a;
    }
    b
}

/// Return the modular inverse of a % m.
///
/// Modular inverse is the number x such that a*x % m = 1.
///
/// # Parameters:
/// * a: First integer.
/// * m: Second integer.
///
/// # Returns:
/// * Module inverse integer.
pub fn find_mod_inverse(a: isize, m: isize)-> Option<isize> {
    return if gcd(a, m) != 1 {
        None
    } else {
        let (mut u1, mut u2, mut u3) = (1, 0, a);
        let (mut v1, mut v2, mut v3) = (0, 1, m);
        while v3 != 0 {
            let q = u3 / v3;
            let new_u1 = v1;
            let new_u2 = v2;
            let new_u3 = v3;
            let new_v1 = u1 - q * v1;
            let new_v2 = u2 - q * v2;
            let new_v3 = u3 - q * v3;
            v1 = new_v1;
            v2 = new_v2;
            v3 = new_v3;
            u1 = new_u1;
            u2 = new_u2;
            u3 = new_u3;
        }
        // Whereas Rust % operator behaves as a remainder operator Python's behaves like a modulus.
        // While two operator get similar results for positive operands they differ when any of them
        // is negative.
        // Modular inverse uses a true modulus operation, so a workaround for Rust is needed. I use
        // the one given in this thread:
        // https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
        // let result = ((u1 % m) + m) % m;
        let result = modulus(u1, m);
        Some(result)
    }
}

/// Get true modulus of a % b
///
/// Whereas Rust % operator behaves as a remainder operator Python's one behaves like a modulus.
/// While two operator get similar results for positive operands they differ when any of them
/// is negative.
/// Modular inverse uses a true modulus operation, so a workaround for Rust is needed. I use
/// the one given in this thread:
/// https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
///
/// # Parameters:
/// * a: Left hand operand.
/// * b: Right han operator.
///
/// # Returns:
/// * Modulus result of two operands.
pub fn modulus(a: isize, b: isize)-> isize{
    ((a % b) + b) % b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gcd() {
        let recovered_gcd = gcd(24,32);
        assert_eq!(recovered_gcd, 8)
    }

    #[test]
    fn test_find_mod_inverse() {
        let recovered_mod_inverse = find_mod_inverse(7, 26).unwrap();
        assert_eq!(recovered_mod_inverse, 15)
    }
}