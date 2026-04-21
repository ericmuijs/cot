use cot::db::{Auto, ManyToMany, model};

#[model]
struct Author {
    #[model(primary_key)]
    id: Auto<i32>,
    books: ManyToMany<Book>,
}

#[model]
struct Book {
    #[model(primary_key)]
    id: Auto<i32>,
}

fn main() {}
