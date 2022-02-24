#[allow(unused_imports)] // Delete in the future
#[allow(dead_code)] // Delete in the future

use tokio::sync::mpsc;

pub struct DatabaseQ {
    // value: String
    // sender: mpsc:Sender<String>
}



// static mut db_query : mpsc::Sender<DatabaseQ>;



pub enum DbAction {
    Query = 0isize,
    Upload,
    Exist,
}

struct DbQuery {
    id: usize,
    value: String,
}


#[inline]
fn db_manage() {}

#[inline]
fn init_db() -> DatabaseQ {
    std::thread::spawn(db_manage);

    DatabaseQ {}
}

#[inline]
pub fn answer_db(_value: &str) -> Option<String> {
    None
}