struct MyStack {
    top: Option<Box<MyStackNode>>,
}

struct MyStackNode {
    val: i32,
    next: Option<Box<MyStackNode>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            top: None,
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.top = Some(Box::new(
            MyStackNode {
                val: x,
                next: self.top.take(),
            }
        ));
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        let curr = self.top.take();
        match curr {
            None => {
                self.top = None;
                -1
            },
            Some(node) => {
                self.top = node.next;
                node.val
            }
        }
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        if let Some(node) = &self.top {
            node.val
        } else {
            panic!("error");
        }
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        match self.top {
            None => true,
            _ => false,
        }
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("{:?}", s.pop());
    println!("{:?}", s.pop());
    println!("{:?}", s.pop());
}
