pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = [].to_vec();
        
    for num in &nums {
        let mut count = 0;
        for i in 0..nums.len() {
            if num > &nums[i] {
                count += 1;
            }
        }
        ret.push(count);
    }
    return ret;
}

pub fn main() {
    println!("{:?}", smaller_numbers_than_current([1, 5, 3, 2, 4].to_vec()));
}
