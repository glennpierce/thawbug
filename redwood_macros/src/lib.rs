use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, ReturnType, Type};

#[proc_macro_attribute]
pub fn send(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract function components
    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;
    let fn_body = &input_fn.block;

    // Handle return type
    let fn_return_type = match &input_fn.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    // Generate function arguments for quote!
    let args = fn_args.iter().map(|arg| match arg {
        FnArg::Typed(pat_type) => {
            let pat = &pat_type.pat;
            let ty = &pat_type.ty;
            quote! { #pat: #ty }
        }
        FnArg::Receiver(_) => {
            quote! { self }
        }
    }).collect::<Vec<_>>(); // Collect into a vector to avoid moving issues

    // Convert collected arguments to quote format
    let args_quote = quote! { #(#args),* };

    // Generate the expanded code
    let expanded = quote! {
        #[cfg(feature = "ssr")]
        pub async fn #fn_name(#args_quote) -> #fn_return_type {
            #fn_body
        }

        #[cfg(not(feature = "ssr"))]
        pub async fn #fn_name(#args_quote) -> #fn_return_type {
            use send_wrapper::SendWrapper;

            let result = SendWrapper::new(async move {
                #fn_body
            }).await;

            result
        }
    };

    // Convert the expanded code into a TokenStream
    TokenStream::from(expanded)
}
