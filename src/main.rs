use std::collections::{ HashMap, HashSet };
use itertools::izip;
use std::fs;

fn main() {
    let original = fs::read_to_string("original.txt").unwrap();
    let original :Vec<&str> = original.split('\n').collect();
    let translated = fs::read_to_string("translated.txt").unwrap();
    let translated :Vec<&str> = translated.split('\n').collect();
    let dictionary = fs::read_to_string("dictionary.txt").unwrap();
    let dictionary :HashMap<&str, &str> = dictionary.split('\n')
        .map(|x| {
            let mut k = x.trim().split('.');
            let part = k.next().unwrap();
            let word = k.last().unwrap();
            (word, part)
        })
        .collect();
    
    let mut rules :Vec<(&str, String, String)> = Vec::new();
    let mut vocab :HashMap<&str, Vec<&str>> = HashMap::new();
    let mut vocab_rev :HashMap<&str, (&str, &str)> = HashMap::new();
    let mut clear_vocab :HashSet<&str> = HashSet::new();
    let mut clear_sentence :HashSet<usize> = HashSet::new();

    let mut sentence_idx = 0;
    for (s, d) in izip!(&original, &translated) {
        let words_s :Vec<&str> = s.split('.').last().unwrap().split(' ').collect();
        let words_d :Vec<&str> = d.split(' ').collect();
        let part = s.split('.').next().unwrap();
        let mut matching_idx :Vec<usize> = vec![0; words_d.len()];
        let mut word_idx = 0;
        
        let template :Vec<String> = words_s.iter().enumerate().map(|(i, x)|format!("{{{{{}-{}}}}}", i, dictionary.get(x).unwrap_or(&"n"))).collect();
        let template = template.join(" ");
        
        for word in words_s {
            if !vocab.contains_key(word) {
                let v :Vec<&str> = words_d.iter().filter(|v|!clear_vocab.contains(*v)).map(|e|*e).collect();
                if v.len() == 1 {
                    clear_vocab.insert(v[0]);
                    vocab_rev.insert(v[0], (word, dictionary.get(word).unwrap_or(&"n")));
                }
                vocab.insert(word, v);
            }
            else if vocab.get(word).unwrap().len() >= 2 {
                let candidates = vocab.get_mut(word).unwrap();
                candidates.retain(|x|words_d.contains(x)&&!clear_vocab.contains(x));
                if candidates.len() == 1 {
                    clear_vocab.insert(candidates[0]);
                    vocab_rev.insert(candidates[0], (word, dictionary.get(word).unwrap_or(&"n")));
                }
            }
            else {
                let pair = &vocab[word][0];
                matching_idx[words_d.iter().position(|x|x==pair).unwrap()] = word_idx;
            }
            word_idx += 1;
        }
        for (k, v) in vocab.iter_mut() {
            if v.len() >= 2 {
                v.retain(|&x|!clear_vocab.contains(x));
                if v.len() == 1 {
                    clear_vocab.insert(v[0]);
                    vocab_rev.insert(v[0], (k, dictionary.get(k).unwrap_or(&"n")));
                }
            }
        }
        let mut template_t = String::new();
        let mut idx = 0;
        for word in &words_d {
            if let Some((_, part)) = vocab_rev.get(word) {
                if idx == 0 {
                    template_t = format!("{{{{{}-{}}}}}", matching_idx[idx], part);
                }
                else {
                    template_t = format!("{} {{{{{}-{}}}}}", template_t, matching_idx[idx], part);
                }
                idx += 1;
            }
            else {
                break;
            }
        }
        if idx == words_d.len() {
            clear_sentence.insert(sentence_idx);
            let template = (part, template, template_t);
            if !rules.contains(&template) {
                rules.push(template);
            }
        }
        sentence_idx += 1;
    }
    for (k, v) in &vocab_rev {
        println!("{}. {} = {}", v.1, v.0, k);
    }
    for (k, o, t) in &rules {
        println!("TMP: {}. {} => {}", k, o, t);
    }
}