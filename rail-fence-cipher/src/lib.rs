pub struct RailFence(isize);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self(rails as isize)
    }

    pub fn encode(&self, text: &str) -> String {
        let step = (2 * self.0 - 2) as isize;

        (0..self.0)
            .into_iter()
            .flat_map(|row| {
                text.chars().zip(0..).filter_map(move |(c, i)| {
                    ((i - row) % step == 0 || (i + row) % step == 0).then(|| c)
                })
            })
            .collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let step = (2 * self.0 - 2) as usize;
        let count = cipher.chars().count();
        let mut row = 0;
        let mut first = true;
        let mut r: Vec<_> = cipher
            .chars()
            .scan(0, |i, ch| {
                let result = Some((*i, ch));
                let mut delta = 0;
                while delta == 0 {
                    delta = if first { step - 2 * row } else { 2 * row };
                    first = !first;
                }
                *i += delta;
                if *i >= count {
                    row += 1;
                    *i = row;
                    first = true;
                }
                result
            })
            .collect();
        r.sort();
        r.iter().map(|(_, ch)| ch).collect()
    }
}
