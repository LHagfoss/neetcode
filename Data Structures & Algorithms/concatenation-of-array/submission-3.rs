impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new(); 

        result.extend(nums.clone());
        result.extend(nums.clone());
    
        return result
    }
}
