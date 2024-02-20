use std::collections::HashMap;

pub fn test_hashmap_basic(){
    let mut stock_list:HashMap<String,f32>=HashMap::new();
    println!("{}",stock_list.len());

    stock_list.insert("NVD".to_string(),478.52);
    stock_list.insert("AAPL".to_string(),488.52);
    stock_list.insert("AMSC".to_string(),56.52);

    stock_list.insert("AAPL".to_string(),498.92);  //override the existing value
    
     stock_list.entry("META".to_string()).or_insert(346.68);
     stock_list.entry("META".to_string()).or_insert(347.68); //does not over ride if value already exist
 

    println!("{:#?}",stock_list);

     stock_list.remove(&("AAPL".to_string()));

     println!("{:#?}",stock_list);
}