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

    p_ref_parten();
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

fn p_ref_parten () {
    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equivalent ref_c2 {} {}", *ref_c1, *ref_c2);

    let point = Point { x: 1, y: 1, z: 1 };
    let _copy_of_x = {
        let Point { x: ref ref_to_x, y:_, z:_ } = point;
        *ref_to_x
    };

    let mut mutable_point = point;
    {
        let Point { x:_, y: ref mut mut_ref_to_y, z:_ } = mutable_point;
        *mut_ref_to_y = 1
    }

    // println!("point is {}", point.x);
    println!("point is {} {} {}", mutable_point.x, mutable_point.y, mutable_point.z);

    let mut mut_tuple = (Box::new(5u32), 2u32);
    {
        let (_, ref mut last) = mut_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mut_tuple);
}