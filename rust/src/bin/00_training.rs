

/* 
fn largest_value (list: &[i32]) -> i32{
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

fn largest_value(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main(){

let numbers = vec![10, 20, 30];

for item in &numbers {
    let item_in_loop = item + 100;
    println!("{}", item_in_loop);
}



}