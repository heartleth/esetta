use super::*;

pub fn deduce_rule<'h, 'v>(words_s :&Vec<&'v str>, part :&str, rules :HashMap<String, Vec<&'h (Rule<'h, 'v>, Rule<'h, 'v>)>>, dictionary :&HashMap<&'h str, &str>)
    ->std::result::Result<(HashMap<usize, (String, (Rule<'h, 'v>, Rule<'h, 'v>))>, &'h (Rule<'h, 'v>, Rule<'h, 'v>)), ()> {

    if let Some(ruleset) = rules.get(part) {
        for pair in ruleset {
            let rule = &pair.0;
            let mut ret :HashMap<usize, (String, (Rule<'h, 'v>, Rule<'h, 'v>))> = HashMap::new();
            let mut index = 0;
            let mut progress = 0;
            for pattern in rule {
                match pattern {
                    Template((num, p)) => {
                        let mut template :Rule = Vec::new();
                        let mut i = 0;
                        if progress == rule.len() - 1 {
                            if index < words_s.len() - 1 {
                                for word in &words_s[index..] {
                                    template.push(Template((i, dictionary[word].to_string())));
                                    i += 1;
                                }
                                index += i;
                                ret.insert(*num, (p.to_string(), (template, Vec::new())));
                            }
                            else {
                                index += 1;
                            }
                        }
                        else {
                            let w = match_phrase(&words_s[index..].to_vec(), p, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|&x|x).collect();(x.0.to_string(), k)}).collect(), dictionary, 0);
                            let mut next_phrase_len = 0;
                            if let Ok(w) = w {
                                index += w.0;
                                next_phrase_len = w.0;
                                template.push(Template((i, p.to_string())));
                            }
                            i = 1;
                            let next_word = words_s[index];
                            let next_pattern = &rule[progress + 1];
                            if !match next_pattern {
                                Template((_, p)) => dictionary[next_word] == *p || next_phrase_len >= 2,
                                Voca(v) => next_word == *v
                            } {
                                if let Template((_, p)) = next_pattern {
                                    for word in &words_s[index..] {
                                        if let Ok(_) = match_phrase(&words_s[index+i-1..].to_vec(), p, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|&x|x).collect();(x.0.to_string(), k)}).collect(), dictionary, 0) {
                                            break;
                                        }
                                        else {
                                            template.push(Template((i - 1, dictionary[word].to_string())));
                                            i += 1;
                                        }
                                    }
                                    index += i - 1;
                                }
                                else if let Voca(v) = next_pattern {
                                    for word in &words_s[index..] {
                                        if word == v {
                                            break;
                                        }
                                        else {
                                            template.push(Template((i - 1, dictionary[word].to_string())));
                                            i += 1;
                                        }
                                    }
                                    index += i - 1;
                                }
                                ret.insert(*num, (p.to_string(), (template, Vec::new())));
                            }
                        }
                    },
                    Voca(v) => {
                        if words_s[index] == *v {
                            index += 1;
                        }
                    }
                };
                progress += 1;
            }
            if ret.len() > 0 {
                return Ok((ret, pair));
            }
        }
    }
    Err(())
}

pub fn deduce_pair_rule<'h, 'v>(words_t :&Vec<&'v str>, rule: &(Rule<'h, 'v>, Rule<'h, 'v>), deduced :&'h HashMap<usize, (String, (Rule<'h, 'v>, Rule<'h, 'v>))>, rules :HashMap<String, Vec<&(Rule<'h, 'v>, Rule<'h, 'v>)>>, dictionary :&HashMap<&'v str, &'h str>)
    ->std::result::Result<Vec<(String, (&'h Rule<'h, 'v>, Rule<'h, 'v>))>, ()> {
    let mut ret :Vec<(String, (&Rule<'h, 'v>, Rule<'h, 'v>))> = Vec::new();
    let mut index = 0;
    let mut progress = 0;
    for pattern in &rule.1 {
        match pattern {
            Template((num, p)) => {
                if let Some((_, (t, _))) = deduced.get(num) {
                    let mut template :Rule = Vec::new();
                    let mut i = 0;
                    if progress == rule.1.len() - 1 {
                        if index < words_t.len() - 1 {
                            let mut clear :HashSet<usize> = HashSet::new();
                            for word in &words_t[index..] {
                                if let Some(part) = dictionary.get(word) {
                                    for pattern in t {
                                        if let Template((i, p)) = pattern {
                                            if p == part {
                                                if !clear.contains(i) {
                                                    template.push(Template((*i, dictionary[word].to_string())));
                                                    clear.insert(*i);
                                                }
                                            }
                                        }
                                    }
                                }
                                else {
                                    template.push(Voca(word));
                                }
                                i += 1;
                            }
                            index += i;
                            ret.push((p.to_string(), (t, template)));
                        }
                        else {
                            index += 1;
                        }
                    }
                    else {
                        let w = match_phrase(words_t, p, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|&x|x).collect();(x.0.to_string(), k)}).collect(), &dictionary, 1);
                        let mut next_phrase_len = 0;
                        if let Ok(w) = w {
                            index += w.0;
                            next_phrase_len = w.0;
                            template.push(Template((i, p.to_string())));
                        }
                        i = 1;
                        let next_word = words_t[index];
                        let next_pattern = &rule.1[progress + 1];
                        if !match next_pattern {
                            Template((_, p)) => dictionary[next_word] == *p || next_phrase_len >= 2,
                            Voca(v) => next_word == *v
                        } {
                            if let Template(_) = next_pattern {
                                let mut clear :HashSet<usize> = HashSet::new();
                                let mut left = t.len();
                                for word in &words_t[index..] {
                                    // if let Ok(_) = match_phrase(&words_t[index+i-1..].to_vec(), p, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|&x|x).collect();(x.0.to_string(), k)}).collect(), &dictionary, 1) {
                                    if left == 0 {
                                        break;
                                    }
                                    else {
                                        if let Some(part) = dictionary.get(word) {
                                            for pattern in t {
                                                if let Template((i, p)) = pattern {
                                                    if p == part {
                                                        if !clear.contains(i) {
                                                            template.push(Template((*i, dictionary[word].to_string())));
                                                            clear.insert(*i);
                                                            left -= 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        else {
                                            template.push(Voca(word));
                                            left -= 1;
                                        }
                                        i += 1;
                                    }
                                }
                                index += i - 1;
                            }
                            else if let Voca(v) = next_pattern {
                                let mut clear :HashSet<usize> = HashSet::new();
                                for word in &words_t[index..] {
                                    if word == v {
                                        break;
                                    }
                                    else {
                                        let part = dictionary[word];
                                        for pattern in t {
                                            if let Template((i, p)) = pattern {
                                                if p == part {
                                                    if !clear.contains(i) {
                                                        template.push(Template((*i, dictionary[word].to_string())));
                                                        clear.insert(*i);
                                                    }
                                                }
                                            }
                                        }
                                        i += 1;
                                    }
                                }
                                index += i - 1;
                            }
                            ret.push((p.to_string(), (t, template)));
                        }
                    }
                }
            },
            Voca(v) => if words_t[index] != *v { return Err(()); } else { index += 1; }
        }
        progress += 1;
    }
    Ok(ret)
}