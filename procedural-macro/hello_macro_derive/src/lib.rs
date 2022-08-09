use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let f = input.to_string();
    let mut r = String::new();
    for (index, ele) in f.chars().enumerate() {
        if ele != ' ' {
            r.push(ele as char);
        } else {
            r.push('&');
        }
    }
    let s = format!("\"{}\"", r.to_string());
    TokenStream::from_str(&s).unwrap()
}

#[proc_macro_attribute]
pub fn myrouter(v: TokenStream, v1: TokenStream) -> TokenStream {
    let s = v.to_string();
	let attributes:Vec<String> = s.split(",").map(|x|x.to_string()).collect();
    let func = v1.to_string();
    let pos = func.find("{").unwrap();
    let head = &func[..pos];
    let body = &func[pos..];
    let mut r = String::new() + head;
    r = r + "{";
    r = r + &format!(r#"let method="{}";"#, attributes[0]);
	r = r+ &format!(r#"let path={};"#, attributes[1]);
    r = r + body;
    r = r + "}";
	TokenStream::from_str(&r).unwrap()
    // let ss = quote! {
    //     fn index(){
    //         println!("{}", #r);
    //     }
    // };
    // ss.into()
}
