extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

#[proc_macro_derive(DisplayInnerSegment)]
pub fn display_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_inner_display(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

#[proc_macro_derive(DisplayOuterSegment)]
pub fn display_outer(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_outer_display(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

#[proc_macro_derive(DisplayEdifact)]
pub fn display_edifact(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_edifact(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

#[proc_macro_derive(DisplayEdifactSg)]
pub fn display_edifact_sg(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = generate_edifact_sg(&ast).unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}

fn generate_inner_display(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let output = gen_types(ast);
    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt<'x>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut str: Vec<String> = vec![];
                #(#output)*
                let joined = str.join(":");
                let joined = joined.trim_end_matches(":");
                write!(f, "{}", joined)
            }
        }
    })
}

fn generate_outer_display(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let output = gen_types(ast);
    let s = format_ident!("{}", name).to_string().to_uppercase();
    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt<'x>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut str: Vec<String> = vec![];
                str.push(#s.to_string());
                #(#output)*
                let joined = str.join("+");
                let joined = joined.trim_end_matches("+");
                if joined.len() > 3 {
                    write!(f, "{}", joined)
                }else{
                    write!(f, "")
                }
            }
        }
    })
}

fn generate_edifact(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let output = gen_types(ast);
    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt<'x>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut str: Vec<String> = vec![];
                #(#output)*
                // filter empty lines
                let str: Vec<String> = str
                    .iter()
                    .map(|v| v.clone())
                    .filter(|s| !s.is_empty())
                    .collect();
                let joined = str.join("'\n");
                write!(f, "{}'", joined)
            }
        }
    })
}

fn generate_edifact_sg(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let output = gen_types(ast);
    Ok(quote! {
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt<'x>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut str: Vec<String> = vec![];
                #(#output)*
                // filter empty lines
                let str: Vec<String> = str
                    .iter()
                    .map(|v| v.clone())
                    .filter(|s| !s.is_empty())
                    .collect();
                let joined = str.join("'\n");
                write!(f, "{}", joined)
            }
        }
    })
}

fn gen_types(ast: &DeriveInput) -> Vec<TokenStream> {
    let x = &ast.data;
    let mut output = vec![];
    match x {
        Data::Struct(s) => {
            let f = &s.fields;
            for o in f {
                let id = o.ident.clone().unwrap();
                let t = &o.ty;
                match t {
                    syn::Type::Path(p) => {
                        let s = &p.path.segments;
                        let w = &s.first().unwrap().ident;
                        let ty = w.to_string();
                        match ty.as_str() {
                            "Vec" => {
                                let ts = quote! {
                                    if self.#id.is_empty() {
                                        str.push("".to_string());
                                    }else{
                                        self.#id.iter().for_each(|x| str.push(format!("{}",x)));
                                    }
                                };
                                output.push(ts);
                            }
                            "Option" => {
                                let ts = quote! {
                                    str.push(self.#id.as_ref().map_or("".to_string(),|x| format!("{}",x)));
                                };
                                output.push(ts);
                            }
                            _ => {
                                let ts = quote! {
                                    str.push(format!("{}",self.#id));
                                };
                                output.push(ts);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    output
}
