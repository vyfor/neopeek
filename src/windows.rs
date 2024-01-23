use std::collections::HashMap;
use std::process::Command;

use crate::enums::StatType;

const COMMANDS: &str = concat!(
    "mode con | findstr Lines & ",
    "wmic os get Caption,Version /VALUE & ",
    "wmic cpu get Name /VALUE & ",
    "wmic path win32_VideoController get Name,CurrentHorizontalResolution,CurrentVerticalResolution,CurrentRefreshRate /VALUE & ",
    "wmic OS get FreePhysicalMemory,TotalVisibleMemorySize,LastBootUpTime /VALUE & ",
    "wmic logicaldisk get DeviceID,Size /VALUE"
);

pub fn fetch() -> (HashMap<StatType, String>, bool) {
    let output = match Command::new("cmd").args(&["/c", COMMANDS]).output() {
        Ok(o) => o.stdout,
        Err(_) => panic!(
            "Could not obtain necessary system information through wmic!"
        ),
    };

    parse(String::from_utf8(output).unwrap().as_str())
}

fn parse(input: &str) -> (HashMap<StatType, String>, bool) {
    let mut map: HashMap<StatType, String> = HashMap::new();
    let mut screen_width: u16 = 0;
    let mut screen_height: u16 = 0;
    let mut refresh_rate: u8 = 0;
    let mut total_ram: u64 = 0;
    let mut free_ram: u64 = 0;
    let mut disks: Vec<String> = Vec::new();
    let mut term: bool = true;

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            term = line
                .split("Lines:")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<u8>()
                .unwrap()
                > 37;
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "Caption" => {
                    map.insert(StatType::Os, value.trim().to_string());
                }
                "Version" => {
                    map.insert(StatType::Kernel, value.trim().to_string());
                }
                "Name" => {
                    if map.contains_key(&StatType::Cpu) {
                        map.insert(StatType::Gpu, value.trim().to_string());
                    } else {
                        map.insert(StatType::Cpu, value.trim().to_string());
                    }
                }
                "CurrentHorizontalResolution" => {
                    screen_width = value.trim().parse().unwrap();
                }
                "CurrentVerticalResolution" => {
                    screen_height = value.trim().parse().unwrap();
                }
                "CurrentRefreshRate" => {
                    refresh_rate = value.trim().parse().unwrap_or(60);
                }
                "FreePhysicalMemory" => {
                    free_ram = value.trim().parse().unwrap();
                }
                "TotalVisibleMemorySize" => {
                    total_ram = value.trim().parse().unwrap();
                }
                "LastBootUpTime" => {
                    let timestamp = value.trim();
                    map.insert(
                        StatType::Uptime,
                        format!(
                            "{} hours, {} minutes",
                            &timestamp.trim()[9..10],
                            &timestamp.trim()[11..12]
                        ),
                    );
                }
                "DeviceID" | "Size" => {
                    if value.contains(':') {
                        disks.push(format!("{}{}{}", "(", value.trim(), ")"));
                    } else {
                        disks.push(format!(
                            "{} GiB",
                            match value.trim().parse::<u64>() {
                                Ok(v) => (v / 1073741824).to_string(),
                                Err(_) => value.trim().to_string(),
                            }
                        ));
                    }
                }
                _ => {}
            }
        }
    }
    map.insert(
        StatType::Resolution,
        format!("{}x{} @ {}Hz", screen_width, screen_height, refresh_rate),
    );
    map.insert(
        StatType::Ram,
        format!(
            "{} MiB / {} MiB",
            (total_ram - free_ram) / 1024,
            total_ram / 1024,
        ),
    );
    map.insert(
        StatType::Disks,
        disks
            .chunks(2)
            .map(|chunk| chunk.join(" "))
            .collect::<Vec<String>>()
            .join(" | "),
    );

    (map, term)
}
