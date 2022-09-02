pub fn median(array: &[i32]) -> f32 {
    let mut vec_array = Vec::new();
     
    for i in array {
        vec_array.push(i);
    }
    vec_array.sort_unstable();

    if vec_array.len() % 2 == 0 {
        //pair_branch
        (**&vec_array[(vec_array.len() / 2) - 1] +
        **&vec_array[vec_array.len() / 2]
        ) as f32/ 2.0
    
    } else {
        //odd_branch
        (**&vec_array[vec_array.len() / 2]) as f32
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd_elements() {
        let list = [1, 1, 2, 7, 2, 5, 9, 2, 3];
        
        assert_eq!(median(&list), 2.0);
    }
    
    #[test]
    fn test_median_pair_elements() {
        let list = [1, 1, 2, 7, 2, 5, 9, 2, 3, 10];
        
        assert_eq!(median(&list), 2.5);
    }
}