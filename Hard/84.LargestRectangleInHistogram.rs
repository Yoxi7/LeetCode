impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack = Vec::with_capacity(n);
        let mut max_area = 0;
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if heights[i] < heights[top] {
                    stack.pop();
                    let h = heights[top];
                    let w = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                    max_area = max_area.max(h * w as i32);
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        while let Some(&top) = stack.last() {
            stack.pop();
            let h = heights[top];
            let w = if stack.is_empty() { n } else { n - stack.last().unwrap() - 1 };
            max_area = max_area.max(h * w as i32);
        }
        max_area
    }
}