#[proc_macro]
pub fn curry_all(_: TokenStream) -> TokenStream {
	let mut result_arr = Vec::new();
    for e in 1..=2 {
        let mut type_arr = Vec::new();
        let mut constraint_type_arr = Vec::new();
        let mut variable_arr = Vec::new();
        for i in 0..e {
            type_arr.push(format!("T{i}"));
            constraint_type_arr.push(format!("T{i}:'a"));
            variable_arr.push(format!("t{i}"));
        }
        let mut fn_type = Vec::new();
        for it in &type_arr {
            fn_type.push(format!("FnOnce({it})"));
        }
        let mut Rtype = String::new();
        for ele in &fn_type {
            Rtype += &format!("Box<dyn {ele}->");
        }
        for (index, _) in fn_type.iter().enumerate() {
            if index == 0 {
                Rtype += &format!("U + 'a >");
            } else {
                Rtype += &format!(" + 'a >");
            }
        }
        fn_type.push("U".to_string());
        let trait_str = format!(
            "trait CurryLize{e}<'a,{},U>{{ fn to_curry(self)->{Rtype}; }}",
            type_arr.join(",")
        );
        // /println!("{trait_str}");
        let body = {
            let mut s = String::new();
            for (index, e) in type_arr.iter().enumerate() {
                s += &format!("Box::new(move |t{index}:{e}|{{");
            }
            s += &format!("self({})", variable_arr.join(","));
            for (_, _) in type_arr.iter().enumerate() {
                s += &format!("}})");
            }
            s
        };

        let impl_trait_str = format!(
            r#"impl<'a,F,{},U> CurryLize{e}<'a,{},U> for F 
		    where F: FnOnce({})->U + 'a{{
				fn to_curry(self)->{Rtype} {{
                    {body}
				}}
			}}
		"#,
            constraint_type_arr.join(","),
            type_arr.join(","),
            type_arr.join(",")
        );
        //println!("{impl_trait_str}");
		result_arr.push(trait_str);
		result_arr.push(impl_trait_str);
    }
	TokenStream::from_str(&result_arr.join("\n")).unwrap()
}