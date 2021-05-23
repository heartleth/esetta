use super::*;

pub fn deduce_rule<'h, 'v>((words_s, _word_t) :(&Vec<&'h str>, &Vec<&'h str>), part :&str, rules :&HashMap<String, Vec<(Rule<'h, 'v>, Rule)>>, dictionary :&'h HashMap<&'h str, &str>)->std::result::Result<Vec<(String, (Rule<'h, 'v>, Rule<'h, 'v>))>, ()> {
    if let Some(ruleset) = rules.get(part) {
        for (rule, _) in ruleset {
            let mut ret :Vec<(String, (Rule<'h, 'v>, Rule<'h, 'v>))> = Vec::new();
            let mut index = 0;
            let mut progress = 0;
            for pattern in rule {
                match pattern {
                    Template((_, p)) => {
                        if *p != "dfghdfghdfgh" {
                            let mut template :Rule = Vec::new();
                            let mut i = 0;
                            if progress == rule.len() - 1 {
                                if index < words_s.len() - 1 {
                                    for word in &words_s[index..] {
                                        template.push(Template((i, dictionary[word].to_string())));
                                        i += 1;
                                    }
                                    index += i;
                                    ret.push((p.to_string(), (template, Vec::new())));
                                }
                                else {
                                    index += 1;
                                }
                            }
                            else {
                                let w = match_phrase(&words_s[index..].to_vec(), p, rules.iter().map(|x|{let k:Vec<&(Rule, Rule)>=x.1.iter().map(|x|x).collect();(x.0.to_string(), k)}).collect(), dictionary);
                                if let Ok(w) = w {
                                    index += w.0;
                                    template.push(Template((i, p.to_string())));
                                }
                                i = 1;
                                let next_word = words_s[index];
                                let next_pattern = &rule[progress + 1];
                                if !match next_pattern {
                                    Template((_, p)) => dictionary[next_word] == *p,
                                    Voca(v) => next_word == *v
                                } {
                                    if let Template((_, p)) = next_pattern {
                                        for word in &words_s[index..] {
                                            if dictionary[word] == *p {
                                                break;
                                            }
                                            else {
                                                template.push(Template((i, dictionary[word].to_string())));
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
                                                template.push(Template((i, dictionary[word].to_string())));
                                                i += 1;
                                            }
                                        }
                                        index += i - 1;
                                    }
                                    ret.push((p.to_string(), (template, Vec::new())));
                                }
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
                return Ok(ret);
            }
        }
    }
    Err(())
}