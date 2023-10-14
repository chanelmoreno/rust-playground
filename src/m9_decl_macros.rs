#[cfg(test)]

mod tests {
    #[test]
    fn tests_declarative_macro(){

        println!("Hello 1");
        dbg!("Hello 2");
        
        let x = vec!(1, 2,3);
        let formatted: String = format!("Hello 3 with vec{:?}", x);
        dbg!(formatted);


    }
}