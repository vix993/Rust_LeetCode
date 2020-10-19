pub fn balanced_string_split(s: String) -> i32 {
    let s_copy: Vec<char> = s.clone().chars().collect();
    let mut r_count: i32 = 0;
    let mut l_count: i32 = 0;
    let mut ret: i32 = 0;
        
    for i in 0..s_copy.len() {
        if s_copy[i] == 'R' {
            r_count += 1;
        } else {
            l_count += 1;
        }
        if r_count == l_count && r_count != 0 {
            ret += 1;
        }
    }
        
    return ret;
}

fn main() {
    println!("{:?}", balanced_string_split("RRLLRRLLRL".to_string()));
}
