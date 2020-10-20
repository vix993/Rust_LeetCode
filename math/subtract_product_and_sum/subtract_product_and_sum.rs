pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut prod: i32 = 1;
    let mut sum: i32 = 0;
    let mut num_copy: i32 = n.clone();
        
    while num_copy > 0 {
        sum = sum + (num_copy % 10);
        prod = prod * (num_copy % 10);
        num_copy /= 10;
    }
        
    return prod - sum;
}

pub fn main() {
    println!("{}", subtract_product_and_sum(234));
    println!("{}", subtract_product_and_sum(12));
    println!("{}", subtract_product_and_sum(1112));
}
