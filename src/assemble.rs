use super::*;

pub fn spread_phrase<'h, 'v>(tree :&Ready<'h, 'v>, vocab :&HashMap<&str, Vec<&'v str>>, dictionary :&'h HashMap<&'h str, &'v str>, i :usize)->(Rule<'h, 'v>, Vec<&'v str>) {
    if let Ready::Voca(v) = tree {
        if let Some(tv) = vocab.get(v) {
            if tv.len() == 1 {
                return (vec![Voca(tv[0])], Vec::new());
            }
            else {
                return (vec![Template((i, dictionary.get(v).unwrap_or(&"n").to_string()))], vec![v]);
            }
        }
        else {
            panic!();
        }
    }
    else if let Ready::Template((rule, params)) = tree {
        let mut i = i;
        let mut new_args :Vec<&'v str> = Vec::new();
        let mut ret :Rule<'h, 'v> = Vec::new();
        for pattern in &rule.1 {
            if let Voca(v) = pattern {
                if let Some(tv) = vocab.get(v) {
                    if tv.len() == 1 {
                        ret.push(Voca(tv[0]));
                    }
                    else {
                        ret.push(Template((i, dictionary[v].to_string())));
                        new_args.push(v);
                    }
                }
                else {
                    panic!()
                }
                i += 1;
            }
            else if let Template((num, _)) = pattern {
                let (child_rule, child_args) = spread_phrase(&params[&num], vocab, dictionary, i);
                ret.extend(child_rule);
                i += child_args.len();
                new_args.extend(child_args);
            }
        }
        (ret, new_args)
    }
    else {
        panic!();
    }
}