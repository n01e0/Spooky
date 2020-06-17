extern crate gmo_coin;
extern crate shrust;

use shrust::Shell;
use std::io::prelude::*;
use gmo_coin::*;

pub fn init() -> Shell<()> {
    let mut shell = Shell::new(());
    shell.new_command_noargs("status", "取引所の稼働状態", |io, _| {
        match public::api::status() {
            Ok(resp) => writeln!(io, "{:#?}", resp)?,
            Err(e) => writeln!(io, "{:?}", e)?,
        }
    
        Ok(())
    });
    
    shell.new_command("ticker", "銘柄の最新レート(arg: Symbol)", 1, |io, _, s|{
        match Symbol::from_str(s[0]) {
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
        match Symbol::from_str(s[0]) {
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
        match Symbol::from_str(s[0]) {
            Ok(symbol) => {
                let page: Option<usize> = match s[1].to_string().parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };
    
                let count: Option<usize> = match s[2].to_string().parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };
                match public::api::trades(symbol, page, count) {
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

    shell.new_command_noargs("margin", "余力情報", |io, _|{
        match private::api::margin() {
            Ok(resp) => writeln!(io, "{:#?}", resp)?,
            Err(e) => writeln!(io, "{:?}", e)?,
        }
    
        Ok(())
    });

    shell.new_command_noargs("assets", "資産残高", |io, _|{
        match private::api::assets() {
            Ok(resp) => writeln!(io, "{:#?}", resp)?,
            Err(e) => writeln!(io, "{:?}", e)?,
        }

        Ok(())
    });

    shell.new_command("orders", "注文情報の取得(arg: order_id)", 1, |io, _, s|{
        match s[0].to_string().parse::<usize>()  {
            Ok(id) => {
                let orders = private::api::orders(id);
                match orders {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
            },
            Err(e) => writeln!(io, "{:?}", e)?,
        }

        Ok(())
    });

    shell.new_command("active_orders", "有効注文の取得", 3, |io, _, s|{
        match Symbol::from_str(s[0]) {
            Ok(symbol) => {
                let page: Option<usize> = match s[1].parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };

                let count: Option<usize> = match s[2].parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };

                let ao = private::api::active_orders(symbol, page, count);
                match ao {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
            },
            Err(e) => writeln!(io, "{:?}", e)?,
        }
        Ok(())
    });

    shell.new_command("executions", "約定情報の取得(arg: order|execution, id)", 2, |io, _, s|{
        let nth = s[1].parse::<usize>()?;
        match s[0] {
            "order"|"order_id" => {
                let param = private::api::ExecutionsParam::OrderId(nth);
                match private::api::executions(param) {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
            },
            "execution"|"execution_id" => {
                let param = private::api::ExecutionsParam::ExecutionId(nth);
                match private::api::executions(param) {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
            },
            s => writeln!(io, "Can't parse {:?}: you need specify ID type order or execution", s)?,
        }

        Ok(())
    });

    shell.new_command("latest_executions", "最新約定一覧取得(arg: Symbol, page, count)", 3, |io, _, s|{
        match Symbol::from_str(&s[0].to_string())  {
            Ok(symbol) => {
                let page: Option<usize> = match s[1].parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };

                let count: Option<usize> = match s[2].parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                };
                
                match private::api::latest_executions(symbol, page, count) {
                    Ok(resp) => writeln!(io, "{:#?}", resp)?,
                    Err(e) => writeln!(io, "{:?}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }

        Ok(())
    });

    shell
}
