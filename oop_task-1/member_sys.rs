#[derive(Clone)]
pub struct Member {
    pub name: String,
    pub id: u32,
    borrowed_books: Vec<String>, // for the isbn of the book
}

impl Member {
    pub fn new(name: String, id: u32) -> Self{
        Member {
            name,
            id,
            borrowed_books: Vec::new(),
        }
    }
    pub fn get_info(&self) {
        let _ = format!("Member name: {}, Member ID: {}, The books he/she borrowed: {:?}", self.name,self.id,self.borrowed_books);
}
    pub fn borrow_book(&mut self, isbn: String) {
        if !self.borrowed_books.contains(&isbn) {  // prevent duplicate
            self.borrowed_books.push(isbn);
        }
    }
    pub fn return_book(&mut self, isbn: String) {
            self.borrowed_books.retain(|b| *b != isbn);
    }
}

