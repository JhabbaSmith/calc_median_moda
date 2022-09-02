use median::median;
fn main() {
    let list = [2, 1, 4, 9, 2, 7, 3];

    println!("\n\nList is: \t{:?}", list);    
    println!("Median is: \t{}", median(&list));
}
