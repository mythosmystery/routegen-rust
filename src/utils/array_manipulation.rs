use rand::Rng;

pub fn chunk_vec<T>(vector: Vec<f64>, chunk_size: usize) -> Vec<Vec<f64>> {
    let mut chunks: Vec<Vec<f64>> = Vec::new();
    let mut chunk: Vec<f64> = Vec::new();
    let mut i = 0;
    for value in vector {
        if i == chunk_size {
            chunks.push(chunk);
            chunk = Vec::new();
            i = 0;
        }
        chunk.push(value);
        i += 1;
    }
    chunks.push(chunk);
    chunks
}

pub fn swap(vector: &mut Vec<f64>, i: usize, j: usize) {
    let temp = vector[i];
    vector[i] = vector[j];
    vector[j] = temp;
}

pub fn get_random_int(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}
