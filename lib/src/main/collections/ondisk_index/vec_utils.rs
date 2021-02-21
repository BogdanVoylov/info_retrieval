pub fn chunkinfy(vec_len: usize, chunk_num: usize) -> Vec<usize> {
    let chunk_size: usize = vec_len / chunk_num;
    let mut res_vec: Vec<usize> = (vec![chunk_size])
                                .iter()
                                .cycle()
                                .take(chunk_num)
                                .map(|x| x.clone())
                                .collect();
    let last_chunk_size = vec_len - chunk_size*chunk_num;
    res_vec[chunk_num-1] = last_chunk_size+chunk_size;
    res_vec
}

pub fn remove_multiple<T>(source: &mut Vec<T>, indices_to_remove: &[usize]) -> Vec<T> {
    indices_to_remove.iter()
        .copied()
        .map(|i| source.swap_remove(i))
        .collect()
}
