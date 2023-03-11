#[derive(Default)]
struct User {
    email: String,
    age: u8,
}

// obviously
impl AsRef<User> for User {
    fn as_ref(&self) -> &User {
        self
    }
}

enum Privilege {
    // imagine different moderator privileges here
}

#[derive(Default)]
struct Moderator {
    user: User,
    privileges: Vec<Privilege>,
}

// since moderators are just regular users
impl AsRef<User> for Moderator {
    fn as_ref(&self) -> &User {
        &self.user
    }
}

fn takes_user<U: AsRef<User>>(user: U) {}

fn main() {
    let user = User::default();
    let moderator = Moderator::default();

    takes_user(&user);
    takes_user(&moderator); // yay
}
