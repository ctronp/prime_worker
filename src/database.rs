// pub struct DatabaseQ {
//     // value: String
//     // sender: mpsc:Sender<String>
// }

// static mut db_query : mpsc::Sender<DatabaseQ>;

// pub enum DbAction {
//     Query = 0isize,
//     Upload,
//     Exist,
// }
//
// struct DbQuery {
//     id: usize,
//     value: String,
// }

// #[inline]
// fn db_manage() {}
//
// #[inline]
// fn init_db() -> DatabaseQ {
//     std::thread::spawn(db_manage);
//
//     DatabaseQ {}
// }

/// Database, need changes to be async
#[inline]
pub fn answer_db(_value: &str) -> Option<String> {
    None
}
