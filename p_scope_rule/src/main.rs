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

    // aliasing
    p_alising();
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

struct Point { x: i32, y: i32, z: i32 }
fn p_alising () {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinate {} {} {}", borrowed_point.x, another_borrow.y, point.z);
    println!("Point has coordinate {} {} {}", borrowed_point.x, another_borrow.y, point.z);

    let mutaborrow = &mut point;

    mutaborrow.x = 1;
    mutaborrow.y = 2;
    mutaborrow.z = 3;
    println!("Point has coordinate {} {} {}", mutaborrow.x, mutaborrow.y, mutaborrow.z);

    let new_borrow_point = &point;
    println!("Point has coordinate {} {} {}", new_borrow_point.x, new_borrow_point.y, new_borrow_point.z);
    println!("Point has coordinate {} {} {}", point.x, point.y, point.z);


}