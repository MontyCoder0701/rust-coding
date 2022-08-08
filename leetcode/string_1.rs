//Defanging IP Address
pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

//Jewels and Stones
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .filter(|&stones| jewels.contains(stones))
        .count() as i32
}

//Maximum words in sentence
pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let mut wordvector = sentences.iter().map(|x| x.split(' ').count());
    let mut mostwords = wordvector.max().unwrap() as i32;
    mostwords
}
