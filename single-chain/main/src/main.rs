use single_chain::MyList;


fn main() {
	let mut l = MyList::List::new(10);
    l.push(2).push(3);
    println!("{:?}", l);
    println!("-------------------------------------");
    l.pop();
    println!("{:?}", l);
    println!("------------------------------------- {}",l.empty());
    l.pop();
    println!("{:?}", l);
    println!("------------------------------------- {}",l.empty());
    l.pop();
    println!("{:?}", l);
    println!("------------------------------------- {}", l.empty());
}
