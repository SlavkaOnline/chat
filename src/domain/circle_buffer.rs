use std::cmp::min;

pub struct CircleBuffer<T> {
    position: usize,
    len: usize,
    capacity: usize,
    buffer: Vec<T>
}

impl<T> CircleBuffer<T>{
    pub fn with_capacity(capacity: usize) -> CircleBuffer<T> {
        CircleBuffer {position: 0, len: 0, capacity: capacity, buffer: Vec::with_capacity(capacity)}
    }

    pub fn push(&mut self, item: T) {
        
        if self.len < self.capacity {

            self.buffer.push(item);
            self.len = min(self.capacity, self.len + 1);
            self.position += 1;
        
        } else {

            if self.position == self.capacity  {
                self.position = 0;
            } 
                
            self.buffer[self.position] = item;
            self.position += 1;
        
        }
    }

    pub fn iter(&self) -> CircleBufferIter<T> {
        CircleBufferIter {
            position: self.position,
            index: self.position,
            end: self.len,
            buffer: &self.buffer,
            circled: false
        }
    }
}

pub struct CircleBufferIter<'a, T: 'a> {
    index: usize,
    position: usize,
    buffer: &'a Vec<T>,
    end: usize,
    circled: bool
}


impl<'a, T> Iterator for CircleBufferIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {

        if self.index == self.end && !self.circled {
            self.circled = true;
            self.index = 1;
            return self.buffer.get(0);
        }

        if self.index == self.position && self.circled {
            return None;
        }

        let current = self.buffer.get(self.index);
        self.index += 1;
        return current;
    }
}

impl<'a, T: 'a> IntoIterator for &'a CircleBuffer<T> {
    type Item = &'a T;
    type IntoIter = CircleBufferIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::circle_buffer::CircleBuffer;

    #[test]
    fn push_part() {
        let mut buffer = CircleBuffer::<i32>::with_capacity(10);

        for i in 0..5 {
            buffer.push(i)
        }

        let expected = vec![0,1,2,3,4];
        assert_eq!(buffer.buffer, expected)
    }

    #[test]
    fn push_full() {
        let mut buffer = CircleBuffer::<i32>::with_capacity(5);

        for i in 0..5 {
            buffer.push(i)
        }

        let expected = vec![0,1,2,3,4];
        assert_eq!(buffer.buffer, expected)
    }

    #[test]
    fn push_overflow() {
        let mut buffer = CircleBuffer::<i32>::with_capacity(5);

        for i in 0..7 {
            buffer.push(i)
        }

        let expected = vec![5,6,2,3,4];
        assert_eq!(buffer.buffer, expected)
    }

    #[test]
    fn iner_part_push() {
        let mut buffer = CircleBuffer::<i32>::with_capacity(10);

        for i in 0..5 {
            buffer.push(i)
        }

        let mut arr = Vec::<i32>::new();

        for item in &buffer {
            arr.push(item.to_owned());
        }

        let expected = vec![0,1,2,3,4];
        assert_eq!(arr, expected)
    }


    #[test]
    fn iter_full_push() {
        let mut buffer = CircleBuffer::<i32>::with_capacity(5);

        for i in 0..5 {
            buffer.push(i)
        }

        let mut arr = Vec::<i32>::new();

        for item in &buffer {
            arr.push(item.to_owned());
        }

        let expected = vec![0,1,2,3,4];
        assert_eq!(arr, expected)
    }

    #[test]
    fn iter_overflow_push() {
        let mut buffer = CircleBuffer::<i32>::with_capacity(5);

        for i in 0..7 {
            buffer.push(i)
        }
        let mut arr = Vec::<i32>::new();

        for item in &buffer {
            arr.push(item.to_owned());
        }

        let expected = vec![2,3,4,5,6];
        assert_eq!(arr, expected)
    }
}