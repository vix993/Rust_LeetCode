// This is a medium difficulty problem on leetCode
// The idea is to order an array of cards so that
// when we 'reveal the top card' and remove it from the deck
// we can shuffle the next card into the last position
// and repeat the process making sure that we are revealing
// the cards in increasing order.

pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
    let mut sorted_deck: Vec<i32> = deck;
    let mut ret: Vec<i32> = [].to_vec();
    let mut temp: i32;
       
    if sorted_deck.len() <= 1 {
        return sorted_deck;
    }
        
    for i in 0..sorted_deck.len() {
        for j in (i+1)..sorted_deck.len() {
            if sorted_deck[i] > sorted_deck[j]{
                temp = sorted_deck[i];
                sorted_deck[i] = sorted_deck[j];
                sorted_deck[j] = temp;
            }
        }
    }
    ret.insert(0, sorted_deck.swap_remove(sorted_deck.len() - 1));
    ret.insert(0, sorted_deck.swap_remove(sorted_deck.len() - 1));
    for _ in 0..sorted_deck.len() {
        let last = ret.swap_remove(ret.len()-1);
        ret.insert(0, sorted_deck.swap_remove(sorted_deck.len() - 1));
        ret.insert(1,last);
    }
    return ret;
}

fn main () {
    println!("{:?}", deck_revealed_increasing([17,13,11,2,3,5,7].to_vec())); // change vector input for different conditions
}
