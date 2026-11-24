impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n == 1 {
            return 2;
        }

        if n % 2 == 0 { 
            n
        } else {
            n * 2
        }
    }
}