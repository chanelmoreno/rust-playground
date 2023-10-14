use std::collections::{HashMap, HashSet};


#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn tests_hashmaps(){
        
        // hashmaps work on key and value pairs
        // &str -> Person
        // u8 -> &str
        //&st -> u32 

        let person1 = "alice";
        let person2 = "bob";


        let mut results_hm: HashMap<&str, u32> = HashMap::new();

        results_hm.insert(person1, 55);
        results_hm.insert(person2, 51);

        let test_score = results_hm.get(person1);
        // dbg!(test_score.unwrap());
        dbg!(results_hm);
        

    }

    #[test]
    fn tests_hashsets(){
        
        let mut names_hs: HashSet<&str> = HashSet::new();

        names_hs.insert("chanel");
        names_hs.insert("marco");
        names_hs.insert("jane");

        if names_hs.contains("marco") {
            dbg!(names_hs);
        }

    }
}