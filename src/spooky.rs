extern crate gmo_coin;
extern crate shrust;

use shrust::Shell;
use std::io::prelude::*;
use gmo_coin::*;

pub fn init() -> Shell<()> {
    let mut shell = Shell::new(());
    shell.new_command_noargs("status", "取引所の稼働状態", |io, _| {
        let status = public::api::status();
        match status {
            Ok(resp) => writeln!(io, "{:#?}", resp)?,
            Err(e) => writeln!(io, "{:?}", e)?,
        }
    
        Ok(())
    });
    
    shell.new_command("ticker", "銘柄の最新レート(arg: Symbol)", 1, |io, _, s|{
        match Symbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                let tick = public::api::ticker(Some(symbol));
                match tick {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
    
                Ok(())
            },
            Err(_) => {
                let tick = public::api::ticker(None);
                match tick {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
    
                Ok(())
            }
        }
    });
    
    shell.new_command("orderbooks", "板情報(arg: Symbol)", 1, |io, _, s|{
        match Symbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                let book = public::api::orderbooks(symbol);
                match book {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
    
                Ok(())
            },
            Err(e) => {
                write!(io, "{}", e)?;
                Ok(())
            }
        }
    });
    
    shell.new_command("trades", "取引履歴のリスト(arg: Symbol, page, count)", 3, |io, _, s|{
        match s[0].to_string().parse::<Symbol>() {
            Ok(symbol) => {
                let page: Option<usize> = match s[1].to_string().parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };
    
                let count: Option<usize> = match s[2].to_string().parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };
                let trades = public::api::trades(symbol, page, count);
                match trades {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
    
                Ok(())
            },
            Err(e) => {
                write!(io, "{}", e)?;
                Ok(())
            }
        }
    });

    shell
}
