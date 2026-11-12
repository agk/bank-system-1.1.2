use bank_system::{Name, Storage};
use bank_system::transaction::{Deposit, Transaction, Transfer, Withdraw};
use std::io::{self, BufRead, Write};

fn main() {
    // Загружаем состояние из файла (или создаём с нуля, если файла нет)
    let mut storage = Storage::load_data("balance.csv");

    println!("=== Bank CLI Utils ===");
    println!("Команды:");
    println!("  add <name> <balance>            - добавить пользователя");
    println!("  remove <name>                   - удалить пользователя");
    println!("  deposit <name> <amount>         - пополнить баланс");
    println!("  wd <name> <amount>              - снять со счёта");
    println!("  balance <name>                  - показать баланс");
    println!("  transfer <name> <name> <amount> - перевести средства");
    println!("  list                            - показать всех пользоватлей и их балансы");
    println!("  exit                            - выйти");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        // flush нужен, чтобы приглашение "> " появилось до того, как пользователь начнёт вводить
        print!("> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        // read_line возвращает 0 байт при EOF (например, Ctrl+D) — выходим из цикла
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "add" => {
                if args.len() != 3 {
                    println!("Пример: add John 100");
                    continue;
                }
                let name: Name = args[1].to_string();
                let balance: i64 = match args[2].parse() {
                    Ok(b) => b,
                    Err(_) => { println!("Сумма должна быть целым числом"); continue; }
                };
                if storage.add_user(name.clone()).is_some() {
                    let _ = storage.deposit(&name, balance);
                    println!("Пользователь '{}' добавлен с балансом {}", name, balance);
                    storage.save("balance.csv");
                } else {
                    println!("Пользователь '{}' уже существует", name);
                }
            }
            "remove" => {
                if args.len() != 2 {
                    println!("Пример: remove John");
                    continue;
                }
                let name = args[1].to_string();
                match storage.remove_user(&name) {
                    Some(_) => {
                        println!("Пользователь '{}' удалён", name);
                        storage.save("balance.csv");
                    }
                    None => println!("Пользователь '{}' не найден", name),
                }
            }
            "transfer" => {
                if args.len() != 4 {
                    println!("Пример: transfer Alice Bob 50");
                    continue;
                }
                let from = args[1].to_string();
                let to = args[2].to_string();
                let amount: i64 = match args[3].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };
                let tr = Transfer {
                    from: from.clone(),
                    to: to.clone(),
                    amount: amount,
                };
                // Применяем транзакцию 
                match tr.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: перевод {} -> {} на {}", from, to, amount);
                        storage.save("balance.csv");
                    }
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
                // Ваш код здесь 
            }
            "deposit" => {
                if args.len() != 3 {
                    println!("Пример: deposit John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => {
                        println!("Сумма должна быть числом");
                        continue;
                    }
                };

                let tx = Deposit {
                    account: name.clone(),
                    amount,
                };
                // Применяем транзакцию 
                match tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: депозит {} на {}", name, amount);
                        storage.save("balance.csv");
                    }
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
            } 
            "wd" => {
                if args.len() != 3 {
                    println!("Пример: withdraw John 100");
                    continue;
                }
                let name = args[1].to_string();
                let amount: i64 = match args[2].parse() {
                    Ok(a) => a,
                    Err(_) => { println!("Сумма должна быть целым числом"); continue; }
                };
                let tx = Withdraw {
                    account: name.clone(),
                    amount,
                };
                // Применяем транзакцию 
                match tx.apply(&mut storage) {
                    Ok(_) => {
                        println!("Транзакция: списание {} на {}", name, amount);
                        storage.save("balance.csv");
                    }
                    Err(e) => println!("Ошибка транзакции: {:?}", e),
                }
            }
            "balance" => {
                if args.len() != 2 {
                    println!("Пример: balance John");
                    continue;
                }
                let name = args[1].to_string();
                match storage.get_balance(&name) {
                    Some(b) => println!("Баланс '{}': {}", name, b),
                    None => println!("Пользователь '{}' не найден", name),
                }
            }
            "list" => {
                let mut all = storage.get_all();
                all.sort_by(|a, b| a.0.cmp(&b.0)); // сортируем по имени
                for (name, balance) in all {
                    println!("  {} : {}", name, balance);
                } 
            }
            "exit" => break,
            _ => println!("Неизвестная команда: '{}'", args[0]),
        }
    }

    println!("Выход. Все изменения сохранены.");
}
