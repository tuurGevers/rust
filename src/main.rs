use std::io;

#[derive(Debug, Clone, Copy)]
enum Currency {
    Eur,
    Usd,
    Btc,
}

#[derive(Debug)]
struct CurrencyValue {
    currency: Currency,
    amount: f64,
}

fn main() {
    loop {
        let mut currency = String::new();
        let mut to_currency = String::new();
        println!("default.Eur");
        println!("1.Usd");
        println!("2.Eur");
        println!("3.Btc\n");

        println!("please enter the currency you have.");
        io::stdin().read_line(&mut currency).expect("not valid");

        let currency = match currency.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut amount = "".to_string();
        println!("please enter an amount.");
        io::stdin()
            .read_line(&mut amount)
            .expect("input must be a string");

        let amount = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a valid input");
                continue;
            }
        };

        let currency = to_enum(currency);
        let currency_value1 = CurrencyValue { currency, amount };

        println!("please enter the currency you want to convert to.");
        io::stdin().read_line(&mut to_currency).expect("not valid");

        let to_currency = match to_currency.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let to_currency = to_enum(to_currency);
        println!("{}", calc_value(currency_value1, to_currency));
        break;
    }
}

fn to_enum(s: i32) -> Currency {
    match s {
        1 => Currency::Usd,
        2 => Currency::Eur,
        3 => Currency::Btc,
        _ => Currency::Eur,
    }
}

fn calc_value(f: CurrencyValue, t: Currency) -> f64 {
    let eur_usd = 0.84;
    let eur_btc = 32000.0;
    let usd_btc = eur_btc / eur_usd;

    if matches!(f.currency, Currency::Eur) && matches!(t, Currency::Usd) {
        return f.amount / eur_usd;
    } else if matches!(f.currency, Currency::Eur) && matches!(t, Currency::Btc) {
        return f.amount / eur_btc;
    } else if matches!(f.currency, Currency::Usd) && matches!(t, Currency::Eur) {
        return f.amount * eur_usd;
    } else if matches!(f.currency, Currency::Usd) && matches!(t, Currency::Btc) {
        return f.amount * usd_btc;
    } else if matches!(f.currency, Currency::Btc) && matches!(t, Currency::Eur) {
        return f.amount * eur_btc;
    } else if matches!(f.currency, Currency::Btc) && matches!(t, Currency::Usd) {
        return f.amount * usd_btc;
    };

    f.amount
}
