pub mod helper;

fn main(){
    println!("hello");
    // test_func();
    let myresult:String=helper::namehelpers::get_full_name("Ayush","SINGH");
    println!("Hello from {0}",myresult);

    let new_age=helper::privatefns::get_age_plue_5(23);
    println!("new age is {}",new_age);
}



#[allow(dead_code)]
fn test_func(){
    let x:f32=255.0;
    let y:i32= x as i32 -5;
    println!("{}",y);


    let mut iam:bool=true;
    iam=false;
    println!("{}",iam);

    let mystr:char='😊';
    println!("{}",mystr);

    let mut name:&str="ayush";
    name="singh";
    println!("{}",name);

    //in tuples u can have multiple data types
    let name1:(&str,&str,i32)=("ayush","singh",40);
   println!("{:?}",name1);

   // if default formatter error  {} --> {:?}

   let ages:[u16;7]=[40,30,50,60,70,80,90];
    println!("{:?}",ages);

    let new_ages:&[u16]=&ages[1..=4];
    println!("{:?}",new_ages);   
 
}




