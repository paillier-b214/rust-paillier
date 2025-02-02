#![cfg(feature = "inclgmp")]

extern crate gmp;

use self::gmp::mpz::Mpz;
use super::traits::*;
use rand::rngs::OsRng;
use rand::RngCore;

impl Samplable for Mpz {
    fn sample_below(upper: &Self) -> Self {
        let bits = upper.bit_length();
        loop {
            let n = Self::sample(bits);
            if n < *upper {
                return n;
            }
        }
    }

    fn sample(bitsize: usize) -> Self {
        let mut rng = OsRng::default();
        let bytes = (bitsize - 1) / 8 + 1;
        let mut buf: Vec<u8> = vec![0; bytes];
        rng.fill_bytes(&mut buf);
        Self::from(&*buf) >> (bytes * 8 - bitsize)
    }

    fn sample_range(lower: &Self, upper: &Self) -> Self {
        lower + Self::sample_below(&(upper - lower))
    }
}

impl NumberTests for Mpz {
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
    fn is_even(&self) -> bool {
        self.is_multiple_of(&Mpz::from(2))
    }
    fn is_negative(&self) -> bool {
        self < &Mpz::from(0)
    }
}

pub use num_traits::{One, Zero};

impl ModPow for Mpz {
    fn modpow(base: &Self, exponent: &Self, modulus: &Self) -> Self {
        base.powm(exponent, modulus)
    }
}

impl ModInv for Mpz {
    fn modinv(a: &Self, modulus: &Self) -> Self {
        a.invert(modulus).unwrap()
    }
}

impl EGCD for Mpz {
    fn egcd(a: &Self, b: &Self) -> (Self, Self, Self) {
        a.gcdext(b)
    }
}

impl ConvertFrom<Mpz> for u64 {
    fn _from(x: &Mpz) -> u64 {
        let foo: Option<u64> = x.into();
        foo.unwrap()
    }
}

impl BitManipulation for Mpz {
    fn set_bit(self: &mut Self, bit: usize, bit_val: bool) {
        if bit_val {
            self.setbit(bit);
        } else {
            self.clrbit(bit);
        }
    }
}

pub type BigInteger = Mpz;
