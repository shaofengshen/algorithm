struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut i = 0;
        for j in 1..nums.len() {
            // 判断是否为重复元素，如果是则跳过重复项并递增j，否则复制元素并递增i
            if nums[i] != nums[j] {
                if j - i > 1 {
                    nums[i + 1] = nums[j];
                }
                i += 1;
            }
        }

        return (i + 1) as i32;
    }
}

// 添加测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(&nums[0..2], &[1, 2]);

        let mut nums = vec![3, 3, 4, 4, 33];
        assert_eq!(Solution::remove_duplicates(&mut nums), 3);
        assert_eq!(&nums[0..3], &[3, 4, 33]);
    }
}
