use std::collections::HashMap;

pub fn median_modas(array: &[i32]) -> (f32, Vec<i32>) {
    let mut vec_array = Vec::new();
    let mut map_array = HashMap::new();
    let mut vec_modas = Vec::new();
    
    //create hash map & and vector from list
    for i in array {
        let count = map_array.entry(i).or_insert(0);
        *count += 1;

        vec_array.push(i);
    }
    
    //search max value in list
    let mut max_value = 0;
    for value in map_array.values() {
        if value > &max_value {
            max_value = *value;
        }
    }
    
    //create vector of modas
    for (key, value) in &map_array {
        if value == &max_value {
            vec_modas.push(**key);
        }
    }    
    
    vec_array.sort_unstable();
    vec_modas.sort_unstable();

    if vec_array.len() % 2 == 0 {
        //pair_elements_in_list_branch
        ((**&vec_array[(vec_array.len() / 2) - 1] +
        **&vec_array[vec_array.len() / 2]
        ) as f32/ 2.0, vec_modas)
    
    } else {
        //odd_elements_in_list_branch
        ((**&vec_array[vec_array.len() / 2]) as f32, vec_modas)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd_elements() {
        let list = [1, 1, 2, 7, 5, 9, 2, 3, 1];
        
        assert_eq!(median_modas(&list), (2.0, vec![1]));
    }
    
    #[test]
    fn test_median_pair_elements() {
        let list = [1, 1, 2, 7, 2, 5, 9, 2, 3, 10];
        
        assert_eq!(median_modas(&list), (2.5, vec![2]));
    }
    
    #[test]
    fn test_median_bimodas() {
        let list = [1, 1, 2, 7, 2, 5, 9, 2, 3, 1, 10];
        
        assert_eq!(median_modas(&list), (2.0, vec![1, 2]));        
    }
    
    #[test]
    fn test_median_multimodas() {
        let list = [1, 1, 2, 7, 2, 5, 9, 7, 9, 2, 3, 1, 10, 7];
        
        assert_eq!(median_modas(&list), (4.0, vec![1, 2, 7]));        
    }
    
}