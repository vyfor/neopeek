#![allow(unused_must_use)]
mod ascii;
mod windows;
use chrono::{Local, NaiveDateTime};
use std::io::Write;
use std::{cmp, io};

#[tokio::main]
async fn main() {
    display().await;
}

async fn display() {
    let mut handle = io::stdout().lock();
    let username = &std::env::var("COMPUTERNAME")
        .unwrap_or("null".to_string())
        .to_lowercase();
    let info = windows::fetch().await;
    let os_name = &format!("{} {}", info.0.caption, std::env::consts::ARCH);
    let uptime = Local::now().naive_local().signed_duration_since(
        NaiveDateTime::parse_from_str(
            &info.0.last_boot_up_time[..info.0.last_boot_up_time.len() - 4],
            "%Y%m%d%H%M%S%.6f",
        )
        .unwrap(),
    );
    let shell = &windows::fetch_latest_ps_version();

    let arr: [(&str, &str); 11] = [
        ("", username),
        ("", ""), // separator
        ("OS", os_name),
        ("Kernel", &info.0.version),
        ("Shell", shell),
        (
            "Resolution",
            &format!(
                "{}x{} @ {}Hz",
                &info.2.current_horizontal_resolution,
                &info.2.current_vertical_resolution,
                &info.2.current_refresh_rate
            ),
        ),
        (
            "Uptime",
            &format!(
                "{} hours, {} minutes",
                uptime.num_hours(),
                uptime.num_minutes() % 60
            ),
        ),
        ("CPU", &info.1.name),
        ("GPU", &info.2.name),
        (
            "Memory",
            &format!(
                "{} MiB / {} MiB",
                (info.0.total_visible_memory_size
                    - info.0.free_physical_memory)
                    / 1024,
                info.0.total_visible_memory_size / 1024
            ),
        ),
        (
            "Disk",
            &info
                .3
                .into_iter()
                .map(|x| {
                    format!("({}) {} GiB", x.device_id, x.size / 1073741824)
                })
                .collect::<Vec<String>>()
                .join(" | "),
        ),
    ];
    let term = term_size::dimensions().unwrap().1 > 37;
    let ascii_len = if term {
        ascii::REM_ASCII.len()
    } else {
        ascii::REM_ASCII_MINI.len()
    };
    let start = (ascii_len - arr.len()) / 2
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
        if i >= start && i < start + arr.len() {
            let value = arr.get(i - start).unwrap();
            if i == start {
                writeln!(
                    handle,
                    "{}  {}\x1b[1m\x1b[38;2;137;187;234m@{}",
                    line,
                    " ".repeat(padding),
                    &value.1.trim_end()
                );
            } else if i == start + 1 {
                writeln!(
                    handle,
                    "{}  \x1b[1m\x1b[38;2;137;187;234m{}",
                    line,
                    "—".repeat(max_length)
                );
            } else {
                writeln!(
                    handle,
                    "{}  \x1b[1m\x1b[38;2;137;187;234m{}\x1b[0m{}{}",
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
                writeln!(
                    handle,
                    "{}{}{}",
                    line,
                    " ".repeat(color_padding + 4),
                    ascii::COLORS
                );
            } else {
                writeln!(handle, "{}", line);
            }
        }
    }
}
