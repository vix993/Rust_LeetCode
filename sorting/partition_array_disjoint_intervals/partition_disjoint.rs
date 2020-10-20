// This is a medium level problem on leetCode
// The idea is to receive an array of i32's and find
// the position where every following array element
// is greater or equal to the previous elements.

// As of my submission this is the fastest
// and least costly solution built with Rust

pub fn check_is_lesseq_than_rest(num_to_run: i32, vec_to_check: Vec<i32>) -> bool {
    for num in vec_to_check {
        if num_to_run > num {
            return false;
        }
    }
    return true;
}
    
pub fn partition_disjoint(a: Vec<i32>) -> i32 {
    let mut ret: i32 = 0;
    let mut current_largest: i32 = a[0];
    let mut add_to_left: bool = true;
    let mut left: Vec<i32> = [].to_vec();
    let mut right: Vec<i32> = [].to_vec();
    let mut copy: Vec<i32> = a.clone();
        
    if copy.len() == 2 || check_is_lesseq_than_rest(current_largest, copy.clone()) {
        if copy.len() > 2 {
            left.insert(0, copy.swap_remove(0));
//            right = copy.clone();
            return 1;
        }
        left.insert(0, copy.swap_remove(0));
        right.insert(0, copy.swap_remove(0));
        return 1;
    }
    for num in a.clone() {
        if num >= current_largest {
            if check_is_lesseq_than_rest(current_largest, copy.clone()) {
                add_to_left = false;
            }
            else {
                current_largest = num;
            }
        }
        if add_to_left {
            let len = left.len();
            left.insert(len, copy[0]);
            copy.remove(0);
            ret += 1;
        }
        else {
            let len = right.len();
            right.insert(len, copy.swap_remove(0));
        }    
    }
    return ret;
}

fn main() {
    println!("{}", partition_disjoint([5, 0, 3, 8, 6].to_vec())); // change array values for different conditions
}
