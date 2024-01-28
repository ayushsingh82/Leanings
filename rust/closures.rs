struct Person{
    first_name:String,
    last_name:String
}

pub fn test_closures(){

    let add =|x:i8,y:i8| println!("Returning some text {} {}",x,y);

    // let add =|x,y| x+y;

    let add =|x,y| {
        println!("x:{} y:{}",x,y);
        x+y
    };
    
   let result= add(8,18);

   let print_result=|| println!("result is {}",result);
   print_result();

    let mut p1=Person{first_name:"Trevour".to_string(),last_name:"Sullivan".to_string()};
    let mut  change_name=|new_lat_name:&str| p1.last_name=new_lat_name.to_string();
    change_name("Johens");
    println!("{}",p1.last_name);
    
    
}