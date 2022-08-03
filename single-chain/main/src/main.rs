use std::{cell::RefCell, rc::Rc, vec};

use single_chain::MyList;

fn main() {
    let mut l = MyList::List::new(1);
    l.push(2);
    l.push(3);
    let r = &mut l[1..];

    for item in r {
        println!("{}", item.borrow_mut().value);
    }

    l.push(4).push(5);
    println!("{:?}", l);
    println!("-------------------------------------");
    l.pop();
    println!("{:?}", l);
    println!("------------------------------------- {}", l.empty());
    l.pop();
    println!("{:?}", l);
    println!("------------------------------------- {}", l.empty());
    l.pop();
    println!("{:?}", l);
    println!("------------------------------------- {}", l.empty());

    println!("endl");
}
