use std::io;

struct User<Privledges> {
    username: String,
    email: String,
    online: bool,
    login_count: i32,
    privledges: Privledges,
    banned: bool,
}

enum Privledges {
    Admin,
    Member, 
    Guest, 
}

trait greet {
    fn say_hi(&self) -> () {
        println!("Hello! My username is {}", self.username)
    } 
}

impl User<Privledges> {

    fn create(username: String, email: String) -> User<Privledges> {

        let privledges = match username == "steve" {
            true => Privledges::Admin, 
            _ => Privledges::Member,
        };

        return User {
            username,
            email,
            online: true,
            login_count: 0,
            privledges,
            banned: false,
        }
    }

    fn login(&mut self) -> () {
        if self.banned { return println!("you are banned and cannot log in!") }

        match self.online{
            true => println!("Already logged in"),
            false => {
                self.online = true;
                self.login_count += 1;
                println!("Successfully logged in. Total logins: {}", &self.login_count);
            },
        }
    }

    fn logout(&mut self) -> () {
        match self.online{
            true => {
                println!("You successfully logged out {}!", self.username);
                self.online = false
            },
            false => println!("You are already logged out!"),
        }
    }

    fn ban(&self, user: &mut User<Privledges>) -> () {
        if let Privledges::Admin = &self.privledges {
            println!("User {} has been banned!", user.username);
            user.logout();
            user.banned = true
        } else {
            println!("You do not have privledges to do this action")
        }
    }


}

fn main() -> () {
    let mut john = User::create(
        String::from("john"),
        String::from("john@john.com"
                    ));
    let mut steve = User::create(
        String::from("steve"),
        String::from("steve@steve.com"
                    ));
    john.login();
    john.logout();
    john.ban(&mut steve);

    steve.login();
    steve.logout();
    steve.ban(&mut john);

}
