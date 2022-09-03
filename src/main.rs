use median::median_modas;
fn main() {
    let list = [2, 2, 1, 1, 1, 4, 9, 2, 7, 3];
    let tup: (f32, Vec<i32>) = median_modas(&list);
    
    println!("\n\nList is: \t{:?}", list);    
    println!("Median is: \t{}\nModa(s) is: \t{:?}", tup.0, tup.1);
}
