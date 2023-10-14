// structs are like an object in javascript

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_signin_count(&mut self) {
        //read values but also mutate the values of self, in this case the User
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email)
    }

    fn change_username(&mut self, new_username: &str) {
        self.username = String::from(new_username)
    }

    fn change_activestate(&mut self) {
        if self.active {
            self.active = false
        } else {
            self.active = true
        }
    }
}

fn change_username_manual(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn tests_struts() {
        let mut user_1: User = User {
            username: String::from("someusername"),
            email: String::from("someusername@example.com"),
            sign_in_count: 1,
            active: true,
        };

        change_username_manual(&mut user_1, "manual-update-new-username");

        dbg!(user_1);

        let mut user_2: User = User {
            username: String::from("someusername2"),
            email: String::from("someusername2@example.com"),
            sign_in_count: 5,
            active: false,
        };

        user_2.increment_signin_count();
        user_2.change_email("user2-new-email@example.com");

        dbg!(user_2);
    }
}
