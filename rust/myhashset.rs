use std::collections::HashSet;

pub fn test_hashset_type(){
    let planet_list=HashSet::from(["MERCURY","venus","Earth"]);

    let planet_list_more=HashSet::from(["Earth","Mars","Jupitor"]);

    let planet_diff=planet_list.difference(&planet_list_more);

    // for planet in planet_list{
    //     println!(" thanks for adding {}",planet);
    // }

    for planet in planet_diff{
        println!(" thanks for adding {}",planet)
    }
}