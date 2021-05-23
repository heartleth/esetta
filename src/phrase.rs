use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Phrase<'v> {
    Template((usize, String)), Voca(&'v str)
}
#[derive(Clone, Debug)]
pub enum Ready<'r, 'v> {
    Template((&'r (Rule<'r, 'v>, Rule<'r, 'v>), HashMap<usize, Ready<'r, 'v>>)), Voca(&'v str)
}
impl<'a, 'v> std::fmt::Display for Ready<'a, 'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ready::Template((rule, voca)) => {
                let mut s = String::new();
                for elem in &rule.0 {
                    if let Template((i, p)) = elem {
                        s += &format!(" ({}.{})", p, voca[&i])[..];
                    }
                    else if let Voca(v) = elem {
                        s += &format!(" {}", v)[..];
                    }
                }
                write!(f, "{}", s)
            },
            Ready::Voca(v) => write!(f, "{}", v),
        }
    }
}
pub use Phrase::*;
pub type Rule<'a, 'v> = Vec<Phrase<'v>>;

struct Candidate<'r, 'v> {
    params: HashMap<usize, Ready<'r, 'v>>,
    pair :&'r(Rule<'r, 'v>, Rule<'r, 'v>),
    rule :&'r Rule<'r, 'v>,
    progress :usize,
    virgin :bool,
    index: usize,
    alive: bool
}
impl<'r, 'v> Candidate<'r, 'v> {
    fn pattern(&'r self)->Option<&'r Phrase> {
        self.rule.get(self.progress)
    }
    fn next(&mut self) {
        self.progress += 1;
    }
    fn drop(&mut self) {
        self.alive = false;
    }
    fn update(&mut self, k: usize, v: Ready<'r, 'v>) {
        self.params.insert(k, v);
    }
}

pub fn match_phrase<'v, 'p>(phrase :&Vec<&'v str>, part :&str, rules :HashMap<String, Vec<&'p (Rule<'p, 'v>, Rule<'p, 'v>)>>, dictionary :&HashMap<&str, &str>)
    ->std::result::Result<(usize, Ready<'p, 'v>), ()> {
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
                if candidate.index <= word_idx || candidate.progress < candidate.rule.len() {
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
                                    if let Ok(e) = match_phrase(&phrase[..k].to_vec(), part, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|&x|x).collect();(x.0.to_string(), k)}).collect(), dictionary) {
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
                            let w = match_phrase(&phrase[word_idx..].to_vec(), &p[..], rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|&x|x).collect();(x.0.to_string(), k)}).collect(), dictionary)?;
                            let num = *num;
                            candidate.update(num, w.1);
                            candidate.index += w.0;
                        }
                        candidate.virgin = false;
                        candidate.next();
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
            let e = candidates[best_candidate].pair;
            return Ok((candidates[best_candidate].index, Ready::Template((e, vocabs))));
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