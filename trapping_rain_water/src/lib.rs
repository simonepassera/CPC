use core::cmp::max;

/// Given n non-negative integers representing an elevation map where the width of each bar is 1,
/// compute how much water it can trap after raining.
pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut left_max = height[left];
    let mut right_max = height[right];
    let mut water = 0;

    while left < right {
        if left_max < right_max {
            left += 1;
            left_max = max(left_max, height[left]);
            water += left_max - height[left];
        } else {
            right -= 1;
            right_max = max(right_max, height[right]);
            water += right_max - height[right];
        }
    }

    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(result, 6);
    }
}
