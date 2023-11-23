use crate::browser::BrowserSettings;

use super::ChromeVersion;

mod v104;
mod v105;
mod v106;
mod v106_119;
mod v108;
mod v110;
mod v117;
mod v118;
mod v119;

pub(super) fn get_config_from_ver(ver: ChromeVersion) -> BrowserSettings {
    match ver {
        ChromeVersion::V104 => v104::get_settings(),
        ChromeVersion::V105 => v105::get_settings(),
        ChromeVersion::V106 => v106_119::get_settings(v106::create_headers()),
        ChromeVersion::V108 => v106_119::get_settings(v108::create_headers()),
        ChromeVersion::V110 => v106_119::get_settings(v110::create_headers()),
        ChromeVersion::V117 => v106_119::get_settings(v117::create_headers()),
        ChromeVersion::V118 => v106_119::get_settings(v118::create_headers()),
        ChromeVersion::V119 => v106_119::get_settings(v119::create_headers()),
        ChromeVersion::Latest => v106_119::get_settings(v119::create_headers()),
    }
}
