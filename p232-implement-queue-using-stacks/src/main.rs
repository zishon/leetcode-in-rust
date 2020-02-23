struct MyQueue {
    stack: Vec<i32>,
    length: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            stack: vec![],
            length: 0,
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.stack.insert(0, x);
        self.length += 1;
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.length -= 1;
        self.stack.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        self.stack[self.length as usize - 1]
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.length == 0
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
fn main() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);

    println!("{:?}", queue.peek());
    println!("{:?}", queue.pop());
    println!("{:?}", queue.empty());
    println!("{:?}", queue.pop());
}
