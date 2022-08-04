use core::slice;
use std::{
    alloc::{alloc, dealloc, Layout},
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub struct Array<T> {
    ptr: *mut T,
    size: usize,
}

impl<T> Array<T> {
    pub fn new(size: usize) -> Self {
        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);
            Self {
                ptr: ptr as *mut T,
                size,
            }
        }
    }

    fn get_value(&self, index: usize) -> &mut T {
        if index >= self.size {
            panic!("outside the range of Array");
        }
        unsafe {
            let ptr = self.ptr;
            let ptr = ptr.offset(index as isize);
            let r = &mut *ptr;
            r
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::new::<i32>();
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_value(index)
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_value(index)
    }
}

impl<T> Index<std::ops::Range<usize>> for Array<T> {
    type Output = [T];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        let start = index.start;
        let end = index.end - 1;
        if start >= self.size {
            panic!("start outside the range of Array");
        }
        if start > end {
            panic!("invalid range");
        }
        if end >= self.size {
            panic!("end outside the range of Array");
        }
        unsafe { slice::from_raw_parts(self.ptr.add(start as usize), (end - start + 1) as usize) }
    }
}

impl<T> IndexMut<std::ops::Range<usize>> for Array<T> {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        let start = index.start;
        let end = index.end - 1;
        if start >= self.size {
            panic!("start outside the range of Array");
        }
        if start > end {
            panic!("invalid range");
        }
        if end >= self.size {
            panic!("end outside the range of Array");
        }
        unsafe {
            slice::from_raw_parts_mut(self.ptr.add(start as usize), (end - start + 1) as usize)
        }
    }
}

pub struct DataIter<'a, T> {
    begin: *mut T,
    end: *mut T,
    placeholder: PhantomData<&'a T>,
}

impl<'a, T> Iterator for DataIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.begin <= self.end {
                let r = &mut *self.begin;
                self.begin = self.begin.add(1);
                Some(r)
            } else {
                None
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;

    type IntoIter = DataIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            DataIter {
                begin: self.ptr,
                end: self.ptr.offset(self.size as isize - 1),
                placeholder: PhantomData::<&'a T> {},
            }
        }
    }
}

pub struct DataIterMut<'a, T> {
    begin: *mut T,
    end: *mut T,
    placeholder: PhantomData<&'a T>,
}

impl<'a, T> Iterator for DataIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.begin <= self.end {
                let r = &mut *self.begin;
                self.begin = self.begin.add(1);
                Some(r)
            } else {
                None
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a mut Array<T> {
    type Item = &'a mut T;

    type IntoIter = DataIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            DataIterMut {
                begin: self.ptr,
                end: self.ptr.offset(self.size as isize - 1),
                placeholder: PhantomData::<&'a T> {},
            }
        }
    }
}
