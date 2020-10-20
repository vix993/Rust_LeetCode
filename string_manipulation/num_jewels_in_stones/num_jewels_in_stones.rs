pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let j_copy: Vec<char> = j.chars().collect();
    let s_copy: Vec<char> = s.chars().collect();
    let mut ret: i32 = 0;
        
    for c in &s_copy {
        for u in &j_copy {
            if c == u {
                ret += 1;
            }
        }
    }
    return ret;
}

pub fn main() {
    println!("{}", num_jewels_in_stones("aA".to_string(), "bbbaaA".to_string()));
    println!("{}", num_jewels_in_stones("c".to_string(), "ccccccc".to_string()));
    println!("{}", num_jewels_in_stones("lrs".to_string(), "lrlrdfs".to_string()));
}
