use max_item::MaxItem;
fn main() {

    let answer = MaxItem::find_max(&[1,2,5]);
    println!("Our answer is {:?}", answer);

    let answer = MaxItem::find_max(&['a','~','3','*']);
    println!("Our answer is {:?}", answer);
    
}

mod max_item;
mod linked_list;