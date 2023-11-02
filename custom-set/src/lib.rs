#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Eq + PartialEq + Clone + Ord> {
    set: Vec<T>,
}

impl<T: Eq + PartialEq + Clone + Ord> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut set = vec![];
        for i in _input {
            if !set.contains(i) {
                set.push(i.clone());
            }
        }

        set.sort_unstable();
        Self { set }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.set.contains(&_element) {
            self.set.push(_element);
        }
        self.set.sort_unstable();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        for v in &self.set {
            if !_other.set.contains(v) {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        for v in &_other.set {
            if self.set.contains(v) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let mut set = vec![];
        for v in &_other.set {
            if self.set.contains(v) {
                set.push(v.clone());
            }
        }
        set.sort_unstable();
        Self { set }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut set = vec![];
        for v in &self.set {
            if !_other.set.contains(v) {
                set.push(v.clone());
            }
        }
        set.sort_unstable();
        Self { set }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut set = self.set.clone();
        for v in &_other.set {
            if !set.contains(v) {
                set.push(v.clone());
            }
        }
        set.sort_unstable();
        Self { set }
    }
}
