#![allow(non_camel_case_types)]

use serde::Deserialize;
use tokio::join;
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
use wmi::{COMLibrary, WMIConnection};

const HKLM: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);

type Info = (
    Win32_OperatingSystem,
    Win32_Processor,
    Win32_VideoController,
    Vec<Win32_LogicalDisk>,
);

pub async fn fetch<'a>() -> Info {
    let con = get_wmi_con();

    let os_info_future = con.async_raw_query::<Win32_OperatingSystem>("SELECT LastBootUpTime, FreePhysicalMemory, Version, Caption, TotalVisibleMemorySize FROM Win32_OperatingSystem");
    let cpu_info_future = con
        .async_raw_query::<Win32_Processor>("SELECT Name FROM Win32_Processor");
    let video_controller_info_future = con.async_raw_query::<Win32_VideoController>("SELECT Name, CurrentHorizontalResolution, CurrentVerticalResolution, CurrentRefreshRate FROM Win32_VideoController");
    let disk_info_future = con.async_raw_query::<Win32_LogicalDisk>(
        "SELECT DeviceID, Size FROM Win32_LogicalDisk",
    );

    let (os_info, cpu_info, video_controller_info, disk_info) = join!(
        os_info_future,
        cpu_info_future,
        video_controller_info_future,
        disk_info_future
    );

    (
        os_info.unwrap().into_iter().next().unwrap(),
        cpu_info.unwrap().into_iter().next().unwrap(),
        video_controller_info.unwrap().into_iter().next().unwrap(),
        disk_info.unwrap(),
    )
}

fn get_wmi_con() -> WMIConnection {
    WMIConnection::new(COMLibrary::without_security().unwrap()).unwrap()
}

pub fn fetch_latest_ps_version() -> String {
    if let Ok(installed_versions) = HKLM
        .open_subkey("SOFTWARE\\Microsoft\\PowerShellCore\\InstalledVersions")
    {
        let keys: Vec<_> = installed_versions
            .enum_keys()
            .filter_map(Result::ok)
            .collect();
        let mut latest_version = String::new();
        for guid in keys.iter() {
            if let Ok(version) = installed_versions.open_subkey(&guid) {
                if let Ok(semantic_version) =
                    version.get_value("SemanticVersion")
                {
                    if semantic_version > latest_version {
                        latest_version = semantic_version;
                    }
                }
            }
        }
        if !latest_version.is_empty() {
            return format!("PowerShell {}", latest_version);
        }
    }

    match std::env::var("PSModulePath") {
        Ok(var) => {
            let lowercased = var.to_lowercase();
            let paths: Vec<&str> = lowercased.split(';').collect();
            for path in paths {
                if path.contains("powershell_") {
                    let version: Vec<&str> = path.split('_').collect();
                    if version.len() > 1 {
                        return format!("PowerShell {}", version[1]);
                    }
                }
            }
        }
        Err(_) => {}
    }

    match fetch_legacy_ps_version() {
        Ok(v) => format!("PowerShell {}", v),
        Err(_) => "Console Host".to_string(),
    }
}

fn fetch_legacy_ps_version() -> Result<String, std::io::Error> {
    let regkey = HKLM
        .open_subkey("SOFTWARE\\Microsoft\\PowerShell\\3\\PowerShellEngine")
        .unwrap_or(HKLM.open_subkey(
            "SOFTWARE\\Microsoft\\PowerShell\\1\\PowerShellEngine",
        )?);
    regkey.get_value("RunTimeVersion")
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_OperatingSystem {
    pub last_boot_up_time: String,
    pub free_physical_memory: u64,
    pub version: String,
    pub caption: String,
    pub total_visible_memory_size: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_Processor {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_VideoController {
    pub name: String,
    pub current_horizontal_resolution: u16,
    pub current_vertical_resolution: u16,
    pub current_refresh_rate: u8,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Win32_LogicalDisk {
    pub device_id: String,
    pub size: u64,
}
