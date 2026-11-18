// my_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro]
pub fn say_hello(input: TokenStream) -> TokenStream {
    // Ожидаем строковый литерал
    let msg = parse_macro_input!(input as syn::LitStr);

    let expanded = quote! {
        println!("{}", #msg);
    };

    expanded.into()
}


// #[proc_macro_derive(Describable)]
// pub fn derive_describable(input: TokenStream) -> TokenStream {
//     // Шаг 1: разбираем входной TokenStream в удобную структуру
//     let input = parse_macro_input!(input as DeriveInput);

//     // Шаг 2: берём имя структуры
//     let name = input.ident;

//     // Шаг 3: генерируем реализацию
//     let expanded = quote! {
//         impl Describable for #name {
//             fn describe(&self) -> String {
//                 format!("Я — структура {}", stringify!(#name))
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }

#[proc_macro_derive(Describable)]
pub fn derive_describable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let field_names: Vec<_> = if let syn::Data::Struct(data) = &input.data {
        if let syn::Fields::Named(fields) = &data.fields {
            fields.named.iter()
                .map(|f| f.ident.as_ref().unwrap().clone())
                .collect()
        } else { vec![] }
    } else { vec![] };

    let expanded = quote! {
        impl Describable for #name {
            fn describe(&self) -> String {
                let fields = vec![
                    #( format!("{}: {:?}", stringify!(#field_names), self.#field_names) ),*
                ];
                format!("{} {{ {} }}", stringify!(#name), fields.join(", "))
            }
        }
    };

    TokenStream::from(expanded)
}

// #[proc_macro_derive(Transaction)]
// pub fn derive_transaction(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;

//     let expanded = quote! {
//         impl Transaction for #name {
//             fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
//                 *storage.accounts
//                     .entry(self.account.clone())
//                     .or_insert(0) += self.amount;
//                 Ok(())
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }

#[proc_macro_derive(Transaction, attributes(transaction))]
pub fn transaction_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Читаем атрибут #[transaction("...")]
    // По умолчанию — deposit
    let mut kind = "deposit";

    for attr in &input.attrs {
        if attr.path().is_ident("transaction") {
            if let Ok(meta) = attr.parse_args::<syn::LitStr>() {
                let val = meta.value();
                if val == "withdraw" {
                    kind = "withdraw";
                } else if val == "transfer" {
                    kind = "transfer";
                }
            }
        }
    }

    // Генерируем тело apply в зависимости от kind
    let body = match kind {
        "deposit" => quote! {
            *storage.accounts.entry(self.account.clone()).or_insert(0) += self.amount;
        },
        "withdraw" => quote! {
            let bal = storage.accounts.entry(self.account.clone()).or_insert(0);
            if *bal < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            *bal -= self.amount;
        },
        "transfer" => quote! {
            let from_bal = storage.accounts.entry(self.from.clone()).or_insert(0);
            if *from_bal < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            *from_bal -= self.amount;
            *storage.accounts.entry(self.to.clone()).or_insert(0) += self.amount;
        },
        _ => panic!("Неизвестный тип транзакции: {}", kind),
    };

    let expanded = quote! {
        impl Transaction for #name {
            fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
                #body
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}