use std::iter;

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    };
    let mut x = n;
    let mut v = (0..7)
        .into_iter()
        .map(|i| {
            let a = x % 1000;
            x = x / 1000;
            encode_less_than_1000(a, i as usize)
        })
        .filter(|x| *x != "")
        .collect::<Vec<String>>();
    v.reverse();
    v.join(" ").trim().to_string()
}

pub fn encode_less_than_1000(remainder: u64, base: usize) -> String {
    if remainder == 0 {
        return "".to_string();
    }

    let pre: Vec<&str> = vec![
        "",
        " one",
        " two",
        " three",
        " four",
        " five",
        " six",
        " seven",
        " eight",
        " nine",
        " ten",
        " eleven",
        " twelve",
        " thirteen",
        " fourteen",
        " fifteen",
        " sixteen",
        " seventeen",
        " eighteen",
        " nineteen",
    ];
    let tens: Vec<&str> = vec![
        "", "", " twenty", " thirty", " forty", " fifty", " sixty", " seventy", " eighty",
        " ninety",
    ];
    let hundred = iter::once("")
        .chain(iter::repeat(" hundred").take(9))
        .collect::<Vec<&str>>();
    let thousand_base: Vec<&str> = vec![
        "",
        " thousand",
        " million",
        " billion",
        " trillion",
        " quadrillion",
        " quintillion",
    ];

    let within_hundred = pre
        .iter()
        .map(|x| x.to_owned().to_owned())
        .chain((2..10).into_iter().flat_map(|x| {
            iter::repeat(tens[x])
                .take(10)
                .zip(pre.iter().take(10))
                .map(|(x, y)| {
                    if !y.is_empty() {
                        format!("{}-{}", x, y.trim())
                    } else {
                        x.to_string()
                    }
                })
        }))
        .collect::<Vec<String>>();

    format!(
        "{}{}{}{}",
        pre[(remainder / 100) as usize],
        hundred[(remainder / 100) as usize],
        within_hundred[(remainder % 100) as usize],
        thousand_base[base]
    )
    .trim()
    .to_string()
}
