use miller_rabin;

pub trait IsPrime {
    fn is_prime(&self) -> bool;
}

impl IsPrime for u64 {
    fn is_prime(&self) -> bool {
        miller_rabin::is_prime(self, 50)
    }
}

impl IsPrime for i64 {
    fn is_prime(&self) -> bool {
        miller_rabin::is_prime(&(*self as u64) , 50)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use rug::Integer;
    use crate::IsPrime;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn rug() {
        let int1 = Integer::from(10);
        let int2 = Integer::from(10);
        assert_eq!(int1.cmp(&int2), Ordering::Equal);
    }

    #[test]
    fn not_prime_1() {
        assert!(!1u64.is_prime());
    }

    #[test]
    fn prime_2() {
        assert!(2u64.is_prime());
    }

    #[test]
    fn big_prime() {
        assert!(18446744073709551557u64.is_prime());
    }

    #[test]
    fn gmp_prime() {
        let s1 = "531137992816767098689588206552468627329593117727031\
        9231994441382004035598608522427391625022652292856688893294862465010153465793376527072394095\
        19978766587351943831270835393219031728127";
        let i1 = s1.parse::<Integer>().unwrap();
        assert_eq!(i1.is_probably_prime(50), rug::integer::IsPrime::Probably);
    }
}
