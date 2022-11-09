mod vector;

use vector::{Vector};

#[derive(Debug)]
struct MyStruct{
    id:i32
}
impl Drop for MyStruct{
    fn drop(&mut self) {
        println!("destroy MyStruct {id}",id=self.id);
    }
}
fn main() {

    let mut v = Vector::new();
    v.push(MyStruct{id:11});
    v.push(MyStruct{id:12});
    v.push(MyStruct{id:13});
    let c = &v[1];
    println!("{c:?}");

    let c =& mut v[1];
    c.id = 122;
    println!("{c:?}");

    println!("----------- list them");

    for i in &v{
        println!("{i:?}");
    };
    //v.pop();
    println!("exit");
}

