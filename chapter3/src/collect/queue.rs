#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T>{
    pub fn new(size : usize) -> Self{
        Queue {
            cap : size,
            data : Vec::with_capacity(size),
        }
    }

    pub fn enqueue(&mut self, item : T) -> Result<(), String>{
        if self.size() == self.cap {
            return Err("no space to enqueue".to_string());
        }

        self.data.insert(0, item);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.data.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len() 
    }
}

#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T>{
    pub fn new(size : usize) -> Self{
        Deque {
            cap : size,
            data : Vec::with_capacity(size),
        }
    }

    pub fn add_front(&mut self, item : T) -> Result<(), String>{
        if self.size() == self.cap {
            return Err("no space to add_front".to_string());
        }

        self.data.push(item);
        Ok(())
    }

    pub fn add_rear(&mut self, item : T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("no space to add_rear".to_string());
        }

        self.data.insert(0, item);
        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        }else{
            self.data.pop()
        }
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        }else{
            Some(self.data.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len() 
    }
}

pub fn hot_potato(names : Vec<&str>, num : usize) -> &str {
    let mut que = Queue::new(names.len());

    for name in names {
        que.enqueue(name);
    }


    while que.size() > 1 {
        for i in 0 .. num {
            let name = que.dequeue().unwrap();
            que.enqueue(name);
        }
        que.dequeue();
    }

    que.dequeue().unwrap()
}

pub fn pal_check(pal : &str) -> bool {
    let mut deque = Deque::new(pal.len());
    for c in pal.chars() {
        deque.add_front(c);
    }

    let mut is_pal = true;

    while deque.size() > 1 && is_pal {
        let front = deque.remove_front();
        let rear = deque.remove_rear();
        if front != rear {
            is_pal = false;
        }
    }
    
    is_pal
}