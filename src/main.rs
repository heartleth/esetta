use std::collections::{ HashMap, HashSet };
use itertools::izip;
use std::fs;

fn main() {
    let original = fs::read_to_string("original.txt").unwrap();
    let original :Vec<&str> = original.split('\n').collect();
    let translated = fs::read_to_string("translated.txt").unwrap();
    let translated :Vec<&str> = translated.split('\n').collect();
    
    let mut vocab :HashMap<&str, Vec<&str>> = HashMap::new();
    let mut clear_vocab :HashSet<&str> = HashSet::new();

    for (s, d) in izip!(&original, &translated) {
        let words_s :Vec<&str> = s.split(' ').collect();
        let words_d :Vec<&str> = d.split(' ').collect();

        for word in words_s {
            if !vocab.contains_key(word) {
                let v :Vec<&str> = words_d.iter().filter(|v|!clear_vocab.contains(*v)).map(|e|*e).collect();
                vocab.insert(word, v);
            }
            else if vocab.get(word).unwrap().len() >= 2 {
                let candidates = vocab.get_mut(word).unwrap();
                candidates.retain(|x|words_d.contains(x)&&!clear_vocab.contains(x));
                if candidates.len() == 1 {
                    clear_vocab.insert(candidates[0]);
                }
            }
        }
    }
    for (k, v) in &vocab {
        if v.len() == 1{
            println!("{} = {:?}", k, v[0]);
        }
    }
}