// use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // let mut seen: HashMap<char, i32> = HashMap::new();
        // for val in s.chars() {
        //     *seen.entry(val).or_insert(0) += 1;
        // }
        // // for (viking, health) in &seen {
        // //     println!("{:?} has {} hp", viking, health);
        // // }
        // for (i, val) in s.chars().enumerate() {
        //     if *seen.get(&val).unwrap() == 1 {
        //         return i as i32;
        //     }
        // }
        // return -1;
        let mut counter = [0u16; 26];
        let s = s.into_bytes();
        for &byte in s.iter() {
            counter[(byte - b'a') as usize] += 1;
        }
        // println!("{:?}", counter);
        for (i, val) in s.iter().enumerate() {
            if counter[(val - b'a') as usize] == 1u16 {
                return i as i32;
            }
        }
        return -1;
    }
}