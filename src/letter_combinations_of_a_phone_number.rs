//https://leetcode.com/problems/letter-combinations-of-a-phone-number/

const LOOKUP: [[Option<char>; 4]; 8] = [
    /*2*/[Some('a'), Some('b'), Some('c'), None],
    /*3*/[Some('d'), Some('e'), Some('f'), None],
    /*4*/[Some('g'), Some('h'), Some('i'), None],
    /*5*/[Some('j'), Some('k'), Some('l'), None],
    /*6*/[Some('m'), Some('n'), Some('o'), None],
    /*7*/[Some('p'), Some('q'), Some('r'), Some('s')],
    /*8*/[Some('t'), Some('u'), Some('v'), None],
    /*9*/[Some('w'), Some('x'), Some('y'), Some('z')],
];

pub fn letter_combinations_2(digits: &str) -> Vec<String> {
    let first = digits.chars().nth(0).unwrap();
    //as ascii - number offset - index offset
    let index = first as usize - 48 - 2;
    let rest = if digits.len() > 1 {
        letter_combinations_2(&digits[1..])
    } else {
        vec![]
    };

    LOOKUP[index].iter().filter_map(|letter| {
        match letter {
            None => None,
            Some(letter) => {
                let mut s = letter.to_string();
                let res = if rest.len() > 0 {
                    rest.iter().map(|rest| {
                        let mut inner = s.clone();
                        inner.push_str(rest);
                        inner
                    }).collect()
                } else {
                    vec![s]
                };

                Some(res)
            }
        }
    }).flatten().collect()
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![]
    }

    letter_combinations_2(digits.as_str())
}