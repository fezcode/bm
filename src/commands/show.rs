use std::collections::HashMap;
use crate::utils::error;
use crate::utils::success::ExecutionResult;
use std::io::Write;
use termsize;

trait CharsTrait {
    const DASH_D: &'static str = "═";
    const DASH_S: &'static str = "─";
    const LINE_D : char = '║';
    const LINE_S : char = '│';
    const LINE_MID_L : char = '╟';
    const LINE_MID_R : char = '╢';
    const UP_L : char = '╔';
    const UP_R : char = '╗';
    const BOT_L : char = '╚';
    const BOT_R : char = '╝';
    const PLUS : char = '┼';
    const DIV_TOP : char = '╤';
    const DIV_BOT : char = '┼';
}

struct Chars;

impl CharsTrait for Chars {}

/// Shows bookmark(s)
///
/// bm show
/// bm show NAME
pub fn show(params: &Option<Vec<String>>, store: HashMap<String, String>) -> ExecutionResult {
    // unsafe {
    //     libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    // }

    let name : String;
    match params {
        // Show all
        None => {
            let mut sorted: Vec<_> = store.iter().collect();
            sorted.sort_by_key(|a| a.0);
            for (key, value) in sorted.iter() {
                println!("{} {}", key, value);
            }

        }
        Some(p) => {
            name = p[0].to_owned();
            if store.contains_key(name.as_str()) {
                let prnt =store[&name].to_owned();

                // Pipes and Rust.
                // https://github.com/rust-lang/rust/issues/46016#issuecomment-415444138
                // writeln!(std::io::stdout(), "{}", store[&name]).unwrap();
                // println!("{}", store[&name]);
                match write!(std::io::stdout(), "{: <width$}\n", prnt , width=prnt.len()) {
                    Ok(_) => {}
                    Err(_) => { eprintln!("{}", prnt); }
                }
                return ExecutionResult{ success: true, write_to_file: false };
            } else if name == "--pretty" || name == "-p" {
                // ----------------------------------------------
                // \ NAME | PRETTY_LONG_PATH_EITHER_FILE_OR_DIR |
                // \ NAME | PRETTY_LONG_PATH_EITHER_FILE_OR_DIR |
                // ----------------------------------------------

                let mut sorted: Vec<_> = store.iter().collect();
                sorted.sort_by_key(|a| a.0);

                // for (key, value) in sorted.iter() {
                //     println!("{} {}", key, value);
                // }

                let mut table: String = "".into();
                let mut table_content : String = "".into();
                // let mut line_size = 0 as usize;

                let mut r = 0;
                let mut c = 0;

                let mut max_name_len = 0 as usize;
                let mut max_path_len = 0 as usize;
                let mut possible_path_len = 0 as usize;

                termsize::get().map(|size| {
                    r = size.rows as usize;
                    c = size.cols as usize;
                    // println!("rows {} cols {}", size.rows, size.cols)
                });

                // Get max sizes of bookmark entries.
                for x in &store {
                    if x.0.len() > max_name_len { max_name_len = x.0.len(); }
                    if x.1.len() > max_path_len { max_path_len = x.1.len(); }
                }

                

                if max_name_len > c/2 - 4 {
                    max_name_len = c/2 - 4; // for ellipses.
                }

                // max_path_len = c - max_name_len;

                for (key, value) in sorted.iter() {
                    let bm_str : String;
                    let bm_path: String;

                    let bm_name;
                    if key.len() > max_name_len {
                        bm_name = format!("{:<w$}...", &key[..max_name_len-3], w=max_name_len-3);
                    } else if key.len() < 4 {   // header row 'Name' string
                        max_name_len = 4;
                        bm_name = format!("{:<w$}",key, w=max_name_len);
                    } 
                    else {
                        bm_name = format!("{:<w$}",key, w=max_name_len);
                    }

                    possible_path_len = c - (10 + max_name_len);

                    // limit path
                    if value.len() > possible_path_len {
                        // println!("LENGTH TOO LONG: current: {}, possible: {}", 7 + key.len() + value.len(), possible_path_len);
                        // bm_path = format!("{name:<.ppl$}", name=value, ppl = possible_path_len);

                        let range = possible_path_len - 13;
                        let temp_path = format!("{}{}{}",&value[0..10], "...", &value[value.len()-range..value.len()]);
                        bm_path = format!("{}", temp_path);

                    } else {
                        bm_path = format!("{name:<ppl$}", name=value, ppl = possible_path_len);
                    }
                    bm_str = format!("\n{line_d} {name} {line_s} {path} {line_d}", name=bm_name, path=bm_path,
                                     line_d=Chars::LINE_D, line_s=Chars::LINE_S);
                    table_content.push_str(bm_str.as_str());
                }

                // line_size = 7 + max_name_len + possible_path_len;
                // table.push_str(format!("{upl}{dash}{upr}", upl=UP_L, upr=UP_R, dash=DASH_D.repeat(line_size-2)).as_str());

                table.push_str(format!("\n{upl}═{name:═^mnl$}═{div}═{path:═^ppl$}═{upr}",
                                       upl=Chars::UP_L,
                                       upr=Chars::UP_R,
                                       div=Chars::DIV_TOP,
                                       name=Chars::DASH_D.repeat(4), mnl=max_name_len,
                                       path=Chars::DASH_D.repeat(5), ppl=possible_path_len ).as_str());

                table.push_str(format!("\n{line_d} {name:^mnl$} {line_s} {path:^ppl$} {line_d}",
                                       name="Name",
                                       path="Paths",
                                       mnl=max_name_len,
                                       ppl=possible_path_len,
                                       line_d=Chars::LINE_D,
                                       line_s=Chars::LINE_S ).as_str());

                table.push_str(format!("\n{left}─{name:─^mnl$}─{line_s}─{path:─^ppl$}─{right}",
                                       left=Chars::LINE_MID_L,
                                       right=Chars::LINE_MID_R,
                                       mnl=max_name_len,
                                       ppl=possible_path_len,
                                       line_s=Chars::PLUS,
                                       name=Chars::DASH_S.repeat(4),
                                       path=Chars::DASH_S.repeat(5)).as_str());

                // table.push_str(format!("{left}{dash}{plus}{dash}{right}", left=LINE_MID_L, right=LINE_MID_R,
                //                        plus=PLUS, dash=DASH_S.repeat((line_size)/2)).as_str());

                table.push_str(table_content.as_str());

                table.push_str(format!("\n{botl}═{name:═^mnl$}═{line_s}═{path:═^ppl$}═{botr}",
                                       botl=Chars::BOT_L,
                                       botr=Chars::BOT_R,
                                       line_s="╧",
                                       name=Chars::DASH_D.repeat(4), mnl=max_name_len,
                                       path=Chars::DASH_D.repeat(5), ppl=possible_path_len ).as_str());


                println!("{}", table);



            }
            else {
                error::print_error_and_exit(format!("Given key `{}` does not exist.", name),
                                            error::ErrorCode::ShowCommandNameNotFound)
            }
        }
    }
    return ExecutionResult{ success: false, write_to_file: false };
}