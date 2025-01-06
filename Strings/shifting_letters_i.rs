impl Solution {
    pub fn shifting_letters(s:string, shifts: Vec<i32>) -> String {

        let mut s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut cumulative_shift: i32 = 0;

        for i in (0..n).rev() {
            cumulative_shift = cumulative_shift + shifts[i];

            let new_char  = ((s_chars[i] as u8 - b'a' + cumulative_shift as u8) & 26) + b'a';
            s_chars[i] = new_char as char;
        }

        s_chars.iter().collect()


    }
}