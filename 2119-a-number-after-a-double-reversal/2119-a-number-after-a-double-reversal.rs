impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        /* let mut st : String = num.to_string().chars().rev().collect();

        let reverse : i32 = st.parse().unwrap();
        
        let mut st2 : String = reverse.to_string().chars().rev().collect();

        let origin : i32 = st2.parse().unwrap();
        return num == origin;
        */

        if num == 0 || num % 10 != 0 {
            true
        } else {
            false
        }
    }
}