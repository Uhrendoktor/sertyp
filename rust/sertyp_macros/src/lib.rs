use quote::{format_ident, quote};
use syn::parse_macro_input;

/// Exposes a function as a Typst WASM function with automatic serialization and deserialization.
/// The function may take zero or more arguments and must return a value.
/// Each argument must implement [TryFrom]<[sertyp::Item]> and the return type must implement [Into]<[sertyp::Item]>.
///
/// Types you may find useful:
/// - [sertyp::Or]
/// - [sertyp::TypedArray]
/// - [sertyp::Pair]
/// - [sertyp::auto_impl]
/// - [sertyp::auto_impl_str]
/// - [sertyp::auto_impl_func]
///
/// # Example
/// ```rust
/// use sertyp::typst_func;
///
/// //#[typst_func]
/// pub fn fibonacci<'a>(n: sertyp::Integer) -> Result<sertyp::Integer, sertyp::String<'a>> {
///     let n: i32 = n.try_into().map_err(|_| "Invalid integer range")?;
///
///     let (mut v0, mut v1) = (0, 1);
///     for _ in 0..n {
///         (v0, v1) = (v1, v0 + v1);
///     }
///
///     Ok(v1.into())
/// }
/// ```
///
/// # References
/// Sometimes it may be useful to keep the ownership of the input data within the macro instead of shipping it to the user function. This is helpful in cases where the return value references the input lifetimes. In this case, the `&` or `&mut` modifiers may be used to change the ownership semantics.
/// ```rust
/// use sertyp::typst_func;
///
/// //#[typst_func(&)]
/// pub fn not_owning<'a>(n: &'a sertyp::Integer) -> Result<&'a sertyp::Integer, sertyp::String<'a>> {
///     Ok(n)
/// }
/// ```
///
/// # Error Cascading
/// If the user function expects a type that does not implement [TryFrom]<[sertyp::Panic]>, the macro will automatically abort and create a traceable error message that includes the original error.
/// If the user function does implement [TryFrom]<[sertyp::Panic]> (e.g. [sertyp::Item] as input), it will be called normally with the panic as input argument.
#[proc_macro_attribute]
pub fn typst_func(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let modifier: proc_macro2::TokenStream = attr.into();

    let mut item: syn::ItemFn = parse_macro_input!(item);
    item.attrs.retain(|attr| !attr.path().is_ident("wasm_func"));
    let mut wrapper_sig = item.sig.clone();

    let inputs = match wrapper_sig
        .inputs
        .iter_mut()
        .enumerate()
        .map(|(i, input)| match input {
            syn::FnArg::Typed(pat_type) => {
                let ident = format_ident!("data{}", i);
                *pat_type = syn::parse_quote! { #ident: &[u8] };
                Ok(ident)
            }
            syn::FnArg::Receiver(_) => Err(syn::Error::new_spanned(
                &input,
                "Function cannot take self argument",
            )
            .to_compile_error()
            .into()),
        })
        .collect::<Result<Vec<_>, proc_macro::TokenStream>>()
    {
        Ok(inputs) => inputs,
        Err(e) => return e,
    };

    match &mut wrapper_sig.output {
        syn::ReturnType::Type(_, ty) => {
            *ty = syn::parse_quote! { Vec<u8> };
        }
        syn::ReturnType::Default => {
            return syn::Error::new_spanned(&item.sig.output, "Function must have a return type")
                .to_compile_error()
                .into();
        }
    };

    item.sig.ident = format_ident!("__impl_{}", wrapper_sig.ident);
    let orig_ident = &wrapper_sig.ident;
    let ident = &item.sig.ident;
    let modified_inputs = inputs.iter().map(|ident| quote! { #modifier #ident });

    quote! {
        #[wasm_minimal_protocol::wasm_func]
        #wrapper_sig {
            use sertyp::FromString;
            #(let #inputs = match sertyp::deserialize_cbor(#inputs) {
                Ok(v) => {
                    let mut p: std::result::Result<sertyp::Panic, _> = v.clone().try_into();
                    match v.try_into() {
                        Ok(v) => v,
                        Err(e) => match p {
                            Ok(mut p) => {
                                sertyp::feature_switch! {
                                    {
                                        sertyp::error!(sertyp::RawContent::from_string("Cascading Error"), "{} failed because of previous error:\n{}\n{:#?}", 
                                            format!("[{} {}:{}:{}]",  stringify!(#orig_ident), file!(), line!(), column!()), 
                                            &p.ty, 
                                            p.msg
                                        );
                                    },
                                    {
                                        use std::ops::DerefMut;
                                        match (*p.ty).deref_mut() {
                                            sertyp::Content::Text(t) => {
                                                *t = t.clone().weight(sertyp::TextWeight::Bold).into();
                                                t.text = sertyp::String::from(format!("{}\n", *t.text)).into(); 
                                            },
                                            _ => {}
                                        }
                                        return sertyp::serialize_cbor(&sertyp::Panic{
                                            ty: Box::<sertyp::Content>::new(sertyp::Text::from_string("Cascading Error").into()).into(),
                                            msg: Box::<sertyp::Content>::new(sertyp::Sequence::from(vec![
                                                sertyp::Link{
                                                    dest: Some(sertyp::LinkDestination::String(format!("file://{}:{}:{}", concat!(env!("CARGO_MANIFEST_DIR"), "/", file!()), line!(), column!()).into()).into()),
                                                    body: Some(sertyp::TypedItem::new(sertyp::Text::from_string(format!("[{} {}:{}:{}]",  stringify!(#orig_ident), file!(), line!(), column!())).into()))
                                                }.into(),
                                                sertyp::Content::from(sertyp::Text::from_string(" failed because of previous error:\n")),
                                                *p.ty.into_inner(),
                                                *p.msg.into_inner()
                                            ]).into()).into(),
                                        }.into()).unwrap();
                                    }
                                };
                            }
                            Err(_) => {
                                sertyp::error!(sertyp::ItemContent::from_string("Type Conversion Error"), "{}", &e);
                            }
                        }
                    }
                },
                Err(e) => {
                    sertyp::error!(sertyp::ItemContent::from_string("Deserialization Error"), "{}", &e);
                }
            };)*

            #item
            let result = #ident(#(#modified_inputs),*);
            match sertyp::serialize_cbor(&result.into()) {
                Ok(data) => data,
                Err(e) => {
                    sertyp::error!(sertyp::ItemContent::from_string("Serialization Error"), "{}", &e);
                }
            }
        }
    }
    .into()
}
