struct Person<PetType:Animal>{
    first_name:String,
    pet:PetType,
}

trait  Animal {}

struct Dog{}
impl Animal for Dog{ }

#[allow(dead_code)]
struct Cat{}
impl Animal for Cat{}

#[allow(dead_code)]
struct Bear{}
impl Animal for Bear{}

#[allow(dead_code)]
struct Tiger{}
impl Animal for Tiger{}



// struct Character{
//     hit_points:u16,

// }

pub fn create_person(){
    let pet1=Dog{};
    let pet2=Cat{};
    let pet3=Bear{};
    let pet4=Tiger{};
   let p1:Person<Cat>=Person{first_name:"Ayush".to_string(),pet:pet2};
}