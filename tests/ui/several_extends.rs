#[nanopass::langs]
mod langs {
    #[extends(foo)]
    #[expected(bar)]
    mod baz {}
}

fn main() {}
