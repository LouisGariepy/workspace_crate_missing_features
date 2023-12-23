use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    syn::parse_macro_input!(item as syn::ItemEnum);
    "".parse().unwrap()
}
