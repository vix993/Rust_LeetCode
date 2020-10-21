pub fn calc_num_size (num: i32) -> i32 {
    let mut num_size: i32 = 1;
    let mut num_copy: i32 = num.clone();
        
    while num_copy > 9 {
        num_size *= 10;
        num_copy /= 10;
    }
        
    return num_size;
}
pub fn maximum69_number (num: i32) -> i32 {
    let mut num_size: i32 = calc_num_size(num);
    let mut num_copy: i32 = num.clone();
        
    while num_size > 0 {
        if (num_copy / num_size) % 6 == 0 {
            return num_copy + (3*num_size); 
        }
        num_size /= 10;
    }
    return num_copy;
}
pub fn main(){
    println!("{}, {}, {}", maximum69_number(6999), maximum69_number(6), maximum69_number(9669));
}
