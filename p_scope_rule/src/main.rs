fn main() {
    println!("Hello, world!");

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let ref_to_i32: &i32 = &boxed_i32;
        // eat_box_i32(boxed_i32);
        borrow_i32(ref_to_i32);
    }
    eat_box_i32(boxed_i32);

    println!("-----------------------");

    let immutabook = Book {
        author: "aya",
        title: "Good Book",
        year: 2021,
    };

    let mut mutabook = immutabook;
    borrow_book(&immutabook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    // new_edition(&mut immutabook);
}

fn eat_box_i32 (boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32 (borrowed_i32: &i32) {
    println!("borrowed_i32 is {}", borrowed_i32);
}

#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: i32,
}

fn borrow_book (book: &Book) {
    println!("borrow book title :{}, author: {}", book.title, book.author);
}

fn new_edition (book: &mut Book) {
    book.year = 2022;
    println!("book edition year is {}", book.year);
}