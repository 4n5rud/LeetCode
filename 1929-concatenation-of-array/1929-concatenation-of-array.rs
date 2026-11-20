impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = nums.clone();

        for i in 0..nums.len() {
            answer.push(nums[i]);
        }

        return answer;
    }
}