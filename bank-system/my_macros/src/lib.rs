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
            match storage.accounts.get_mut(&self.account) {
                Some(acc) => {
                    acc.balance += self.amount;
                    Ok(())
                }
                None => Err(TxError::InvalidAccount),
            }
        },
        "withdraw" => quote! {
            let balance = storage.accounts.get(&self.account)
                .ok_or(TxError::InvalidAccount)?
                .balance;
            if balance < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            storage.accounts.get_mut(&self.account).unwrap().balance -= self.amount;
            Ok(())
        },
        "transfer" => quote! {
            let from_balance = storage.accounts.get(&self.from)
                .ok_or(TxError::InvalidAccount)?
                .balance;
            if from_balance < self.amount {
                return Err(TxError::InsufficientFunds);
            }
            if !storage.accounts.contains_key(&self.to) {
                return Err(TxError::InvalidAccount);
            }
            storage.accounts.get_mut(&self.from).unwrap().balance -= self.amount;
            storage.accounts.get_mut(&self.to).unwrap().balance += self.amount;
            Ok(())
        },
        _ => quote! { compile_error!("unknown transaction kind") },
    };

    let expanded = quote! {
        impl Transaction for #name {
            fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
                #body
            }
        }
    };

    TokenStream::from(expanded)
}