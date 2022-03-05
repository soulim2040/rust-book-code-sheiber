#[derive(Debug, Clone, PartialEq)]
pub struct HashMap<T> {
    size : usize,
    slot : Vec<usize>,
    data : Vec<T>,
}

impl<T : Clone + Default + PartialEq> HashMap<T> {
    pub fn new() -> Self {
        let size : usize = 11;
        let mut slot = Vec::with_capacity(size);
        let mut data = Vec::with_capacity(size);

        for _i in 0..size {
            slot.push(0);
            data.push(Default::default());
        }

        HashMap{
            size,
            slot,
            data,
        }
    }

    pub fn hash(&self, key : usize) -> usize {
        key % self.size
    }

    pub fn rehash(&self, pos : usize) -> usize {
        (pos + 1) % self.size
    }

    pub fn insert(&mut self, key : usize, value : T){
        if 0 == key { panic!("erro key must > 0");}

        let pos = self.hash(key);
        if self.slot[pos] == 0 {
            self.slot[pos] = key;
            self.data[pos] = value;
        }else {
            let mut next = self.rehash(pos);
            while self.slot[next] != 0 
                && key != self.slot[next] {
                    next = self.rehash(next);
                }

            if self.slot[next] == 0 {
                self.slot[next] = key;
                self.data[next] = value;
            }else{
                //这里为何是这样(就是如果有重key,直接覆盖)
                self.data[next] = value;
            }
        }
    }

    pub fn remove(&mut self, key : usize) -> Option<T> {
        if 0 == key { panic!("erro key must > 0");}

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            return None;
        }else if key == self.slot[pos] {
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            return data;
        }else{
            let mut data : Option<T> = None;
            let mut stop = false;
            let mut found = false;
            let mut curr = pos;

            while self.slot[curr] != key && !found && !stop {
                if key == self.slot[curr] {
                    found = true;
                    data = Some(self.data[curr].clone());
                    self.slot[curr] = 0;
                    self.data[curr] = Default::default();
                }else{
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }
            return data;
        }
    }

    pub fn get(&self, key : usize) -> Option<&T>{
        if 0 == key { panic!("erro key must > 0");}

        let pos = self.hash(key);
        let mut data : Option<&T> = None;
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        while self.slot[curr] != 0 && !found && !stop {
            if key == self.slot[curr] {
                found = true;
                data = self.data.get(curr);
            }else {
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }
        data
    }

    pub fn len(&self) -> usize {
        let mut length = 0;

        for d in self.slot.iter(){
            if *d != 0 {
                length += 1;
            }
        }

        length
    }

    pub fn contains(&self, key : usize) -> bool {
        if 0 == key { panic!("erro key must > 0");}

        self.slot.contains(&key)
    }
}