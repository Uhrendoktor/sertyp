use quote::{format_ident, quote};

#[proc_macro_attribute]
pub fn typst_func(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item: syn::ItemFn = syn::parse(item).unwrap();
    item.attrs.retain(|attr| !attr.path().is_ident("wasm_func"));
    let mut wrapper_sig  = item.sig.clone();

    if item.sig.inputs.len() != 1 {
        return syn::Error::new_spanned(
            item.sig.inputs,
            "Function must have exactly one argument",
        )
        .to_compile_error()
        .into();
    }
    let input_type = match wrapper_sig.inputs.first_mut().unwrap() {
        syn::FnArg::Typed(pat_type) => {
            let old = pat_type.clone();
            *pat_type = syn::parse_quote! { data: &[u8] };

            match *old.ty {
                syn::Type::Path(path) => path,
                _ => {
                    return syn::Error::new_spanned(
                        &item.sig.inputs,
                        "Function argument must be a type path",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        syn::FnArg::Receiver(_) => {
            return syn::Error::new_spanned(
                &item.sig.inputs,
                "Function cannot take self argument",
            )
            .to_compile_error()
            .into();
        }
    };
    let match_pattern = if input_type.path.segments.last().unwrap().ident == "Item" {
        quote! { v }
    } else {
        let mut input_item_type =input_type.clone();
        input_item_type.path.segments.insert(
            input_item_type.path.segments.len()-1,
            syn::parse_quote!(Item)
        );
        quote! { #input_item_type(v) }
    };

    match &mut wrapper_sig.output {
        syn::ReturnType::Type(_, ty) => {
            *ty = syn::parse_quote! { Vec<u8> };
        }
        syn::ReturnType::Default => {
            return syn::Error::new_spanned(
                &item.sig.output,
                "Function must have a return type",
            )
            .to_compile_error()
            .into();
        }
    };

    item.sig.ident = format_ident!("__impl_{}", wrapper_sig.ident);
    let ident = &item.sig.ident;

    quote!{
        #[wasm_func]
        #wrapper_sig {
            let value = match deserialize_cbor(data) {
                Ok(#match_pattern) => v,
                Ok(other) => {
                    error!("Expected {}, found {}", #input_type::name(), other.type_name());
                }
                Err(e) => {
                    error!("Deserialization Error: {}", &e);
                }
            };

            #item
            let result = #ident(value);
            match serialize_cbor(&result.into()) {
                Ok(data) => data,
                Err(e) => {
                    error!("Serialization Error: {}", &e);
                }
            }
        }
    }.into()
}