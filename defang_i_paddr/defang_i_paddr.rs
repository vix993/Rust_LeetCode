pub fn defang_i_paddr(address: String) -> String {
    let mut ret: Vec<char> = address.clone().chars().collect();
    let mut iterating: bool = true;
    let mut i: usize = 0;
        
    while iterating == true {
        if ret.clone()[i] == '.' {
            ret.insert(i, '[');
            ret.insert(i + 2, ']');
            i += 2;
        }
        i += 1;
        if i >= ret.clone().len() {
            iterating = false;
        }
    }
    return ret.iter().cloned().collect::<String>();
}

pub fn main() {
    println!("{:?}", defang_i_paddr("255.100.50.0".to_string()));
}
