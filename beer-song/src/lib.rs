const MAX: u32 = 100;

pub fn verse(n: u32) -> String {
    let bottle_str = |n: u32| if n % MAX == 0 { "no more".to_string() } else { (n % MAX).to_string() };

    let begin_bottle_str = |n: u32| if n == 0 { "No more".to_string() } else { n.to_string() };

    let s = |n| if n % MAX == 1 { "" } else { "s" };

    let take_str = |n| if n == 0 { "Go to the store and buy some more" } else if n == 1 { "Take it down and pass it around" } else {
        "Take one down and pass it around"
    };

    format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\n{}, {} bottle{} of beer on the wall.\n",
            begin_bottle_str(n),
            s(n),
            bottle_str(n),
            s(n),
            take_str(n),
            bottle_str(n + MAX - 1),
            s(n + MAX - 1))
}

pub fn sing(start: u32, end: u32) -> String {
    let bigger = if start > end { start } else { end };
    let smaller = if start > end { end } else { start };
    let v = (smaller..=bigger).rev().map(|n| verse(n)).collect::<Vec<String>>();
    v.join("\n")
}
