use crate::browser::BrowserSettings;

use super::ChromeVersion;

mod v104;
mod v105;
mod v106;
mod v108;
mod v110;
mod v106_110;
mod v117;

pub(super) fn get_config_from_ver(ver: ChromeVersion) -> BrowserSettings {
    match ver {
        ChromeVersion::V104 => v104::get_settings(),
        ChromeVersion::V105 => v105::get_settings(),
        ChromeVersion::V106 => v106_110::get_settings(v106::create_headers()),
        ChromeVersion::V108 => v106_110::get_settings(v108::create_headers()),
        ChromeVersion::V110 => v106_110::get_settings(v110::create_headers()),
        ChromeVersion::V117 => v106_110::get_settings(v117::create_headers()),
    }
}
