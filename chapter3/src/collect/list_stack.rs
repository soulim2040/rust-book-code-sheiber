#[derive(Debug, Clone)]
struct Node<T> {
    data : T, 
    next : Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(data : T) -> Self {
        Node {
            data : data,
            next : None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stack<T> {
    size : usize,
    top : Link<T>,
}

impl<T : Clone> Stack<T> {
    pub fn new() -> Self {
        Stack{
            size : 0,
            top : None,
        }
    }

    pub fn push(&mut self, val : T) {
        let mut node = Node::new(val);
        node.next = self.top.take();

        self.top = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node|{
            self.size -= 1;
            self.top = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node|{
            &node.data
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }
}