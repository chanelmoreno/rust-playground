// example of what a trait is here

trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]

enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            //match is similar to a switch statement
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "tai chi".to_string(),
        }
    }
    // fn choose_weapon(&self) -> String {
    //     Character::Warrior => "Sword".to_string(),
    //     Character::Archer => "Archer".to_string(),
    //     Character::Wizard => "Wand".to_string(),

    // }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn tests_traits() {
        let my_character: Character = Character::Archer;
        let chosen_style: String = my_character.choose_style();
        dbg!(chosen_style);
    }
}
