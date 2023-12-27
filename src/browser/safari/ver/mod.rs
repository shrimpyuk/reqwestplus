use crate::browser::BrowserSettings;

use super::SafariVersion;

mod v17_2;

pub(super) fn get_config_from_ver(ver: SafariVersion) -> BrowserSettings {
    match ver {
        SafariVersion::V17_2 => v17_2::get_settings(v17_2::create_headers()),
        SafariVersion::Latest => v17_2::get_settings(v17_2::create_headers()),
    }
}
