fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one(&mut self) {
        self.0 += 1;
    }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

fn run_add_one() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

fn main() {
    let s1 = String::from("Rust");
    let s1_r = &s1;
    {
        let s2 = String::from("C");
        let res = the_longest(s1_r, &s2);
    }

    run_add_one()
}
