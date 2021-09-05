use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg, Sub};
use std::{cmp, iter};

#[derive(Debug)]
pub struct Decimal {
    // reverse order, last one can't be zero, 123 = vec![3,2,1],  0.123 = vec![3,2,1]
    digits: Vec<u8>,
    // floating digits count
    floating_length: usize,
    // is negative
    is_negative: bool,
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits
            && self.floating_length == other.floating_length
            && self.is_negative == other.is_negative
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (
            self.is_negative,
            self.integer_length(),
            other.is_negative,
            other.integer_length(),
        ) {
            (false, _, true, _) => Some(Ordering::Greater),
            (true, _, false, _) => Some(Ordering::Less),
            (_, s, _, o) => {
                let result = if s == o {
                    self.digits.cmp(&other.digits)
                } else {
                    s.cmp(&o)
                };
                if self.is_negative {
                    Some(result.reverse())
                } else {
                    Some(result)
                }
            }
        }
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.is_negative, rhs.is_negative) {
            (true, true) | (false, false) => {
                let (a, b) = Decimal::align_digit_length(&self, &rhs);
                Self {
                    digits: Decimal::equal_length_add(&a, &b),
                    floating_length: cmp::max(self.floating_length, rhs.floating_length),
                    is_negative: self.is_negative,
                }
                    .remove_leading_zeros()
                    .remove_floating_trailing_zeros()
            }
            (false, true) => self - (-rhs),
            (true, false) => (rhs) - (-self),
        }
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(mut self) -> Self::Output {
        self.is_negative = !self.is_negative;
        self
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        let floating_length = self.floating_length + rhs.floating_length;
        let is_negative = self.is_negative != rhs.is_negative;
        let digits = rhs
            .digits
            .into_iter()
            .rev()
            .zip(0..)
            .fold(vec![0], |v, (d, index)| {
                let r = Decimal::one_digit_mul(&self.digits, d, index);
                Decimal::none_equal_length_add(&v, &r)
            });
        Self {
            digits,
            floating_length,
            is_negative,
        }
            .remove_leading_zeros()
            .remove_floating_trailing_zeros()
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.is_negative, rhs.is_negative) {
            (_, true) => self + (-rhs),
            (true, false) => -(-self + rhs),
            (false, false) => {
                let (a, b) = Decimal::align_digit_length(&self, &rhs);
                let result = if self > rhs {
                    Self {
                        digits: Decimal::equal_length_sub(&a, &b),
                        floating_length: cmp::max(self.floating_length, rhs.floating_length),
                        is_negative: false,
                    }
                } else {
                    Self {
                        digits: Decimal::equal_length_sub(&b, &a),
                        floating_length: cmp::max(self.floating_length, rhs.floating_length),
                        is_negative: true,
                    }
                };
                result
                    .remove_floating_trailing_zeros()
                    .remove_leading_zeros()
            }
        }
    }
}

impl Decimal {
    pub fn align_digit_length(a: &Self, b: &Self) -> (Vec<u8>, Vec<u8>) {
        let trailing_zero = a.floating_length as isize - b.floating_length as isize;
        let leading_zero = if a.floating_length > b.floating_length {
            a.digits.len() as isize - (b.digits.len() as isize + trailing_zero)
        } else {
            a.digits.len() as isize - trailing_zero - b.digits.len() as isize
        };
        (
            a.add_leading_trailing_zero(-leading_zero, -trailing_zero),
            b.add_leading_trailing_zero(leading_zero, trailing_zero),
        )
    }

    pub fn one_digit_mul(source: &[u8], one_digit: u8, trailing_zero: usize) -> Vec<u8> {
        let mut carry = 0;
        let mut result = source
            .into_iter()
            .rev()
            .map(|d| {
                let temp = d * one_digit + carry;
                carry = temp / 10;
                temp % 10
            })
            .collect::<Vec<_>>();
        if carry > 0 {
            result.push(carry);
        }
        result.reverse();
        (0..trailing_zero).into_iter().for_each(|_| {
            result.push(0);
        });
        result
    }

    pub fn none_equal_length_add(first: &[u8], second: &[u8]) -> Vec<u8> {
        let (long, short) = if first.len() < second.len() {
            (second, first)
        } else {
            (first, second)
        };

        let s = iter::repeat(&0)
            .take(long.len() - short.len())
            .chain(short.iter())
            .cloned()
            .collect::<Vec<_>>();
        Self::equal_length_add(long, &s)
    }

    pub fn equal_length_add(first: &[u8], second: &[u8]) -> Vec<u8> {
        let mut carry = 0;
        let mut result = first
            .iter()
            .rev()
            .zip(second.iter().rev())
            .map(|(&a, &b)| {
                let temp = a + b + carry;
                carry = if temp > 9 { 1 } else { 0 };
                temp % 10
            })
            .collect::<Vec<_>>();

        if carry == 1 {
            result.push(carry);
        }

        result.reverse();
        result
    }

    pub fn equal_length_sub(bigger: &[u8], smaller: &[u8]) -> Vec<u8> {
        let mut carry = 0;
        let mut result = bigger
            .iter()
            .rev()
            .zip(smaller.iter().rev())
            .map(|(&a, &b)| {
                let temp = a as i8 - carry - b as i8;
                carry = if temp < 0 { 1 } else { 0 };
                ((temp + 10) % 10) as u8
            })
            .collect::<Vec<_>>();
        result.reverse();
        result
    }

    pub fn remove_leading_zeros(mut self) -> Self {
        while self.integer_length() > 1 && self.digits.first().unwrap() == &0 {
            self.digits.remove(0);
        }

        self
    }

    pub fn remove_floating_trailing_zeros(mut self) -> Self {
        while self.floating_length > 0 && self.digits.last().unwrap() == &0 {
            self.digits.pop();
            self.floating_length -= 1;
        }

        self
    }

    pub fn add_leading_trailing_zero(&self, leading: isize, trailing: isize) -> Vec<u8> {
        iter::repeat(&0)
            .take(cmp::max(0, leading) as usize)
            .chain(self.digits.iter())
            .chain(iter::repeat(&0).take(cmp::max(0, trailing) as usize))
            .cloned()
            .collect()
    }

    pub fn integer_length(&self) -> usize {
        self.digits.len() - self.floating_length
    }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut digits = vec![];
        let mut is_negative = false;
        let mut f_length = 0;
        let mut is_floating = false;
        for (i, c) in input.chars().enumerate() {
            match c {
                '.' => {
                    if !is_floating {
                        is_floating = true
                    } else {
                        return None;
                    }
                }
                '+' => {
                    if i > 0 {
                        return None;
                    }
                }
                '-' => {
                    if i > 0 {
                        return None;
                    } else {
                        is_negative = true;
                    }
                }
                '0'..='9' => {
                    if is_floating {
                        f_length += 1;
                    }
                    digits.push(c.to_digit(10).unwrap() as u8);
                }
                _ => return None,
            }
        }

        let result = Self {
            digits,
            floating_length: f_length,
            is_negative,
        };
        Some(
            result
                .remove_leading_zeros()
                .remove_floating_trailing_zeros(),
        )
    }
}
