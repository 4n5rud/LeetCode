impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let st : String = n.to_string();

        let mut hap = 0;
        let mut gop = 1;

        for i in 0..st.len() {
        let num: i32 = st.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;            
        hap = hap + num;
        gop = gop * num;

        }

        return gop - hap;
    }
}