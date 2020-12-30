use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for (pos, val) in nums.iter().enumerate() {
            let complement: i32 = target - val;
            if seen.contains_key(&complement) {
                let mut res: Vec<i32> = Vec::new();
                res.push(*seen.get(&complement).unwrap());
                res.push(pos as i32);
                return res;
            }
            seen.insert(*val, pos as i32);
        }
        return Vec::new();
    }
}