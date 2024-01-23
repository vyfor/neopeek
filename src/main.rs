mod ascii;
mod enums;
mod windows;
use enums::StatType;
use std::cmp;
use std::process::Command;

fn main() {
    display();
}

fn display() {
    let username = std::env::var("COMPUTERNAME")
        .unwrap_or("null".to_string())
        .to_lowercase();
    let (info, term) = windows::fetch();
    let os_name = &format!(
        "{} {}",
        info.get(&StatType::Os).unwrap(),
        std::env::consts::ARCH
    );
    let shell = match Command::new("pwsh")
        .args(&["-C", "$PSVersionTable.PSVersion.ToString()"])
        .output()
    {
        Ok(output) => {
            format!("PowerShell {}", String::from_utf8_lossy(&output.stdout))
        }
        Err(_) => "Not detected".to_string(),
    };

    let arr: [(&str, &str); 11] = [
        ("", &username),
        ("", ""), // separator
        ("OS", os_name),
        ("Kernel", info.get(&StatType::Kernel).unwrap()),
        ("Shell", &shell),
        ("Resolution", info.get(&StatType::Resolution).unwrap()),
        ("Uptime", info.get(&StatType::Uptime).unwrap()),
        ("CPU", info.get(&StatType::Cpu).unwrap()),
        ("GPU", info.get(&StatType::Gpu).unwrap()),
        ("Memory", info.get(&StatType::Ram).unwrap()),
        ("Disk", info.get(&StatType::Disks).unwrap()),
    ];
    let ascii_len = if term {
        ascii::REM_ASCII.len()
    } else {
        ascii::REM_ASCII_MINI.len()
    };
    let arr_len = arr.len();
    let start = (ascii_len - arr_len) / 2
        - if term { cmp::max(arr.len(), 9) - 9 } else { 0 };
    let max_key_len = arr.iter().map(|v| v.0.len()).max().unwrap_or(0);
    let max_length = arr
        .iter()
        .map(|v| max_key_len + 1 + v.1.len())
        .max()
        .unwrap_or(0);
    let padding = (max_length - username.len()) / 2;
    let color_padding = (max_length - 24) / 2;

    for i in 0..ascii_len {
        let line = if term {
            ascii::REM_ASCII[i]
        } else {
            ascii::REM_ASCII_MINI[i]
        };
        if i >= start && i < start + arr_len {
            let value = arr.get(i - start).unwrap();
            if i == start {
                println!(
                    "{}  {}\x1b[1m\x1b[38;2;137;187;234m@{}",
                    line,
                    " ".repeat(padding),
                    &value.1.trim_end()
                );
            } else if i == start + 1 {
                println!(
                    "{}    \x1b[1m\x1b[38;2;137;187;234m{}",
                    line,
                    "—".repeat(max_length)
                );
            } else {
                println!(
                    "{}    \x1b[1m\x1b[38;2;137;187;234m{}\x1b[0m{}{}",
                    line,
                    &value.0,
                    " ".repeat(max_key_len - &value.0.len()),
                    format!(
                        " \x1b[38;2;137;187;234m: \x1b[38;2;224;155;187m{}",
                        &value.1.trim_end()
                    )
                );
            }
        } else {
            if i == start + arr.len() + 1 {
                println!(
                    "{}{}{}",
                    line,
                    " ".repeat(color_padding + 4),
                    ascii::COLORS
                );
            } else {
                println!("{}", line);
            }
        }
    }
}
