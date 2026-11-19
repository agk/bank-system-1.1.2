// my_macros/src/lib.rs
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(FromSql)]
pub fn from_sql_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields: Vec<_> = if let syn::Data::Struct(data) = &input.data {
        data.fields.iter()
            .map(|f| f.ident.clone().unwrap())
            .collect()
    } else {
        panic!("FromSql работает только со структурами");
    };

    let assigns = fields.iter().map(|f| {
        quote! {
            #f: vals.next().unwrap().parse().expect("Не удалось распарсить поле"),
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn from_sql(sql: &str) -> Self {
                // Извлекаем значения из строки вида VALUES('a','b','c');
                let mut vals = sql
                    .split("VALUES(")
                    .nth(1)
                    .expect("Не найдено VALUES")
                    .trim_end_matches(");")
                    .split(',')
                    .map(|s| s.trim().trim_matches('\''))
                    .into_iter();

                Self {
                    #(#assigns)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ToSql)]
pub fn to_sql_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let name = &input.ident; // имя структуры, например User 
    
    let (field_names, field_values): (Vec<_>, Vec<_>) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.named.iter()
                .map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    (ident, quote! { self.#ident })
                    // field_names получит имя поля,
                    // field_values получит выражение self.имя
                })
                .unzip(),
            _ => panic!("ToSql работает только со структурами с именованными полями"),
        },
        _ => panic!("ToSql работает только со структурами"),
    };
    
    let expanded: TokenStream2 = quote! {
        impl #name {
            pub fn to_sql(&self, table: &str) -> String {
                // stringify! превращает идентификатор в строку на этапе компиляции
                let columns = vec![#(stringify!(#field_names)),*].join(", ");
                // format! формирует строковое значение каждого поля
                let values = vec![#(format!("'{}'", #field_values)),*].join(", ");
                format!("INSERT INTO {} ({}) VALUES ({});", table, columns, values)
            }
        }
    };

    TokenStream::from(expanded)
}
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