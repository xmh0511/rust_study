fn show2<'a, T:Send+'a>(v:T) where 'a:'static{}

struct Data<T>{
	v:T
}
impl<T> Data<T>{
	fn show(self){}
}
impl<'a:'static> Data<&'a i32>{
	fn test(self){
		show2(self);
	}
}
fn main(){
	static I:i32 = 0;
	let d = Data{v:&I};
	d.test();
	let d2 = Data{v:0};
	d2.show();
}