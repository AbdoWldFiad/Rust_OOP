#[derive( Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: u32,
    pub is_available: bool
}
impl Book {
    pub fn new(title: String, author: String, isbn: u32) -> Self {
        Book {
            title,
            author,
            isbn,
            is_available: true,
        }
    }
    pub fn get_info(&self) {
        let _ = format!("Title: {}, Author: {}, isbn: {}, Is it available: {}",self.title,self.author,self.isbn,self.is_available);
}
    pub fn borrow(&mut self) {
        self.is_available = false;
}
    pub fn return_book(&mut self) {
        self.is_available = true;
}
}
