pub fn max_depth(s: String) -> i32 {
    let mut par_started: i32 = 0;
    let mut par_ended: i32 = 0;
    let mut ret: i32 = 0;
    let s_copy: Vec<char> = s.clone().chars().collect();
        
    for c in s_copy {
        if c == '(' as char {
            par_started += 1;
        }
        if c == ('(' as char) && par_ended > 0 {
            ret -= 1;
            par_ended -= 1;
        } 
        if c == (')' as char) && par_started > 0 {
            par_ended += 1;
            par_started -= 1;
            ret += 1;
        }
    }
    return ret;
}

pub fn main() {
    println!("{}", max_depth("(1)(1)(1)".to_string()));
    println!("{}", max_depth("(1)((2))".to_string()));
    println!("{}", max_depth("(1)((2))(((3)))".to_string()));
}
