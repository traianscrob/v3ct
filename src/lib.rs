use core::ops::Deref;
use core::fmt::Display;

struct Node<T> where T:Sized{
    _data: Option<Box<T>>,
    _next: Option<Box<Node<T>>>,
    _prev: Option<Box<Node<T>>>,
}

pub struct V3ct<T> where T:Sized{
    _start: Option<Box<Node<T>>>,
    _last: Option<Box<Node<T>>>,
    _size: i32
}

impl <T> Node<T>{
    pub fn new(data: T, next: Option<Box<Node<T>>>, prev: Option<Box<Node<T>>>) -> Self{
        Self{
            _data: Some(Box::new(data)),
            _prev: prev,
            _next: next
        }
    }

    pub fn add_next(&mut self, data: T){
        self._next = Some(Box::new(Self::new(data, None, None)));
    }

    pub fn get_next(&self) -> Option<&Box<Self>>{
        match &self._next {
            None => None,
            Some(node) => Some(node)
        }
    }

    pub fn get_prev(&self) -> Option<&Box<Self>>{
        match &self._prev {
            None => None,
            Some(node) => Some(node)
        }
    }
}

impl<T> Deref for Node<T> where T:Sized{
    type Target = T;

    fn deref(&self) -> &T {
        &self._data.as_ref().unwrap()
    }
}

fn get_at<T>(start: &Box<Node<T>>, index: i32) -> Option<&Box<Node<T>>> where T:Sized{
    if index <= 0 {
        return Some(start);
    }
    
    match &start._next {
        None => {
            return None;
        },
        Some(node) => {
            return get_at(node, index - 1);
        }
    }
}

impl <T: Display> V3ct<T>{
    fn new() -> Self{
        Self{
            _size: 0,
            _start: None,
            _last: None
        }
    }

    pub fn push(&mut self, data: T){
        let old_start = std::mem::replace(&mut self._start, None);
        let new_start = Box::new(Node::new(data, old_start, None));

        self._start = Some(new_start);

        self._size += 1;
    }

    pub fn last(&mut self) -> Option<&T>{
        let first = self._start.as_ref();
        match first {
            None => None,
            Some(value) =>{
                let data = value._data.as_ref();
                match data {
                    None => None,
                    Some(da) => Some(&*da)
                }
            }
        }
    }

    pub fn first(&mut self) -> Option<&T>{
        let first = &self._start;
        match first {
            None => None,
            Some(value) =>{
                match get_at(value, self._size - 1) {
                    None => None,
                    Some(x) => {
                        let data = x._data.as_ref();
                        match data {
                            None => None,
                            Some(da) => Some(&*da)
                        }
                    }
                }
            }
        }
    }

    pub fn get(&mut self, index: i32) -> Option<&T>{
        let first = &self._start;
        match first {
            None => None,
            Some(node) =>{
                match get_at(node, self._size - index - 1) {
                    None => None,
                    Some(x) => {
                        let data = x._data.as_ref();
                        match data {
                            None => None,
                            Some(da) => Some(&*da)
                        }
                    }
                }
            }
        }
    }

    pub fn size(&self) -> i32{
        self._size
    }
}

#[cfg(test)]
mod tests {
    use crate::V3ct;

    #[test]
    fn it_works() {
        let mut vec = V3ct::<i32>::new();
        vec.push(10);
        vec.push(11);
        vec.push(12);
        vec.push(13);
        vec.push(14);
        vec.push(15);

        assert_eq!(6, vec.size());
        
        assert_eq!(10, *vec.first().unwrap());
        assert_eq!(15, *vec.last().unwrap());

        assert_eq!(11, *vec.get(1).unwrap());
        assert_eq!(12, *vec.get(2).unwrap());
        assert_eq!(13, *vec.get(3).unwrap());
        assert_eq!(14, *vec.get(4).unwrap());
    }
}
