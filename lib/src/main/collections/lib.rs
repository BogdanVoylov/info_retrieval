pub mod single_file_index;
pub mod kgram_index;
#[path = "./process_strategies/lib.rs"]
pub mod process_strategies;
pub mod string_utils;
pub mod permuterm_index;
#[path = "./ondisk_index/lib.rs"]
pub mod ondisk_index;
pub mod parallel_utils;
pub trait MultipleFileIndex {
    fn process_concurrent(&mut self, input_names: &Vec<String>, buff_size: usize);
    fn serialize(&self)->String;
    fn name(&self)->String;
}