pub struct CircularBuffer<T> {
    data: Vec<T>,
    cap: usize,
    head: usize,
    rear: usize,
    size: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Default + std::fmt::Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![T::default(); capacity],
            cap: capacity,
            head: 0,
            rear: 0,
            size: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.size == self.cap
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.data[self.rear] = _element;
        self.rear = (self.rear + 1) % self.cap;
        self.size += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let ele = self.data[self.head].clone();
        self.head = (self.head + 1) % self.cap;
        self.size -= 1;
        Ok(ele)
    }

    pub fn clear(&mut self) {
        self.rear = 0;
        self.head = 0;
        self.data = vec![T::default(); self.cap];
        self.size = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        if !self.is_full() {
            self.size += 1;
            self.data[self.rear] = _element;
            self.rear = (self.rear + 1) % self.cap;
        } else {
            self.data[self.head] = _element;
            self.head = (self.head + 1) % self.cap;
        }
    }
}
