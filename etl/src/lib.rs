use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let init: BTreeMap<char, i32> = BTreeMap::new();
    h.iter().fold(init, |mut btm, (i, vc)| {
        for c in vc {
            btm.insert(c.to_ascii_lowercase(), *i);
        }
        btm
    })
}
