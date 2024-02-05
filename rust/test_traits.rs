struct Person<PetType,PetType2:Animal > where PetType:Animal + NotDangerous{
    first_name:String,
    pet:PetType,
    pet2:PetType2,
}

trait  Animal {
    fn make_sound(&self)->();
}

trait NotDangerous{}
trait Dangerous{}

struct Dog{}
impl NotDangerous for Dog{}
impl Animal for Dog{ 
    fn make_sound(&self)->() {
        println!("Dog barked");
    }
}


struct Cat{}
impl NotDangerous for Cat{}
impl Animal for Cat{
    fn make_sound(&self)->() {
        println!("cat meowed");
    }
}


struct Bear{}
impl Dangerous for Bear{}
impl Animal for Bear{
    fn make_sound(&self)->() {
        println!("bear roared");
    }
}


struct Tiger{}
impl Dangerous for Tiger{}
impl Animal for Tiger{
    fn make_sound(&self)->() {
        println!("tiger roared");
    }
}



// struct Character{
//     hit_points:u16,

// }

pub fn create_person(){
    let pet1=Dog{};
    let pet2=Cat{};
    let pet3=Bear{};
    let pet4=Tiger{};
   let p1:Person<Cat,Bear>=Person{first_name:"Ayush".to_string(),pet:pet2,pet2:pet3};
   p1.pet.make_sound();
}