// use proc_macro::TokenStream;
// use quote::quote;

// #[proc_macro_derive(AnswerFn)]
// pub fn lambda(_item: TokenStream) -> TokenStream {
//     println!("{}", _item.to_string());

//     quote! {
//         fn answer() -> u32 { 42 }

//         // impl Relationship<i32> {
//         //     fn unwrap(self) -> i32 {
//         //         match self {
//         //             Relationship::Edge(t) => t,
//         //             Relationship::None => 0,
//         //             Relationship::Unloaded => panic!("Relationship is not loaded"),
//         //         }
//         //     }
//         // }
//     }
//     .into()
// }
