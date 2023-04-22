#[nanopass::langs]
mod langs {
    mod base {}

    #[extends(base)]
    mod good {}

    #[extends(nonexistant)]
    mod bad {}
}

fn main() {}
