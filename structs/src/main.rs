// #[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    fn greet(&self) -> () {
        println!("Hello! My username is {}", self.username)
    }

    fn same_sign_in_count(&self, other: &User) -> bool {
        self.sign_in_count == other.sign_in_count
    }

    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
            sign_in_count: 0,
        }
    }
}
// ^^^ is the same as vvv 
fn create_user(username: String, email: String) -> User {
    let user = User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    };
    return user;
}

fn main() {
    let user1 = create_user(
        String::from("john"),
        String::from("john@john.com")
        );

    let user2 = User {
        username: String::from("tom"),
        email: String::from("tom@tom.com"),
        sign_in_count: 1,
        ..user1
    };

    let user3 = User::new(
        String::from("tango"),
        String::from("tango@tango.com"),
        );

    // println!("user one{:#?} and user two{:#?}", &user1, &user2);
    // dbg!(&user1);

    user1.greet();
    user2.greet();
    user3.greet();

    // println!("does {} and {} have the same sign in count? {}",
    //          user1.username,
    //          user2.username,
    //          user1.same_sign_in_count(&user2));
}
