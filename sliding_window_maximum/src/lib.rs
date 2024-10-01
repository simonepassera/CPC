use std::collections::{BTreeSet, BinaryHeap, VecDeque};

pub fn brute_force(v: &Vec<i32>, k: usize) -> Vec<i32> {
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();

    if k > n {
        return Vec::<i32>::new();
    }

    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    for i in 0..k - 1 {
        heap.push((nums[i], i));
    }

    let mut maxs = Vec::with_capacity(n - k + 1);

    for i in k - 1..n {
        heap.push((nums[i], i));

        while let Some((_, idx)) = heap.peek() {
            if *idx < i - (k - 1) {
                heap.pop();
            } else {
                break;
            }
        }

        maxs.push(heap.peek().unwrap().0);
    }

    maxs
}

pub fn bst(nums: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();

    if k > n {
        return Vec::<i32>::new();
    }

    let mut maxs = Vec::with_capacity(n - k + 1);

    let mut set = BTreeSet::new();
    let mut max_sf = nums[0];

    for (i, &v) in nums.iter().enumerate() {
        set.insert((v, i));

        // keep track of the max so far to avoid a costly query to the set
        max_sf = max_sf.max(v);

        if i >= k {
            set.remove(&(nums[i - k], i - k));
            if max_sf == nums[i - k] {
                max_sf = set.last().unwrap().0;
            }
        }
        if i >= k - 1 {
            maxs.push(max_sf);
        }
    }

    maxs
}

pub fn linear(nums: &Vec<i32>, k: usize) -> Vec<i32> {
    let n = nums.len();

    if k > n {
        return Vec::<i32>::new();
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut maxs: Vec<i32> = Vec::with_capacity(n - k + 1);

    for i in 0..k {
        while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
            q.pop_back();
        }

        q.push_back(i);
    }
    
    maxs.push(nums[*q.front().unwrap()]);

    for i in k..n {
        while let Some(&front) = q.front() {
            if front + k <= i {
                q.pop_front();
            } else {
                break;
            }
        }

        while (!q.is_empty()) && nums[i] > nums[*q.back().unwrap()] {
            q.pop_back();
        }

        q.push_back(i);
        maxs.push(nums[*q.front().unwrap()]);
    }

    maxs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = brute_force(&vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);

        let result = bst(&vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);

        let result = heap(&vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);

        let result = linear(&vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    }
}
