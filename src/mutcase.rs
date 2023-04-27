use std::collections::VecDeque;
/// equivalent to queue_entry in afl
pub struct MutCase {
    fname : String,

}
impl MutCase {
}
pub struct MutCases {
    cases : VecDeque<MutCase>
}

impl MutCases {
    pub fn add(&mut self, path : String, size : usize) {
        unimplemented!()
    }
}