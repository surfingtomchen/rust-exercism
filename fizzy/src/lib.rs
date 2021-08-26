/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    func: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(_matcher: fn(T) -> bool, _subs: S) -> Matcher<T> {
        Self {
            func: _matcher,
            subs: _subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        // unimplemented!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        iter.map(move |item| {
            let replaced = self
                .matchers
                .iter()
                .filter_map(|m| (m.func)(item.clone()).then(|| m.subs.clone()))
                .collect::<String>();
            if !replaced.is_empty() {
                replaced
            } else {
                item.to_string()
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: ToString + std::ops::Rem<Output = T> + PartialEq + Clone + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|i: T| i % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|i: T| i % 5.into() == 0.into(), "buzz"))
}
