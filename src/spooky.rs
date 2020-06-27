extern crate gmo_coin;
extern crate shrust;

use shrust::Shell;
use std::io::prelude::*;
use gmo_coin::*;
use gmo_coin::private::api::{ExecutionType, SettlePosition};

macro_rules! print_result {
    ($x:expr, $io:expr) => {
        match $x {
            Ok(resp) => writeln!($io, "{:#?}", resp)?,
            Err(e) => writeln!($io, "{:?}", e)?,
        }
    };
}

macro_rules! unwrap_or_none {
    ($x:expr) => {
        match $x {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }
}

pub fn init() -> Shell<()> {
    let mut shell = Shell::new(());
    shell.new_command_noargs("status", "取引所の稼働状態", |io, _| {
        print_result!(public::api::status(), io); 
        Ok(())
    });
    
    shell.new_command("ticker", "銘柄の最新レート(arg: Symbol)", 1, |io, _, s|{
        match Symbol::from_str(s[0]) {
            Ok(symbol) => {
                print_result!(public::api::ticker(Some(symbol)), io);
                Ok(())
            },
            Err(_) => {
                print_result!(public::api::ticker(None), io);
                Ok(())
            }
        }
    });
    
    shell.new_command("orderbooks", "板情報(arg: Symbol)", 1, |io, _, s|{
        match Symbol::from_str(s[0]) {
            Ok(symbol) => {
                print_result!(public::api::orderbooks(symbol), io);
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
                let page = unwrap_or_none!(s[1].to_string().parse::<usize>());
                let count = unwrap_or_none!(s[2].to_string().parse::<usize>());
                print_result!(public::api::trades(symbol, page, count), io);
    
                Ok(())
            },
            Err(e) => {
                write!(io, "{}", e)?;
                Ok(())
            }
        }
    });

    shell.new_command_noargs("margin", "余力情報", |io, _|{
        print_result!(private::api::margin(),io);
        Ok(())
    });

    shell.new_command_noargs("assets", "資産残高", |io, _|{
        print_result!(private::api::assets(), io);
        Ok(())
    });

    shell.new_command("orders", "注文情報の取得(arg: order_id)", 1, |io, _, s|{
        match s[0].to_string().parse::<usize>()  {
            Ok(id) => print_result!(private::api::orders(id), io),
            Err(e) => writeln!(io, "{:?}", e)?,
        }

        Ok(())
    });

    shell.new_command("active_orders", "有効注文の取得", 3, |io, _, s|{
        match Symbol::from_str(s[0]) {
            Ok(symbol) => {
                let page = unwrap_or_none!(s[1].parse::<usize>());
                let count = unwrap_or_none!(s[2].parse::<usize>());

                print_result!(private::api::active_orders(symbol, page, count), io);
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
                print_result!(private::api::executions(param), io);
            },
            "execution"|"execution_id" => {
                let param = private::api::ExecutionsParam::ExecutionId(nth);
                print_result!(private::api::executions(param), io);
            },
            s => writeln!(io, "Can't parse {:?}: you need specify ID type order or execution", s)?,
        }

        Ok(())
    });

    shell.new_command("latest_executions", "最新約定一覧取得(arg: Symbol, page, count)", 3, |io, _, s|{
        match Symbol::from_str(&s[0].to_string())  {
            Ok(symbol) => {
                let page = unwrap_or_none!(s[1].parse::<usize>());
                let count = unwrap_or_none!(s[2].parse::<usize>());
                
                print_result!(private::api::latest_executions(symbol, page, count), io);
            },
            Err(e) => writeln!(io, "{}", e)?,
        }

        Ok(())
    });

    shell.new_command("open_positions", "有効建玉の取得(arg: Symbol, page, count)", 3, |io, _, s|{
        match Symbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                let page = unwrap_or_none!(s[1].parse::<usize>());
                let count = unwrap_or_none!(s[2].parse::<usize>());

                print_result!(private::api::open_positions(symbol, page, count), io);
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("position_summary", "建玉サマリの取得(arg: Symbol)", 1, |io, _, s|{
        match Symbol::from_str(&s[0].to_string()) {
            Ok(symbol) => print_result!(private::api::position_summary(symbol), io),
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("market_order", "新規成行注文(arg: Symbol, Side, size)", 3, |io, _, s|{
        match Symbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                match Side::from_str(&s[1].to_string()) {
                    Ok(side) => writeln!(io, "{:#?}",
                        private::api::order(
                            symbol, 
                            side, 
                            ExecutionType::MARKET,
                            None,
                            s[2].to_string()))?,
                    Err(e) => writeln!(io, "{}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("limit_order", "新規指値注文(arg: Symbol, Side, price, size)", 4, |io, _, s|{
            match Symbol::from_str(&s[0].to_string()) {
                Ok(symbol) => {
                    match Side::from_str(&s[1].to_string()) {
                        Ok(side) => {writeln!(io, "{:#?}",
                            private::api::order(
                                symbol, 
                                side,
                                ExecutionType::LIMIT,
                                Some(s[2].to_string()),
                                s[3].to_string()))?
                            },
                        Err(e) => writeln!(io, "{}", e)?,
                    }
                },
                Err(e) => writeln!(io, "{}", e)?,
            }
            Ok(())
    });

    shell.new_command("stop_order", "新規逆指値注文(arg: Symbol, Side, price, size)", 4, |io, _, s|{
            match Symbol::from_str(&s[0].to_string()) {
                Ok(symbol) => {
                    match Side::from_str(&s[1].to_string()) {
                        Ok(side) => {writeln!(io, "{:#?}",
                            private::api::order(
                                symbol, 
                                side,
                                ExecutionType::STOP,
                                Some(s[2].to_string()),
                                s[3].to_string()))?
                            },
                        Err(e) => writeln!(io, "{}", e)?,
                    }
                },
                Err(e) => writeln!(io, "{}", e)?,
            }
            Ok(())
    });

    shell.new_command("change_order", "注文変更(arg: order_id, price)", 2, |io, _, s|{
        match s[0].parse::<usize>() {
            Ok(id) => {
                writeln!(io, "{:#?}", 
                    private::api::change_order(
                        id,
                        s[1].to_string(),
                        None))?;
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });


    shell.new_command("change_leverage_order", "注文変更(レバレッジ)(arg: order_id, price, losscut_price)", 3, |io, _, s|{
        match s[0].parse::<usize>() {
            Ok(id) => {
                writeln!(io, "{:#?}", 
                    private::api::change_order(
                        id,
                        s[1].to_string(),
                        Some(s[2].to_string())))?;
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("cancel_order", "注文キャンセル(arg: order_id)", 1, |io, _, s|{
        match s[0].parse::<usize>() {
            Ok(id) => writeln!(io, "{:?}", private::api::cancel_order(id))?,
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("market_close_order", "決済注文(成行)(arg: LeverageSymbol, Side, SettlePosition.position_id, SettlePosition.size)", 4, |io, _, s|{
        match LeverageSymbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                match Side::from_str(&s[1].to_string()) {
                    Ok(side) => {
                        match s[2].parse::<usize>() {
                            Ok(id) => {
                                writeln!(io, "{:#?}",
                                    private::api::close_order(
                                        symbol, 
                                        side, 
                                        ExecutionType::MARKET, 
                                        None,
                                        SettlePosition {
                                            position_id: id,
                                            size: s[3].to_string(),
                                        })
                                )?;
                            },
                            Err(e) => writeln!(io, "{}", e)?,
                        }
                    },
                    Err(e) => writeln!(io, "{}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("limit_close_order", "決済注文(指値)(arg: LeverageSymbol, Side, price, SettlePosition.position_id, SettlePosition.size)", 5, |io, _, s|{
        match LeverageSymbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                match Side::from_str(&s[1].to_string()) {
                    Ok(side) => {
                        match s[3].parse::<usize>() {
                            Ok(id) => {
                                writeln!(io, "{:#?}",
                                    private::api::close_order(
                                        symbol, 
                                        side, 
                                        ExecutionType::LIMIT, 
                                        Some(s[2].to_string()),
                                        SettlePosition {
                                            position_id: id,
                                            size: s[4].to_string(),
                                        })
                                )?;
                            },
                            Err(e) => writeln!(io, "{}", e)?,
                        }
                    },
                    Err(e) => writeln!(io, "{}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("stop_close_order", "決済注文(逆指値)(arg: LeverageSymbol, Side, price, SettlePosition.position_id, SettlePosition.size)", 5, |io, _, s|{
        match LeverageSymbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                match Side::from_str(&s[1].to_string()) {
                    Ok(side) => {
                        match s[3].parse::<usize>() {
                            Ok(id) => {
                                writeln!(io, "{:#?}",
                                    private::api::close_order(
                                        symbol, 
                                        side, 
                                        ExecutionType::STOP, 
                                        Some(s[2].to_string()),
                                        SettlePosition {
                                            position_id: id,
                                            size: s[4].to_string(),
                                        })
                                )?;
                            },
                            Err(e) => writeln!(io, "{}", e)?,
                        }
                    },
                    Err(e) => writeln!(io, "{}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("market_close_bulk_order", "一括決済注文(成行)(arg: LeverageSymbol, Side, size)", 3, |io, _, s|{
        match LeverageSymbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                match Side::from_str(&s[1].to_string()) {
                    Ok(side) => {
                        writeln!(io, "{:#?}",
                            private::api::close_bulk_order(
                                symbol, 
                                side, 
                                ExecutionType::MARKET, 
                                None, 
                                s[2].to_string())
                            )?; 
                    },
                    Err(e) => writeln!(io, "{}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("limit_bulk_order", "一括決済注文(指値)(arg: LeverageSymbol, Side, price, size)", 4, |io, _, s|{
        match LeverageSymbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                match Side::from_str(&s[1].to_string()) {
                    Ok(side) => {
                        writeln!(io, "{:#?}",
                            private::api::close_bulk_order(
                                symbol, 
                                side, 
                                ExecutionType::LIMIT, 
                                Some(s[2].to_string()), 
                                s[3].to_string())
                            )?; 
                    },
                    Err(e) => writeln!(io, "{}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("stop_bulk_order", "一括決済注文(逆指値)(arg: LeverageSymbol, Side, price, size)", 4, |io, _, s|{
        match LeverageSymbol::from_str(&s[0].to_string()) {
            Ok(symbol) => {
                match Side::from_str(&s[1].to_string()) {
                    Ok(side) => {
                        writeln!(io, "{:#?}",
                            private::api::close_bulk_order(
                                symbol, 
                                side, 
                                ExecutionType::STOP, 
                                Some(s[2].to_string()), 
                                s[3].to_string())
                            )?; 
                    },
                    Err(e) => writeln!(io, "{}", e)?,
                }
            },
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell.new_command("change_losscut_price", "建玉のロスカットレート変更", 2, |io, _, s|{
        match s[0].to_string().parse::<usize>() {
            Ok(position_id) => writeln!(io, "{:#?}", private::api::change_losscut_price(position_id, s[1].to_string()))?,
            Err(e) => writeln!(io, "{}", e)?,
        }
        Ok(())
    });

    shell
}
