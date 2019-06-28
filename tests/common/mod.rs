use wavelet5::WaveletTree;
pub fn setup_string1() -> WaveletTree<char> {
    wavelet5::WaveletTree::create_tree("Hallo Ich bin ein Test für einen Satz".chars().into_iter())
}

pub fn setup_string() -> WaveletTree<char> {
    wavelet5::WaveletTree::create_tree("AsWDaaaaa aGDW!/(%§".chars().into_iter())
}
