pub fn test_rust_iterators(){
    let fruit_list= vec!["Strawberry","Blueberry","Mango","Orange","Apple"];

let nut_list=vec!["Walnut","Almond","Pecans"];

  let mut fruit_iter=  fruit_list.iter();

//   for fruit in fruit_iter{
//     print!("{}",fruit);
//   }

fruit_iter.next();
fruit_iter.next();
let item01=fruit_iter.next();
println!("First item in iterator is :{} ",item01.unwrap());

let aggregate_foods=fruit_list.iter().chain(&nut_list);

let all_foods:Vec<&&str>=aggregate_foods.clone().collect();

for food in aggregate_foods{
    println!("Eating {}",food);
}

let  fruit_list_strings=fruit_list.iter().map(|e| String::from(*e));
let new_fruits=fruit_list_strings.map(|mut e|  {e.push_str("fruit");  return e;});

new_fruits.for_each(|e| println!("{}",e));




}