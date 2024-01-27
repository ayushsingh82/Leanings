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
    
    
}