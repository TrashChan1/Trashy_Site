// Author: Trash
// Date: Mar 10, 2026
//
// This program models the input text in a map from a string to a binary search tree.
// The binary search tree represents the probability of any given word following the current word
// (current word being represented by the key string)
// This is using magic number 1_000_000_000 for convenience because it should give enough precision in
// most contexts.
// If "has" is followed by "any" 10% of the time and "a" 90% of the time, then the binary tree will
// build like:
// 10_000, any,
// 1_000_000_000, a,
// any number 10_000 and lower will match the key 10_000 in spit_out.
// any number between 1_000_000_000 and 10_000 witll match the key 100_000 in spit_out.
// This could also be solved with a vector for constant instead of logarithmic time complexity,
// but this has a much lower space usage due to our high dictionary size.

#![feature(btree_cursors)]
use std::collections::{HashMap, BTreeMap};
use std::ops::Bound;
use wasm_bindgen::prelude::*;
/*
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
*/
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hello world");
}

fn next_word(model: &HashMap<String, BTreeMap<usize, String>>, word: String) -> String {
    let i = fastrand::usize(..) % 1_000_000_000;
    if let Some(word_tree) = model.get(&word) { 
        if let Some((_, next_word)) = word_tree.lower_bound(Bound::Included(&i)).peek_next() {
            next_word.to_string()
        } else if let Some((_, next_word)) = word_tree.upper_bound(Bound::Included(&i)).peek_prev() {
            next_word.to_string()
        } else {
            "and".to_string()
        }
    } else {
        match model.keys().next() {
            Some(text) => text.to_string(),
            None => "\n".to_string(),
        }
    }
}
#[wasm_bindgen]
pub fn spit_out(model_text: String, word_count: usize) -> String {
    let model = &build_model(model_text.to_string());
    let mut word: String = match model.keys().next() {
        Some(text) => text.to_string(),
        None => "\n".to_string(),
    };
    let mut output: String = "".to_string();
    for _ in 0..word_count {
        word = next_word(model, word);
        output.push_str(&word);
        if &word != "\n" {
            output.push(' ');
        }
    }
    output
}

fn build_model(contents: String) -> HashMap<String, BTreeMap<usize, String>> {
    let mut map: HashMap<String, HashMap<String, usize>> = HashMap::new();

    let mut words = contents.split(&[' ', '\u{9}'][..]).filter(|&x| !x.is_empty());

    let mut prev_word = words.next().expect("should have had a word there").to_owned();
    while let Some(word) = words.next() {
        match map.get_mut(&prev_word) {
            Some(num_map) => {
                match num_map.get_mut(word) {
                    Some(num) => {*num+= 1;}
                    None => {num_map.insert(word.to_string(), 1);}
                }
            }
            None => {
                map.insert(prev_word.to_string(), HashMap::new());
                map.get_mut(&prev_word).expect("Should have inserted word here").insert(word.to_string(), 1);
            }
        }
        prev_word = word.to_string();
    }

    let mut model: HashMap<String, BTreeMap<usize, String>> = HashMap::new();
    for (key, inner_map) in &map {
        let mut total = 0;
        for (_, count) in inner_map {
            total += count;
        }
        let mut tree = BTreeMap::new();
        let mut prev_index = 0;
        for (word, count) in inner_map {
            let idx = count * 1_000_000_000 / total + prev_index;
            tree.insert(idx, word.to_owned());
            prev_index = idx;
        }
        let _ = inner_map.iter().map(|(word, count)| tree.insert(count * 1_000_000_000 / total + prev_index, word.to_owned()));
        model.entry(key.to_owned()).or_insert(tree);
    }
    model
}
