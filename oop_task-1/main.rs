mod book_sys;
mod member_sys;
mod library_sys;

use book_sys::Book;
use member_sys::Member;
use library_sys::Library;

fn main() {
    let mut library = Library::new("City Library".to_string());

    let book1 = Book::new(
        "Rust Book".to_string(),
        "Steve".to_string(),
        123,
    );
    let book2 = Book::new(
        "lol".to_string(),
        "djk".to_string(),
        15166,
    );
    let book3: Book = Book::new(
        "testing".to_string(),
        "bersh".to_string(),
        1524
    );

    let member= Member::new("Alice".to_string(), 1);
    let member2= Member::new("saiid".to_string(), 2);
    let member3= Member::new("saihghghgid".to_string(), 3);

    library.add_book(book1.clone());
    library.add_book(book2.clone());
    library.add_book(book3.clone());
    library.register_member(member.clone());
    library.register_member(member2.clone());
    library.lend_book( member2.id, book2.isbn);
    library.display_available_books();

    library.receve_book(member2, book2);
    library.display_available_books();
    
    
}
