#![feature(test)]

extern crate test;

// IMporTANT NOTE
// This is my (over engineered) attempt at a vowel counter
// Based on "Sloth Bytes" by the coding sloth
// Newspaper https://slothbytes.beehiiv.com/subscribe
//
// Details:
// # How Many Vowels?
//
// Create a function that takes a string and returns the number (count) of vowels in the string.
//
// # Examples
//
// count_vowels("Celebration") ➞ 5 # 5 vowels
// count_vowels("Palm") ➞ 1 # 1 vowel
// count_vowels("Prediction") ➞ 4 # 4 vowels
//
// # Notes
//
// - a, e, i, o, u are considered vowels (not y).
// - All test cases are one word and only contain letters
//
// My Results / conclusion:
// * Note, when saying saying "identical" I mean with a
//   small variation that is not significant enough to be a factor.
//   this is usually caused by memory layout differences, which can be ignored and considered "identical".
//
// 1. Standard for loop on a &str - pretty fast for most use cases
// 2. Functional approach on a &str - sometimes was ~20% faster than 1, sometimes it was identical
// 3. Meme, hardcoded it, 0.25ns let's go
// 4. unsafe get_unchecked on a &[char] - Around 2x faster than 1/2
// 5. unsafe get_unchecked on a &[u8] - Identical to 4
// 6. branch-less 5 by casting to a number - Around 1.5x faster than 1/2 or 50% slower than 4
//    * My assumption is that having a branch is more efficient than just adding a conditional
//      statement, my hypothesis is due to speculative execution but I'm not sure.
// 7. 5 but with half the comparisons - Identical to 5
//    * Assumes the first letter is always capitalized and the rest are lowercase
//      this allows only comparing 5 characters instead of 10, seems to have no impact on performance
//
// So which should I use?
// If you need a BLAZINGLY FAST vowel counter (for whatever reason):
// Your best bet is option 5 for ascii-only or 4 for utf8.
//
// Most likely you won't need a super fast vowel counter:
// I'd recommend option 1 or 2 for best readability.

fn main() {
    println!("Hello, world!");
}

//NOTE
// Ordered from most frequent to least frequent
// This allows the computer to short-circuit more often
macro_rules! is_vowel {
    ($c:expr) => {
        $c == 'e' || // 12.7%
        $c == 'a' || // 8.2%
        $c == 'o' || // 7.5%
        $c == 'i' || // 7.0%
        $c == 'u' || // 2.8%
        
        $c == 'E' || // 12.7%
        $c == 'A' || // 8.2%
        $c == 'O' || // 7.5%
        $c == 'I' || // 7.0%
        $c == 'U'    // 2.8%
    };
}

//NOTE
// Ordered from most frequent to least frequent
// This allows the computer to short-circuit more often
macro_rules! is_vowel_u8 {
    ($c:expr) => {
        $c == b'e' || // 12.7%
        $c == b'a' || // 8.2%
        $c == b'o' || // 7.5%
        $c == b'i' || // 7.0%
        $c == b'u' || // 2.8%
        
        $c == b'E' || // 12.7%
        $c == b'A' || // 8.2%
        $c == b'O' || // 7.5%
        $c == b'I' || // 7.0%
        $c == b'U'    // 2.8%
    };
}

macro_rules! is_vowel_u8_lowercase {
    ($c:expr) => {
        $c == b'e' || // 12.7%
        $c == b'a' || // 8.2%
        $c == b'o' || // 7.5%
        $c == b'i' || // 7.0%
        $c == b'u'    // 2.8%
    };
}

macro_rules! is_vowel_u8_uppercase {
    ($c:expr) => {
        $c == b'E' || // 12.7%
        $c == b'A' || // 8.2%
        $c == b'O' || // 7.5%
        $c == b'I' || // 7.0%
        $c == b'U'    // 2.8%
    };
}

fn vowel_count(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if is_vowel!(c) {
            count += 1;
        }
    }
    count
}

fn vowel_count2(s: &str) -> usize {
    s.chars().filter(|c| is_vowel!(*c)).count()
}

// NOTE - Assumes input doesn't change :P
fn vowel_count3(_s: &str) -> usize {
    3
}

fn vowel_count4(chars: &[char]) -> usize {
    unsafe {
        let mut count = 0;
        for i in 0..chars.len() {
            let c = *chars.get_unchecked(i);
            if is_vowel!(c) {
                count += 1;
            }
        }
        count
    }
}

fn vowel_count5(chars: &[u8]) -> usize {
    unsafe {
        let mut count = 0;
        for i in 0..chars.len() {
            let c = *chars.get_unchecked(i);
            if is_vowel_u8!(c) {
                count += 1;
            }
        }
        count
    }
}

fn vowel_count6(chars: &[u8]) -> usize {
    unsafe {
        let mut count = 0;
        for i in 0..chars.len() {
            let c = *chars.get_unchecked(i);
            count += is_vowel_u8!(c) as usize;
        }
        count
    }
}

fn vowel_count7(chars: &[u8]) -> usize {
    unsafe {
        let mut count = 0;
        if is_vowel_u8_uppercase!(*chars.get_unchecked(0)) {
            count += 1;
        }
        for i in 1..chars.len() {
            let c = *chars.get_unchecked(i);
            if is_vowel_u8_lowercase!(c) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[bench]
    fn bench_count(b: &mut test::Bencher) {
        let chars = "Hello, world!";
        b.iter(|| {
            let result = vowel_count(chars);
            assert_eq!(result, 3);
        });
    }
    
    #[bench]
    fn bench_count2(b: &mut test::Bencher) {
        let chars = "Hello, world!";
        b.iter(|| {
            let result = vowel_count2(chars);
            assert_eq!(result, 3);
        });
    }
    
    #[bench]
    fn bench_count3(b: &mut test::Bencher) {
        let chars = "Hello, world!";
        b.iter(|| {
            let result = vowel_count3(chars);
            assert_eq!(result, 3);
        });
    }
    
    #[bench]
    fn bench_count4(b: &mut test::Bencher) {
        let chars = "Hello, world!".chars().collect::<Vec<char>>();
        b.iter(|| {
            let result = vowel_count4(&chars);
            assert_eq!(result, 3);
        });
    }
    
    #[bench]
    fn bench_count5(b: &mut test::Bencher) {
        let chars = "Hello, world!".as_bytes();
        b.iter(|| {
            let result = vowel_count5(&chars);
            assert_eq!(result, 3);
        });
    }
    
    #[bench]
    fn bench_count6(b: &mut test::Bencher) {
        let chars = "Hello, world!".as_bytes();
        b.iter(|| {
            let result = vowel_count6(&chars);
            assert_eq!(result, 3);
        });
    }
    
    #[bench]
    fn bench_count7(b: &mut test::Bencher) {
        let chars = "Hello, world!".as_bytes();
        b.iter(|| {
            let result = vowel_count7(&chars);
            assert_eq!(result, 3);
        });
    }
}
