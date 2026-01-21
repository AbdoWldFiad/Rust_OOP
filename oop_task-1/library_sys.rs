use crate::book_sys::Book;
use crate::member_sys::Member;

pub struct Library {
    name: String,
    books: Vec<Book>,
    members: Vec<Member>, 
}

impl Library {
    pub fn new(name: String) -> Self {
        Library{
            name,
            books: Vec::new(),
            members: Vec::new(),
        }
    }
    pub fn add_book(&mut self, book: Book){self.books.push(book)}

    pub fn register_member(&mut self, member: Member){self.members.push(member)}

    pub fn lend_book(&mut self, member_id: u32, isbn: u32) {
        let book = self.books.iter_mut().find(|b| b.isbn == isbn);
        let member = self.members.iter_mut().find(|m| m.id == member_id);

        if let (Some(book), Some(member)) = (book, member) {
            if book.is_available {
                book.borrow();
                member.borrow_book(isbn.to_string());
            }
        }
    }


    pub  fn receve_book(&mut self, mut member: Member, mut book: Book){
        member.return_book(book.isbn.to_string());
        book.return_book();
    }

    pub fn display_available_books(&self) {
        println!("Available books in {}:", self.name);
        for book in &self.books {
            if book.is_available {println!("{} , ISBN: {}",book.title,book.isbn )}
        }
    }
}