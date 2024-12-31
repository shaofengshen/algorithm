mod test_cases;

struct Solution;

impl Solution {
    pub fn move_zeroes(nums:&mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            // 每当nums[j] != 0, 将其赋值给num[i]
            // 同时将索引 i 移动到下一位
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
            // 将索引 j 移动到下一位
            j += 1;
        }
        // 当上面的程序运行完之后，不为 0 的都在前面，但是后面还是存在有非零的数
        // 接下来，要把 i～n-1 设置为0
        let mut k = i;
        while k < nums.len() {
            nums[k] = 0;
            k += 1;
        }
    }
}

fn main() {
    for mut nums in test_cases::get_test_cases() {
        println!("before: {:?}", nums);
        Solution::move_zeroes(&mut nums);
        println!("after: {:?}", nums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}

// 解题思路
// 遍历数组 nums，把非零元素(假设有i个)按顺序存入数组第 0~i-1 位置，再把数组的第 i~n-1 位置全部设置为0