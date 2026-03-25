use quote::{format_ident, quote};

/// Exposes a function as typst-wasm function with automatic serialization and deserialization.
/// The function must take exactly one argument and return a value.
/// The argument must implement [TryFrom]<[sertyp::Item]> and the return type must implement [Into]<[sertyp::Item]>.
///
/// Types you may find useful:
/// - [sertyp::Or]
/// - [sertyp::TypedArray]
/// - [sertyp::Pair]
/// - [sertyp::Result]
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
/// # Error Cascading
/// If the user function expects a type that does not implement [TryFrom]<[sertyp::Panic]>, the macro will automatically abort and create a traceable error message that includes the original error.
/// If the user function does implement [TryFrom]<[sertyp::Panic]> (e.g. [sertyp::Item] as input), it will be called normally with the panic as input argument.
#[proc_macro_attribute]
pub fn typst_func(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item: syn::ItemFn = syn::parse(item).unwrap();
    item.attrs.retain(|attr| !attr.path().is_ident("wasm_func"));
    let mut wrapper_sig = item.sig.clone();

    if item.sig.inputs.len() != 1 {
        return syn::Error::new_spanned(item.sig.inputs, "Function must have exactly one argument")
            .to_compile_error()
            .into();
    }
    match wrapper_sig.inputs.first_mut().unwrap() {
        syn::FnArg::Typed(pat_type) => {
            *pat_type = syn::parse_quote! { data: &[u8] };
        }
        syn::FnArg::Receiver(_) => {
            return syn::Error::new_spanned(&item.sig.inputs, "Function cannot take self argument")
                .to_compile_error()
                .into();
        }
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

    quote! {
        #[wasm_func]
        #wrapper_sig {
            let value = match sertyp::deserialize_cbor(data) {
                Ok(v) => {
                    let p: std::result::Result<sertyp::Panic, _> = v.clone().try_into();
                    match v.try_into() {
                        Ok(v) => v,
                        Err(e) => match p {
                            Ok(p) => {
                                sertyp::error!("Cascading Error", "[{} {}:{}:{}] failed because of previous error:\n{}: {}", stringify!(#orig_ident), file!(), line!(), column!(), p.ty, p.msg);
                            }
                            Err(_) => {
                                sertyp::error!("Type Conversion Error", "{}", &e);
                            }
                        }
                    }
                },
                Err(e) => {
                    sertyp::error!("Deserialization Error", "{}", &e);
                }
            };

            #item
            let result = #ident(value);
            match sertyp::serialize_cbor(&result.into()) {
                Ok(data) => data,
                Err(e) => {
                    sertyp::error!("Serialization Error", "{}", &e);
                }
            }
        }
    }
    .into()
}
