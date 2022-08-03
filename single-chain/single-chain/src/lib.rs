pub mod MyList {

    use std::{
        cell::RefCell,
        ops::Index,
        ops::{IndexMut, Range, RangeFrom},
        rc::{Rc, Weak},
    };
    #[derive(Debug)]
    pub enum Next<T> {
        Some(Rc<RefCell<SingleDirectionChain<T>>>),
        NULL,
    }
    impl<T> Next<T> {
        fn new(v: Rc<RefCell<SingleDirectionChain<T>>>) -> Self {
            Self::Some(v)
        }

        fn clone(&self) -> Self {
            match self {
                Self::Some(x) => Self::Some(Rc::clone(&x)),
                _ => Self::NULL,
            }
        }
    }
    #[derive(Debug)]
    enum Before<T> {
        Some(Weak<RefCell<SingleDirectionChain<T>>>),
        NULL,
    }

    #[derive(Debug)]
    pub struct SingleDirectionChain<T> {
        pub value: T,
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

    // impl<T> Drop for SingleDirectionChain<T> {
    //     fn drop(&mut self) {
    //         println!("drop SingleDirectionChain<T>");
    //     }
    // }
    #[derive(Debug)]
    enum Current<T> {
        NULL,
        Some(Weak<RefCell<SingleDirectionChain<T>>>),
    }

    #[derive(Debug)]
    pub struct List<T> {
        begin: Next<T>,
        curr: Current<T>,
        slice: Next<T>,
    }

    impl<T> List<T> {
        pub fn new(v: T) -> Self {
            let data = Self::construct_node(v, Before::<T>::NULL);
            Self {
                begin: Next::Some(data.0),
                curr: Current::Some(data.1),
                slice: Next::NULL,
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
                        if let Some(parentRf) = parent.upgrade() {
                            let mut parent_node = parentRf.borrow_mut();
                            parent_node.next = Next::NULL;
                            self.curr = Current::Some(Rc::downgrade(&parentRf));
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

    impl<T> Index<RangeFrom<usize>> for List<T> {
        fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
            panic!("should be mutable")
        }

        type Output = Next<T>;
    }

    impl<T> IndexMut<RangeFrom<usize>> for List<T> {
        fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
            let mut slice = self.begin.clone();
            for _ in 0..index.start {
                let p;
                match &slice {
                    Next::Some(x) => {
                        let current = x.borrow_mut();
                        p = current.next.clone();
                    }
                    Next::NULL => {
                        panic!("out of range");
                    }
                };
                slice = p;
            }
            self.slice = slice;
            &mut self.slice
        }
    }

	impl<T: Clone> Iterator for Next<T> {
        type Item = Rc<RefCell<SingleDirectionChain<T>>>;

        fn next(&mut self) -> Option<Self::Item> {
            let r = match self {
                Next::Some(x) => {
                    let p = x.borrow_mut();
                    (Some(Rc::clone(x)), p.next.clone())
                }
                Next::NULL => (None, Next::NULL),
            };
            *self = r.1;
            r.0
            //todo!()
        }
    }

    impl<T: Clone> IntoIterator for &List<T> {
        type Item = Rc<RefCell<SingleDirectionChain<T>>>;

        type IntoIter = Next<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.begin.clone()
        }
    }
}
