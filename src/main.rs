use std::collections::{ HashMap, HashSet };
use itertools::izip;
use std::fs;

mod phrase;
use phrase::*;
mod assemble;
use assemble::*;
mod deduce;
use deduce::*;

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
    
    let mut rules :HashMap<String, Vec<(Rule, Rule)>> = HashMap::new();
    let mut vocab :HashMap<&str, Vec<&str>> = HashMap::new();
    let mut vocab_rev :HashMap<&str, (&str, String)> = HashMap::new();
    let mut clear_vocab :HashSet<&str> = HashSet::new();
    let mut clear_sentence :HashSet<usize> = HashSet::new();

    let mut sentence_idx = 0;
    for (s, d) in izip!(&original, &translated) {
        let words_s :Vec<&str> = s.split('.').last().unwrap().split(' ').collect();
        let words_d :Vec<&str> = d.split(' ').collect();
        let part = s.split('.').next().unwrap();
        let mut matching_idx :Vec<usize> = vec![0; words_d.len()];
        let mut word_idx = 0;
        
        let template :Rule = words_s.iter().enumerate().map(|(i, x)| Template((i, dictionary.get(x).unwrap_or(&"n").to_string()))).collect();
        
        for word in words_s {
            if !vocab.contains_key(word) {
                let v :Vec<&str> = words_d.iter().filter(|v|!clear_vocab.contains(*v)).map(|e|*e).collect();
                if v.len() == 1 {
                    clear_vocab.insert(v[0]);
                    if let Some(p) = dictionary.get(word) {
                        vocab_rev.insert(v[0], (word, p.to_string()));
                    }
                    else {
                        dictionary.insert(word, "n");
                        vocab_rev.insert(v[0], (word, "n".to_string()));
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
                        vocab_rev.insert(candidates[0], (word, p.to_string()));
                    }
                    else {
                        dictionary.insert(word, "n");
                        vocab_rev.insert(candidates[0], (word, "n".to_string()));
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
                        vocab_rev.insert(v[0], (k, p.to_string()));
                    }
                    else {
                        dictionary.insert(k, "n");
                        vocab_rev.insert(v[0], (k, "n".to_string()));
                    }
                }
            }
        }
        let mut template_t :Rule = Vec::new();
        let mut idx = 0;
        for word in &words_d {
            if let Some((_, part)) = vocab_rev.get(word) {
                template_t.push(Template((matching_idx[idx], part.to_string())));
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
                rules.insert(part.to_string(), vec![template]);
            }
        }
        sentence_idx += 1;
    }
    
    for (s, d) in izip!(&original, &translated) {
        let words_s :Vec<&str> = s.split('.').last().unwrap().split(' ').collect();
        let words_d :Vec<&str> = d.split(' ').collect();
        let part = s.split('.').next().unwrap();
        if !clear_sentence.contains(&sentence_idx) {
            if let Ok((_, template)) = &match_phrase(&words_s, part, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|x|x).collect();(x.0.to_string(), k)}).collect(), &dictionary, 0) {
                let rule = spread_phrase(template, &vocab, &dictionary, 0);
                let mut i = 0;
                for pattern in rule.0 {
                    if let Template((n, p)) = pattern {
                        let word = words_d[i];
                        let rule = rule.1[n];
                        let p = String::from(p);
                        vocab_rev.insert(word, (rule, p));
                    }
                    i += 1;
                }
            }
        }
        if let Ok(v) = deduce_rule(&words_s, part, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|x|x).collect();(x.0.to_string(), k)}).collect(), &dictionary) {
            let w = deduce_pair_rule(&words_d, v.1, &v.0, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|x|x).collect();(x.0.to_string(), k)}).collect(), &vocab_rev.iter().map(|(k,(_,p))|(*k,&p[..])).collect());
            if let Ok(freshmen) = w {
                for (p, (o, t)) in freshmen {
                    if let Some(v) = rules.get_mut(&p) {
                        let c = (o.clone(), t.clone());
                        if !v.contains(&c) {
                            v.push(c);
                        }
                    }
                    else {
                        rules.insert(p.clone(), vec![(o.clone(), t.clone())]);
                    }
                }
                
                if !clear_sentence.contains(&sentence_idx) {
                    if let Ok((_, template)) = &match_phrase(&words_s, part, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|x|x).collect();(x.0.to_string(), k)}).collect(), &dictionary, 0) {
                        let rule = spread_phrase(template, &vocab, &dictionary, 0);
                        let mut i = 0;
                        for pattern in rule.0 {
                            if let Template((n, p)) = pattern {
                                let word = words_d[i];
                                let rule = rule.1[n];
                                let p = String::from(p);
                                vocab_rev.insert(word, (rule, p));
                            }
                            i += 1;
                        }
                    }
                }
            }
        }
        sentence_idx += 1;
    }
    
    for (k, v) in &vocab_rev {
        println!("{}. {} = {}", v.1, v.0, k);
    }
    if translated.iter().map(|s|s.split(' ').map(|x|vocab_rev.contains_key(x)).fold(true, |a, b| a && b)).fold(true, |a, b| a && b) {
        println!("올클리어!");
    }
}