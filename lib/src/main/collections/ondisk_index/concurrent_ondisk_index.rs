pub struct ConcurrentOndiskIndex {
    collection: Vec<String>,
}
const buff_size: usize = 536870912;
impl ConcurrentOndiskIndex {
    pub fn new(collection:Vec<String>) -> Self {
        Self {collection}
    }

    pub fn process_concurrent(&mut self) {

    }

    pub fn reduce(&mut self, filenames:Vec<String>) {

    }
}