macro_rules! curry_help {
	($p:ident, $block:block) => {
	   move |$p:_| $block
   };
   ($p:ident, $($trailing:ident),+,$block:block)=>{
	  {
	   move |$p:_|{
		  curry_help![$($trailing),+,$block]
		}
	  }
   }
 }
 macro_rules! curry {
	 (|$($id:ident),+ | $block:block) => {
		 {
		  curry_help![$($id),+, $block]
		 }
	 };
 }
 fn main(){
	let f = curry!(|a,c,_e|{
	   println!("{a},{c}");
	});
	f(1)(2)(1.1);
 }