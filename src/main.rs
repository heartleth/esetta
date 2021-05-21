use std::collections::{ HashMap, HashSet };
use itertools::izip;
use std::fs;

mod phrase;
use phrase::*;
mod assemble;
use assemble::*;

fn main() {
    let original = fs::read_to_string("original.txt").unwrap();
    let original :Vec<&str> = original.split('\n').collect();
    let translated = fs::read_to_string("translated.txt").unwrap();
    let translated :Vec<&str> = translated.split('\n').collect();
    let dictionary = fs::read_to_string("dictionary.txt").unwrap();
    let mut dictionary :HashMap<&str, &str> = dictionary.split('\n')
        .map(|x| {
            let mut k = x.trim().split('.');
            let part = k.next().unwrap();
            let word = k.last().unwrap();
            (word, part)
        })
        .collect();
    
    let mut rules :HashMap<&str, Vec<(Rule, Rule)>> = HashMap::new();
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
        
        let template :Rule = words_s.iter().enumerate().map(|(i, x)| Template((i, dictionary.get(x).unwrap_or(&"n")))).collect();
        
        for word in words_s {
            if !vocab.contains_key(word) {
                let v :Vec<&str> = words_d.iter().filter(|v|!clear_vocab.contains(*v)).map(|e|*e).collect();
                if v.len() == 1 {
                    clear_vocab.insert(v[0]);
                    if let Some(p) = dictionary.get(word) {
                        vocab_rev.insert(v[0], (word, p));
                    }
                    else {
                        dictionary.insert(word, "n");
                        vocab_rev.insert(v[0], (word, "n"));
                    }
                    let pair = &v[0];
                    matching_idx[words_d.iter().position(|x|x==pair).unwrap()] = word_idx;
                }
                vocab.insert(word, v);
            }
            else if vocab.get(word).unwrap().len() >= 2 {
                let candidates = vocab.get_mut(word).unwrap();
                candidates.retain(|x|words_d.contains(x)&&!clear_vocab.contains(x));
                if candidates.len() == 1 {
                    clear_vocab.insert(candidates[0]);
                    if let Some(p) = dictionary.get(word) {
                        vocab_rev.insert(candidates[0], (word, p));
                    }
                    else {
                        dictionary.insert(word, "n");
                        vocab_rev.insert(candidates[0], (word, "n"));
                    }
                    let pair = &vocab[word][0];
                    matching_idx[words_d.iter().position(|x|x==pair).unwrap()] = word_idx;
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
                    if let Some(p) = dictionary.get(k) {
                        vocab_rev.insert(v[0], (k, p));
                    }
                    else {
                        dictionary.insert(k, "n");
                        vocab_rev.insert(v[0], (k, "n"));
                    }
                }
            }
        }
        let mut template_t :Rule = Vec::new();
        let mut idx = 0;
        for word in &words_d {
            if let Some((_, part)) = vocab_rev.get(word) {
                template_t.push(Template((matching_idx[idx], part)));
                idx += 1;
            }
            else {
                break;
            }
        }
        if idx == words_d.len() {
            clear_sentence.insert(sentence_idx);
            let template = (template, template_t);
            if let Some(v) = rules.get_mut(part) {
                if !v.contains(&template) {
                    v.push(template);
                }
            }
            else {
                rules.insert(part, vec![template]);
            }
        }
        sentence_idx += 1;
    }
    
    sentence_idx = 0;
    for (s, d) in izip!(&original, &translated) {
        if !clear_sentence.contains(&sentence_idx) {
            let words_s :Vec<&str> = s.split('.').last().unwrap().split(' ').collect();
            let words_d :Vec<&str> = d.split(' ').collect();
            let part = s.split('.').next().unwrap();
            let rule = spread_phrase(&match_phrase(&words_s, part, &rules, &dictionary).unwrap().1, &vocab, &dictionary, 0);

            let mut i = 0;
            for pattern in rule.0 {
                if let Template((n, p)) = pattern {
                    vocab_rev.insert(words_d[i], (rule.1[n], p));
                }
                i += 1;
            }
        }
        sentence_idx += 1;
    }
    for (k, v) in &vocab_rev {
        println!("{}. {} = {}", v.1, v.0, k);
    }
}