use wavelet5::WaveletTree;
pub fn setup_string1() -> WaveletTree<char> {
    wavelet5::WaveletTree::create_tree("Hallo Ich bin ein Test für einen Satz".chars().into_iter())
}

pub fn setup_string() -> WaveletTree<char> {
    wavelet5::WaveletTree::create_tree("AsWDaaaaa aGDW!/(%§".chars().into_iter())
}

pub fn setup_ints() -> WaveletTree<u64> {
    let mut vec = Vec::new();
    for x in 0..1000000 {
        vec.push(x % 30);
    }
    wavelet5::WaveletTree::create_tree(vec.into_iter())
}
