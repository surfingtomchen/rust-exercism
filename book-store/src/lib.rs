use std::cmp;
use std::collections::{HashMap, HashSet};

const BOOK_PRICE: u32 = 800;
const BOOK_DISCOUNT: [f32; 4] = [0.95, 0.9, 0.8, 0.75];

pub fn lowest_price(books: &[u32]) -> u32 {
    lowest_price_in_set(books.iter().fold(HashMap::new(), |mut h, b| {
        *h.entry(*b).or_insert(0) += 1;
        h
    }))
}

fn lowest_price_in_set(books_set: HashMap<u32, u32>) -> u32 {
    if books_set.len() == 0 {
        return 0;
    }

    let mut min = books_set.values().sum::<u32>() * BOOK_PRICE;

    (2..=5).for_each(|x| {
        let mut discount_suit = (1..=x).collect::<HashSet<_>>();

        let books_left = books_set
            .iter()
            .filter_map(|(book, qty)| {
                if discount_suit.contains(book) {
                    discount_suit.remove(book);
                    (qty > &1).then(|| (*book, *qty - 1))
                } else {
                    Some((*book, *qty))
                }
            })
            .collect();
        let discount_qty = x - discount_suit.len() as u32;

        match discount_qty {
            2..=5 => {
                min = cmp::min(
                    min,
                    (BOOK_PRICE as f32
                        * discount_qty as f32
                        * BOOK_DISCOUNT[(discount_qty - 2) as usize]) as u32
                        + lowest_price_in_set(books_left),
                )
            }
            _ => {}
        }
    });

    min
}
