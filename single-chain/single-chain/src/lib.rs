pub mod MyList{

	use std::{
		cell::RefCell,
		rc::{Rc, Weak},
	};
	#[derive(Debug)]
	enum Next<T> {
		Some(Rc<RefCell<SingleDirectionChain<T>>>),
		NULL,
	}
	impl<T> Next<T> {
		fn new(v: Rc<RefCell<SingleDirectionChain<T>>>) -> Self {
			Self::Some(v)
		}
	}
	#[derive(Debug)]
	enum Before<T> {
		Some(Weak<RefCell<SingleDirectionChain<T>>>),
		NULL,
	}
	
	#[derive(Debug)]
	struct SingleDirectionChain<T> {
		value: T,
		next: Next<T>,
		before: Before<T>,
	}
	
	impl<T> SingleDirectionChain<T> {
		fn new(v: T, before: Before<T>) -> Self {
			Self {
				value: v,
				next: Next::<T>::NULL,
				before,
			}
		}
	}
	#[derive(Debug)]
	enum Current<T> {
		NULL,
		Some(Weak<RefCell<SingleDirectionChain<T>>>),
	}
	
	#[derive(Debug)]
	pub struct List<T> {
		begin: Next<T>,
		curr: Current<T>,
	}
	
	impl<T> List<T> {
	 	pub fn new(v: T) -> Self {
			let data = Self::construct_node(v, Before::<T>::NULL);
			Self {
				begin: Next::Some(data.0),
				curr: Current::Some(data.1),
			}
		}
	
		pub fn empty(&self) -> bool {
			if let Next::NULL = self.begin {
				true
			} else {
				false
			}
		}
	
		fn construct_node(
			v: T,
			before: Before<T>,
		) -> (
			Rc<RefCell<SingleDirectionChain<T>>>,
			Weak<RefCell<SingleDirectionChain<T>>>,
		) {
			let data = Rc::new(RefCell::new(SingleDirectionChain::new(v, before)));
			let current = Rc::downgrade(&data);
			(data, current)
		}
	
		pub fn push(&mut self, v: T) -> &mut Self {
			let curr_wrapper = &self.curr;
			if let Current::Some(current) = curr_wrapper {
				if let Some(x) = current.upgrade() {
					let mut point = x.borrow_mut();
					if let Next::<T>::NULL = point.next {
						let before = Before::Some(Rc::downgrade(&x));
						let data = Self::construct_node(v, before);
						point.next = Next::Some(data.0);
						self.curr = Current::Some(data.1);
					}
				};
			};
			self
		}
	
		pub fn pop(&mut self) {
			let curr_wrapper = &self.curr;
			if let Current::Some(current) = curr_wrapper {
				if let Some(x) = current.upgrade() {
					let current_node = x.borrow_mut();
					if let Before::Some(parent) = &current_node.before {
						if let Some(parent) = parent.upgrade() {
							let mut parent_node = parent.borrow_mut();
							parent_node.next = Next::NULL;
							self.curr = Current::Some(Rc::downgrade(&parent));
						}
					} else {
						self.begin = Next::NULL;
						self.curr = Current::NULL;
					};
				} else {
					panic!("unexpected error!");
				}
			} else {
				panic!("list is empty, cannot continue to pop!");
			}
		}
	}


}
