#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn tests_patterns() {
        dbg!("hi patterns");

        let number: i32 = 95;

        let result: &str = match number {
            1 => "This is the first!",
            2 | 3 | 5 | 7 | 15 | 20 => "We found it in the sequence!!",
            _=> "It was something else!"
        };

        println!("{}", result);
        
    }
    
    #[test]
    fn tests_match_option() {
        let some_num: Option<i32> = Some(10);
        // let some_num: Option<i32> = None;

        if  let Some(i) = some_num {
            println!("{}", i)
        } else {
            panic!("PROBLEM0")
        }
        
        // let result = match some_num {
        //     Some(i) => i,
        //     None => {
        //         panic!("there was a problem");
        //     }
        // };
        // println!("{}", result);
    }

    #[test]
    fn tests_match_guard() {
        let pair: (i32, i32)= (2, -2);
        match pair {
            (x,y) if x == y => println!("They match!"),
            (x,y) if x + y == 0 => println!("They neutralise!"),
            (_,y) if y == 2 => println!("Y is indeed 2!"),
            _ => println!("we are not bothered")
        };


    }
  
      #[test]
    fn tests_match_struct() {

        struct Location {
            x: i32, 
            y: i32 
        }

        let location = Location{x: 0, y:20};

        match location {
            Location{x, y:0} => println!("y is on the axis"),
            Location{x:0, y} => println!("x is on the axis"),
            Location{x, y} => println!("neither is on the axis")
        };

    }
}
