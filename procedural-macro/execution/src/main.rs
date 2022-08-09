use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

use hello_macro_derive::sql;

use hello_macro_derive::myrouter;

#[derive(HelloMacro)]
struct Data {}

#[macro_export]
macro_rules! test_name {
    ($t:ty) => {
        impl $t {
            fn new() -> Self {
                Self {}
            }
        }
    };
}

#[macro_export]
macro_rules! test_identifier {
    ($id:ident) => {
        format!("{}", stringify!($id))
    };
}

test_name![Data];

#[macro_export]
macro_rules! define_variable {
    ($k:ident $name:ident = $initializer:expr ;) => {
        let $name = $initializer;
    };
}

#[myrouter(GET, "/")]
fn index() -> i32{
	println!("{},{}",method,path);
	10
}


macro_rules! generate_router {
	($p:vis fn $name:ident () $(-> $ret:ty)?  $block:block) => {
		$p fn $name (v:i32,v1:f64) $(->$ret)? {
             println!("{}, {}",v,v1);
			 $block
		  }
	};
}
generate_router! {
   fn testfun()->i32{
       0
   }
}

#[macro_export]
macro_rules! add_with {
	($($id:ident : $t:ty), *, $block:block) => {
		fn add_variadict($($id:$t,)*){
			$block
		}
	};
}

fn main() {
    Data::hello_macro();
    Data::new();
    let abc = 10;
    let s = test_identifier![abc];
    define_variable! {
        auto c = 1024 * 2;
    };
    println!("{}", c);
    let ff = sql!(Select * from user_tb where user_tb.id='1');
    println!("{}", ff);
    testfun(10, 20.1);
    index();
    add_with!(v0:f64, v1:i32 ,{
      println!("result is {}", v0 + v1 as f64);
    });
    add_variadict(10.0,1);
}
