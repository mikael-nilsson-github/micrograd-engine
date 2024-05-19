#![allow(unused)]
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Tanh,
}

#[derive(Debug)]
struct Node {
    value: f64,
    gradient: f64,
    operation: Option<Operation>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: f64, operation: Option<Operation>, left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            gradient: 0.0,
            operation,
            left,
            right,
        }))
    }

    fn backward(node: Rc<RefCell<Node>>, gradient: f64) {
        node.borrow_mut().gradient += gradient;
        if let Some(op) = node.borrow().operation {
            match op {
                Operation::Add => {
                    if let Some(left) = node.borrow().left.clone() {
                        Node::backward(left, gradient);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        Node::backward(right, gradient);
                    }
                },
                Operation::Mul => {
                    if let Some(left) = node.borrow().left.clone() {
                        let right_value = node.borrow().right.as_ref().unwrap().borrow().value;
                        Node::backward(left.clone(), gradient * right_value);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        let left_value = node.borrow().left.as_ref().unwrap().borrow().value;
                        Node::backward(right.clone(), gradient * left_value);
                    }
                },
                Operation::Tanh => {
                    let tanh_grad = 1.0 - node.borrow().value.powi(2);
                    let grad = gradient * tanh_grad;
                    if let Some(left) = node.borrow().left.clone() {
                        Node::backward(left, grad);
                    }
                }
            }
        }
    }
}

fn main() {
    ex4();
}

fn ex1() {
    let a = Node::new(2.0, None, None, None);
    let b = Node::new(-3.0, None, None, None);
    let c = Node::new(10.0, None, None, None);
    let e = Node::new(a.borrow().value * b.borrow().value, Some(Operation::Mul), Some(a.clone()), Some(b.clone()));
    let d = Node::new(e.borrow().value + c.borrow().value, Some(Operation::Add), Some(e.clone()), Some(c.clone()));
    let f = Node::new(-2.0, None, None, None);
    let loss = Node::new(d.borrow().value * f.borrow().value, Some(Operation::Mul), Some(d.clone()), Some(f.clone()));
    Node::backward(loss.clone(), 1.0);
    println!("grad a = {:?}", a.borrow().gradient);
    println!("grad b = {:?}", b.borrow().gradient);
    println!("grad c = {:?}", c.borrow().gradient);
    println!("grad d = {:?}", d.borrow().gradient);
    println!("grad e = {:?}", e.borrow().gradient);
    println!("grad f = {:?}", f.borrow().gradient);
    println!("grad loss = {:?}", loss.borrow().gradient);
}

fn ex2() {
    // Define constants
    let x1 = Node::new(2.0, None, None, None);
    let x2 = Node::new(0.0, None, None, None);
    let w1 = Node::new(-3.0, None, None, None);
    let w2 = Node::new(1.0, None, None, None);
    let b = 6.881373;
    let x1w1 = Node::new(x1.borrow().value * w1.borrow().value, Some(Operation::Mul), Some(x1.clone()), Some(w1.clone()));
    let x2w2 = Node::new(x2.borrow().value * w2.borrow().value, Some(Operation::Mul), Some(x2.clone()), Some(w2.clone()));
    let x1w1x2w2 = Node::new(x1w1.borrow().value + x2w2.borrow().value, Some(Operation::Add), Some(x1w1.clone()), Some(x2w2.clone()));
    let n = Node::new(x1w1x2w2.borrow().value + b, Some(Operation::Add), Some(x1w1x2w2.clone()), None);
    let result = Node::new(n.borrow().value.tanh(), Some(Operation::Tanh), Some(n.clone()), None);
    Node::backward(result.clone(), 1.0);
    println!("Gradient of x1: {:?}", x1.borrow().gradient);
    println!("Gradient of x2: {:?}", x2.borrow().gradient);
    println!("Gradient of w1: {:?}", w1.borrow().gradient);
    println!("Gradient of w2: {:?}", w2.borrow().gradient);
    println!("Gradient of x1w1: {:?}", x1w1.borrow().gradient);
    println!("Gradient of x2w2: {:?}", x2w2.borrow().gradient);
    println!("Gradient of x1w1x2w2: {:?}", x1w1x2w2.borrow().gradient);
    println!("Gradient of n: {:?}", n.borrow().gradient);
    println!("Gradient of b: {:?}", x1w1x2w2.borrow().gradient);
    println!("Gradient of o (result): {:?}", result.borrow().gradient);
}

fn ex3() {
    let a = Node::new(3.0, None, None, None);
    let b = Node::new(a.borrow().value + a.borrow().value, Some(Operation::Add), Some(a.clone()), Some(a.clone()));
    Node::backward(b.clone(), 1.0);
    println!("Gradient of a: {:?}", a.borrow().gradient);
    println!("Gradient of b: {:?}", b.borrow().gradient);
}

fn ex4() {
    let a = Node::new(-2.0, None, None, None);
    let b = Node::new(3.0, None, None, None);
    let d = Node::new(a.borrow().value * b.borrow().value, Some(Operation::Mul), Some(a.clone()), Some(b.clone()));
    let e = Node::new(a.borrow().value + b.borrow().value, Some(Operation::Add), Some(a.clone()), Some(b.clone()));
    let f = Node::new(d.borrow().value * e.borrow().value, Some(Operation::Mul), Some(d.clone()), Some(e.clone()));
    Node::backward(f.clone(), 1.0);
    println!("grad a: {:?}", a.borrow().gradient);
    println!("grad b: {:?}", b.borrow().gradient);
    println!("grad d: {:?}", d.borrow().gradient);
    println!("grad e: {:?}", e.borrow().gradient);
    println!("grad f: {:?}", f.borrow().gradient);
}
