pub mod single_file_index;
pub mod kgram_index;
#[path = "./process_strategies/lib.rs"]
pub mod process_strategies;
pub mod string_utils;
pub mod prefix_trie;
pub mod prefix_index;
pub mod permuterm_index;
pub trait MultipleFileIndex {
    fn process_concurrent(&mut self, input_names: &Vec<String>, buff_size: usize);
    fn serialize(&self)->String;
    fn name(&self)->String;
}