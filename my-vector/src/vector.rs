use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

pub struct Vector<T>{
    ptr: Option<* mut T>,
    size:u32,
    layout:std::alloc::Layout,
    pos:u32
}

impl<T> Vector<T>{
    pub fn new()->Self{
        let size = 2;
        let layout = std::alloc::Layout::array::<T>(size).unwrap();
        let raw = unsafe{std::alloc::alloc(layout.clone())};
        Vector{
            ptr:Some(raw as * mut T),
            size:size as u32,
            layout,
            pos:0
        }
    }
    pub fn push(& mut self,v:T){
        match self.ptr {
            None => todo!(),
            Some(ptr)=>{
                if self.pos < self.size{
                    unsafe {
                        let ptr =  ptr.add(self.pos as usize);
                        std::ptr::write(ptr,v);
                        self.pos+=1;
                    }
                }else{
                    //println!("realloc");
                    unsafe {
                        let layout = std::alloc::Layout::array::<T>((self.size * 2) as usize).unwrap();
                        let new_ptr = std::alloc::alloc(layout.clone()) as * mut T;
                        for old in 0..self.pos as usize {
                            let from_old = std::ptr::read(ptr.add(old));
                            std::ptr::write(new_ptr.add(old), from_old);
                        }
                        std::ptr::write(new_ptr.add(self.pos as usize), v);
                        self.pos+=1;
                        let old_layout = self.layout.clone();
                        self.ptr = Some(new_ptr);
                        self.size = self.size * 2;
                        self.layout = layout;
                        std::alloc::dealloc(ptr as * mut u8,old_layout);
                        //println!("realloc over");
                    }
                }
            }
        }
    }

    pub fn pop(& mut self)->T{
        match self.ptr {
            None  => todo!(),
            Some(ptr)=>{
                self.pos-=1;
                return unsafe{std::ptr::read(ptr.add(self.pos as usize))};
            }
        }
    }

    pub fn get(&self,index:usize)->&T{
        match self.ptr {
            None=>todo!(),
            Some(ptr)=>{
                if index >= self.pos as usize{
                    panic!("out of exist data")
                }
                return unsafe {
                  &*ptr.add(index)
                };
            }
        }
    }

    pub fn get_mut(& mut self, index:usize)->& mut T{
        match self.ptr {
            None=>todo!(),
            Some(ptr)=>{
                if index >= self.pos as usize{
                    panic!("out of exist data")
                }
                return unsafe {
                    & mut *ptr.add(index)
                };
            }
        }
    }
}

impl<T> Index<usize> for Vector<T>{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<usize> for Vector<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
       self.get_mut(index)
    }
}

impl<'a,T> IntoIterator for &'a Vector<T>{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a,T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.ptr{
            None=>todo!(),
            Some(ptr)=>{
                let t = unsafe{std::slice::from_raw_parts(ptr,self.pos as usize)};
                return t.into_iter();
            }
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        match self.ptr{
            Some(ptr)=>{
                for i in 0..self.pos as usize{
                    unsafe {std::ptr::drop_in_place(ptr.add(i));}
                }
              unsafe { std::alloc::dealloc(ptr as * mut u8,self.layout)};
            }
            None=>todo!()
        }
    }
}

impl<T:Debug> Debug for Vector<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.ptr{
            None =>todo!(),
            Some(ptr)=>{
                f.write_str("[ ").unwrap();
                unsafe {
                    for i in 0..self.pos{
                        let v = &*ptr.add(i as usize);
                        v.fmt(f).unwrap();
                        f.write_str(" , ").unwrap();
                    };
                }
                f.write_str(" ]").unwrap();
                return Ok(());
            }
        }
    }
}




