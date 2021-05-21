use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Phrase<'a> {
    Template((usize, &'a str)), Voca(&'a str)
}
#[derive(Clone, Debug)]
pub enum Ready<'a> {
    Template((&'a(Rule<'a>, Rule<'a>), HashMap<usize, Ready<'a>>)), Voca(&'a str)
}
pub use Phrase::*;
pub type Rule<'a> = Vec<Phrase<'a>>;

struct Candidate<'r> {
    params: HashMap<usize, Ready<'r>>,
    pair :&'r(Rule<'r>, Rule<'r>),
    rule :&'r Rule<'r>,
    progress :usize,
    virgin :bool,
    index: usize,
    alive: bool
}
impl<'r> Candidate<'r> {
    fn pattern(&'r self)->Option<&'r Phrase> {
        self.rule.get(self.progress)
    }
    fn next(&mut self) {
        self.progress += 1;
    }
    fn drop(&mut self) {
        self.alive = false;
    }
    fn update(&mut self, k: usize, v: Ready<'r>) {
        self.params.insert(k, v);
    }
}

pub fn match_phrase<'h>(phrase :&Vec<&'h str>, part :&str, rules :&'h HashMap<&str, Vec<(Rule, Rule)>>, dictionary :&HashMap<&str, &str>)
    ->std::result::Result<(usize, Ready<'h>), ()> {
    if let Some(ruleset) = rules.get(part) {
        let mut candidates :Vec<Candidate> = ruleset.iter().enumerate().map(|(i, x)| Candidate {
            pair: &x,
            rule: &x.0,
            progress: 0,
            virgin: true,
            index: i,
            params: HashMap::new(),
            alive: true
        }).collect();
        let mut word_idx = 0;
        for word in phrase {
            for candidate in &mut candidates {
                if candidate.index <= word_idx || candidate.progress == candidate.rule.len() {
                    let pattern = candidate.pattern().unwrap();
                    if let Voca(v) = pattern {
                        if word == v {
                            candidate.virgin = false;
                            candidate.index += 1;
                            candidate.next();
                        }
                        else {
                            candidate.drop();
                        }
                    }
                    else if let Template((num, p)) = pattern {
                        let p = *p;
                        if candidate.virgin && p == part {
                            let pattern = candidate.pattern().unwrap();
                            let mut k = 0;
                            let mut ok = false;
                            if let Voca(v) = pattern {
                                for word in phrase {
                                    k += 1;
                                    if word == v {
                                        ok = true;
                                        break;
                                    }
                                }
                                if ok {
                                    if let Ok(e) = match_phrase(&phrase[..k].to_vec(), part, rules, dictionary) {
                                        if e.0 != k {
                                            candidate.drop();
                                        }
                                        else {
                                            let num = *num;
                                            candidate.update(num, e.1);
                                        }
                                    }
                                    else {
                                        candidate.drop();
                                    }
                                }
                                else {
                                    candidate.drop();
                                }
                            }
                            else {
                                k = 1;
                                let num = *num;
                                candidate.update(num, Ready::Voca(phrase[0]));
                                candidate.index += k;
                            }
                        }
                        else {
                            let w = match_phrase(&phrase[word_idx..].to_vec(), p, rules, dictionary)?;
                            let num = *num;
                            candidate.update(num, w.1);
                            candidate.index += w.0;
                        }
                        candidate.next();
                        candidate.virgin = false;
                    }
                }
            }
            candidates.retain(|x| x.alive);
            word_idx += 1;
        }
        
        candidates.retain(|x| x.progress == x.rule.len());
        if candidates.len() > 0 {
            let mut best_candidate = 0;
            let mut i = 0;
            for candidate in &candidates {
                if candidate.index > candidates[best_candidate].index {
                    best_candidate = i;
                }
                i += 1;
            }
            let mut vocabs = HashMap::new();
            for (&k, v) in &candidates[best_candidate].params {
                vocabs.insert(k, v.clone());
            }
            return Ok((candidates[best_candidate].index, Ready::Template((candidates[best_candidate].pair, vocabs))));
        }
        else if part == "n" {
            if dictionary.get(phrase.first().unwrap()).unwrap_or(&"n") == &"n" {
                return Ok((1, Ready::Voca(phrase.first().unwrap())));
            }
            else {
                return Err(());
            }
        }
        else {
            if dictionary.get(phrase.first().unwrap()) == Some(&part) {
                return Ok((1, Ready::Voca(phrase.first().unwrap())));
            }
            else {
                return Err(());
            }
        }
    }
    else if part == "n" {
        if dictionary.get(phrase.first().unwrap()).unwrap_or(&"n") == &"n" {
            return Ok((1, Ready::Voca(phrase.first().unwrap())));
        }
        else {
            return Err(());
        }
    }
    else {
        if dictionary.get(phrase.first().unwrap()) == Some(&part) {
            return Ok((1, Ready::Voca(phrase.first().unwrap())));
        }
        else {
            return Err(());
        }
    }
}