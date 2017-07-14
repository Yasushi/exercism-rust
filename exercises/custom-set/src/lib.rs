
#[derive(Eq, PartialEq, Debug)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: Ord + Copy,
{
    pub fn new(items: Vec<T>) -> Self {
        let mut r = Self { items: vec![] };
        for i in items {
            r.add(i);
        }
        r
    }

    pub fn add(&mut self, item: T) {
        if !self.contains(&item) {
            self.items.push(item);
            self.items.sort();
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.items.iter().all(|i| other.contains(i))
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        !self.items.iter().any(|i| other.contains(i))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut r = CustomSet::new(vec![]);
        for &i in self.items.iter() {
            if other.contains(&i) {
                r.add(i);
            }
        }
        r
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut r = CustomSet::new(vec![]);
        for &i in self.items.iter() {
            if !other.contains(&i) {
                r.add(i);
            }
        }
        r
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut r = CustomSet::new(vec![]);
        for &i in self.items.iter() {
            r.add(i);
        }
        for &i in other.items.iter() {
            r.add(i);
        }
        r
    }
}
