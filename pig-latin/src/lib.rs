pub fn translate(input: &str) -> String {
    let is_vowel = |c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
    let is_not_vowel_3 = |a, b, c| !is_vowel(a) && !is_vowel(b) && !is_vowel(c);
    let is_not_vowel_2 = |a, b| !is_vowel(a) && !is_vowel(b);

    input
        .split(' ')
        .map(|w| {
            let mut v: Vec<&str> = vec![];
            let temp: String;
            match w.chars().take(3).collect::<Vec<char>>()[..] {
                ['a', _, _] | ['e', _, _] | ['i', _, _] | ['o', _, _] | ['u', _, _] => {
                    v.push(w);
                    v.push("ay");
                }
                [_, 'q', 'u'] => {
                    v.push(&w[3..]);
                    v.push("squay");
                }
                ['x', 'r', _] | ['y', 't', _] => {
                    v.push(w);
                    v.push("ay");
                }
                ['q', 'u', _] => {
                    v.push(&w[2..]);
                    v.push("quay");
                }
                [c1, c2, c3] => {
                    if is_not_vowel_3(c1, c2, c3) && c3 != 'y' {
                        v.push(&w[3..]);
                        temp = [c1, c2, c3].iter().collect::<String>();
                        v.push(&temp);
                        v.push("ay");
                    } else if is_not_vowel_2(c1, c2) {
                        v.push(&w[2..]);
                        temp = [c1, c2].iter().collect::<String>();
                        v.push(&temp);
                        v.push("ay");
                    } else if !is_vowel(c1) {
                        v.push(&w[1..]);
                        temp = String::from(c1);
                        v.push(&temp);
                        v.push("ay");
                    }
                }
                _ => {}
            }
            v.join("")
        })
        .collect::<Vec<String>>()
        .join(" ")
}
