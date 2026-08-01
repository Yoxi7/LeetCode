use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();

        for (i, &current) in nums.iter().enumerate() {
            let need = target - current;

            if let Some(&j) = seen.get(&need) {
                return vec![j, i as i32];
            }

            seen.insert(current, i as i32);
        }

        unreachable!()
    }
}