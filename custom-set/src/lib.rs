#[derive(Debug)]
pub struct CustomSet<T: PartialEq> {
    pub data: Vec<T>,
}

impl<T: PartialEq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }

    fn ne(&self, other: &Self) -> bool {
        self.data.len() != other.data.len()
            || (!self.difference(other).is_empty() || !other.difference(self).is_empty())
    }
}

impl<'a, T: PartialEq> CustomSet<T> {
    pub fn new(input: &[T]) -> Self
    where
        T: Clone,
    {
        let mut v: Vec<T> = vec![];
        input.into_iter().for_each(|x| v.push(x.to_owned()));
        Self { data: v }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(&element)
    }

    pub fn add(&mut self, element: T) {
        if self.contains(&element) {
            return;
        } else {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|x| other.data.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.iter().filter(|x| other.contains(*x)).count() == 0
    }

    pub fn intersection(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        let data = self
            .data
            .iter()
            .filter(|&x| other.contains(x))
            .cloned()
            .collect::<Vec<_>>();
        Self { data }
    }

    pub fn difference(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        let data = self
            .data
            .iter()
            .filter(|x| !other.contains(*x))
            .cloned()
            .collect::<Vec<_>>();
        Self { data }
    }

    pub fn union(&self, _other: &Self) -> Self
    where
        T: Clone,
    {
        let data = self
            .data
            .iter()
            .filter(|x| !_other.data.contains(*x))
            .chain(_other.data.iter())
            .cloned()
            .collect::<Vec<_>>();
        Self { data }
    }
}
