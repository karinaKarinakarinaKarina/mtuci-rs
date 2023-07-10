struct Vector<T> {
    data: Box<[T]>,
    length: usize,
}

impl<T> Vector<T> {
    fn new() -> Self {
        Vector {
            data: Vec::new().into_boxed_slice(),
            length: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Vector {
            data: Vec::with_capacity(capacity).into_boxed_slice(),
            length: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.length == self.data.len() {
            self.resize(self.length + 1);
        }
        self.data[self.length] = value;
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;
            Some(std::mem::replace(&mut self.data[self.length], unsafe { std::mem::zeroed() }))
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            self.length -= 1;
            Some(std::mem::replace(&mut self.data[index], unsafe { std::mem::zeroed() }))
        } else {
            None
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            Some(&self.data[index])
        } else {
            None
        }
    }

    fn resize(&mut self, new_length: usize) {
        let mut new_data = vec![unsafe { std::mem::zeroed() }; new_length].into_boxed_slice();
        let copy_length = std::cmp::min(self.length, new_length);
        std::mem::swap(&mut self.data, &mut new_data);
        for i in 0..copy_length {
            self.data[i] = std::mem::replace(&mut new_data[i], unsafe { std::mem::zeroed() });
        }
        self.length = copy_length;
    }
}



fn main() {
    let mut vector = Vector::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    
    assert_eq!(vector.pop(), Some(3));
    assert_eq!(vector.get(1), Some(&2));
    
    vector.remove(0);
    assert_eq!(vector.get(0), Some(&2));
    
    vector.resize(10);
    assert_eq!(vector.get(9), None);
    assert_eq!(vector.get(10), None);
}
