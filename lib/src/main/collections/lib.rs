pub mod single_file_index;
pub mod biword_index;
pub mod traits;
pub mod string_utils;
pub mod coors_index;
pub trait MultipleFileIndex {
    fn process_concurrent(&mut self, input_names: &Vec<String>, buff_size: usize);
    fn serialize(&self)->String;
    fn name(&self)->String;
}