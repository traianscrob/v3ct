use core::ops::Deref;

struct Node<T: Sized + Copy>{
    _data: Option<Box<T>>,
    _next: Option<Box<Node<T>>>,
}

pub struct V3ct<T: Sized + Copy>{
    _start: Option<Box<Node<T>>>,
    _size: i32
}

impl <T: Copy> Node<T>{
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self{
        Self{
            _data: Some(Box::new(data)),
            _next: next
        }
    }

    pub fn get_next(&self) -> Option<&Box<Self>>{
        match &self._next {
            None => None,
            Some(node) => Some(node)
        }
    }

    pub fn get_data(&self) -> Option<&Box<T>>{
        self._data.as_ref()
    }

    pub fn add_next(&mut self, data: T){
        self._next = Some(Box::new(Self::new(data, None)));
    }
}

impl<T:Sized + Copy> Deref for Node<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self._data.as_ref().unwrap()
    }
}

fn get_next<T:Sized + Copy>(start: &mut Box<Node<T>>, level: i32) -> Option<&mut Box<Node<T>>>{
    if level <= 0 {
        return Some(start);
    }
    
    match &mut start._next {
        None => {
            return None;
        },
        Some(node) => {
            return get_next(node, level - 1);
        }
    }
}

impl <T: Copy> V3ct<T>{
    pub fn new() -> Self{
        Self{
            _size: 0,
            _start: None
        }
    }

    pub fn pop(&mut self) -> Option<T>{
        let start = self._start.take().unwrap();
        self._start = start._next;

        self._size -= 1;

        match &start._data {
            None => None,
            Some(value) => {
                Some(*value.deref())
            }
        }
    }

    pub fn push(&mut self, data: T){
        let new_start = Box::new(Node::new(data, self._start.take()));
        self._start = Some(new_start);

        self._size += 1;
    }

    pub fn queue(&mut self, data: T){
        match &mut self._start {
            None => {
                self.push(data);
            },
            Some(node) => {
                match get_next(node, self._size - 1) {
                    None => {},
                    Some(value) => {
                        (&mut *value).add_next(data);

                        self._size += 1;
                    }
                }
            }
        }
    }

    pub fn end(&mut self) -> Option<&T>{
        let start = &self._start;
        match start {
            None => None,
            Some(_) =>{
                self.get(self._size - 1)
            }
        }
    }

    pub fn start(&mut self) -> Option<&T>{
        let start = &self._start;
        match start {
            None => None,
            Some(node) =>{
                match node.get_data() {
                    None => None,
                    Some(da) => Some(&*da)
                }
            }
        }
    }

    pub fn get(&mut self, index: i32) -> Option<&T>{
        let first = &mut self._start;
        match first {
            None => None,
            Some(node) =>{
                match get_next(node, index) {
                    None => None,
                    Some(x) => {
                        match x.get_data() {
                            None => None,
                            Some(data) => Some(data.deref())
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

    #[derive(Copy, Clone)]
    struct User<'a>{
        name: &'a str,
        address: &'a str,
        age: i8,
    }

    impl<'a> User<'a>{
        pub fn new(name: &'a str, address: &'a str, age: i8) -> Self{
            Self{
                name,
                address,
                age
            }
        }
    }

    #[test]
    fn struct_works() {
        let mut vec = V3ct::<User>::new();
        vec.queue(User::new("User 1", "Address 1", 21));
        vec.queue(User::new("User 2", "Address 2", 21));
        vec.queue(User::new("User 3", "Address 3", 21));
        vec.queue(User::new("User 4", "Address 4", 21));

        vec.push(User::new("User 5", "Address 5", 21));
        vec.push(User::new("User 6", "Address 6", 21));

        assert_eq!(6, vec.size());

        assert_eq!("User 6", &*vec.start().unwrap().name);
        assert_eq!("User 6", &*vec.pop().unwrap().name);

        assert_eq!(5, vec.size());
        assert_eq!("User 5", &*vec.start().unwrap().name);

        assert_eq!("User 4", &*vec.end().unwrap().name);
    }

    #[test]
    fn it_works() {
        let mut vec = V3ct::<i32>::new();
        vec.push(10);
        vec.push(11);

        vec.queue(12);
        vec.queue(13);

        vec.push(14);

        assert_eq!(5, vec.size());
        
        assert_eq!(14, *vec.start().unwrap());
        assert_eq!(13, *vec.end().unwrap());

        assert_eq!(14, *vec.get(0).unwrap());
        assert_eq!(11, *vec.get(1).unwrap());
        assert_eq!(10, *vec.get(2).unwrap());
        assert_eq!(12, *vec.get(3).unwrap());
        assert_eq!(13, *vec.get(4).unwrap());
    }
}
