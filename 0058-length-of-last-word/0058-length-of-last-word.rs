impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let reverse : String = s.chars().rev().collect();

        let mut first_idx = 0;
        if s.len() == 1 {
            return 1;
        }

        loop {
            if reverse.chars().nth(first_idx) != Some(' '){
                break;
            }
            first_idx += 1;
        }

        let mut last_idx = first_idx;
        loop {
            if reverse.chars().nth(last_idx) == Some(' ') || last_idx >= s.len(){
                break;
            }
            last_idx += 1;
        }

        return (last_idx - first_idx) as i32;
    }
}