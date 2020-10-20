pub fn number_of_steps (num: i32) -> i32 {
    let mut ret = 0;
    let mut copy = num.clone();
        
    while copy > 0 {
        if copy % 2 == 0 {
            copy /= 2;
        } else {
            copy -= 1;
        }
        ret += 1;
    }
    return ret;
}

pub fn main() {
    println!("{:?}", number_of_steps(14));
    println!("{:?}", number_of_steps(1));
    println!("{:?}", number_of_steps(100));
}
