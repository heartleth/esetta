use super::*;

pub fn deduce_rule<'h>((words_s, _word_t) :(&Vec<&'h str>, &Vec<&'h str>), part :&str, rules :&'h HashMap<&str, Vec<(Rule, Rule)>>, dictionary :&'h HashMap<&'h str, &str>)->std::result::Result<Vec<(&'h str, (Rule<'h>, Rule<'h>))>, ()> {
    if let Some(ruleset) = rules.get(part) {
        for (rule, _) in ruleset {
            let mut ret :Vec<(&'h str, (Rule<'h>, Rule<'h>))> = Vec::new();
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
                                        template.push(Template((i, &dictionary[word])));
                                        i += 1;
                                    }
                                    index += i;
                                    // println!("{}. {:?}", p, template);
                                    ret.push((p, (template, Vec::new())));
                                }
                                else {
                                    index += 1;
                                }
                            }
                            else {
                                println!("{} / {}", &words_s[index..].join(" "), p);
                                let w = match_phrase(&words_s[index..].to_vec(), p, &rules.iter().map(|(k, v)|(*k, v)).collect(), dictionary);
                                if let Ok(w) = w {
                                    index += w.0;
                                    template.push(Template((i, p)));
                                }
                                i = 1;
                                let next_word = words_s[index];
                                let next_pattern = &rule[progress + 1];
                                if !match next_pattern {
                                    Template((_, p)) => dictionary[next_word] == *p,
                                    Voca(v) => next_word == *v
                                } {
                                    if let Template((_, p)) = next_pattern {
                                        // if *p == "v" {
                                            for word in &words_s[index..] {
                                                if dictionary[word] == /*"v"*/ *p {
                                                    break;
                                                }
                                                else {
                                                    template.push(Template((i, dictionary[word])));
                                                    i += 1;
                                                }
                                            }
                                            index += i - 1;
                                        // }
                                    }
                                    else if let Voca(v) = next_pattern {
                                        for word in &words_s[index..] {
                                            if word == v {
                                                break;
                                            }
                                            else {
                                                template.push(Template((i, dictionary[word])));
                                                i += 1;
                                            }
                                        }
                                        index += i - 1;
                                    }
                                    // println!("{}. {:?}", p, template);
                                    ret.push((p, (template, Vec::new())));
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