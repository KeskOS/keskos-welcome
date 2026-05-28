use serde::Deserialize;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

const RELEASE_PATH: &str = "/etc/keskos-release";
const BRANDING_JSON_PATH: &str = "/usr/lib/keskos/branding.json";

#[derive(Clone, Debug, Default, Deserialize)]
struct JsonBranding {
    name: Option<String>,
    pretty_name: Option<String>,
    layer: Option<String>,
    layer_name: Option<String>,
    brand_line: Option<String>,
    channel: Option<String>,
    build_id: Option<String>,
    accent_color: Option<String>,
    home_url: Option<String>,
    documentation_url: Option<String>,
    download_url: Option<String>,
    support_url: Option<String>,
    bug_report_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Branding {
    pub name: String,
    pub pretty_name: String,
    pub layer: String,
    pub layer_name: String,
    pub brand_line: String,
    pub channel: String,
    pub build_id: String,
    pub accent_color: String,
    pub home_url: String,
    pub documentation_url: String,
    pub download_url: String,
    pub support_url: String,
    pub bug_report_url: String,
}

impl Default for Branding {
    fn default() -> Self {
        Self {
            name: String::from("KeskOS"),
            pretty_name: String::from("KeskOS"),
            layer: String::new(),
            layer_name: String::new(),
            brand_line: String::from("KeskOS"),
            channel: String::from("stable"),
            build_id: String::from("dev"),
            accent_color: String::from("#ce6a35"),
            home_url: String::from("https://keskos.org"),
            documentation_url: String::from("https://docs.keskos.org"),
            download_url: String::from("https://downloads.keskos.org"),
            support_url: String::from("https://docs.keskos.org"),
            bug_report_url: String::from("https://github.com/KeskOS"),
        }
    }
}

impl Branding {
    fn apply_json(&mut self, payload: JsonBranding) {
        if let Some(value) = payload.name.filter(|value| !value.trim().is_empty()) {
            self.name = value.trim().to_string();
        }
        if let Some(value) = payload.pretty_name.filter(|value| !value.trim().is_empty()) {
            self.pretty_name = value.trim().to_string();
        }
        if let Some(value) = payload.layer.filter(|value| !value.trim().is_empty()) {
            self.layer = value.trim().to_string();
        }
        if let Some(value) = payload.layer_name.filter(|value| !value.trim().is_empty()) {
            self.layer_name = value.trim().to_string();
        }
        if let Some(value) = payload.brand_line.filter(|value| !value.trim().is_empty()) {
            self.brand_line = value.trim().to_string();
        }
        if let Some(value) = payload.channel.filter(|value| !value.trim().is_empty()) {
            self.channel = value.trim().to_string();
        }
        if let Some(value) = payload.build_id.filter(|value| !value.trim().is_empty()) {
            self.build_id = value.trim().to_string();
        }
        if let Some(value) = payload.accent_color.filter(|value| !value.trim().is_empty()) {
            self.accent_color = value.trim().to_string();
        }
        if let Some(value) = payload.home_url.filter(|value| !value.trim().is_empty()) {
            self.home_url = value.trim().to_string();
        }
        if let Some(value) = payload.documentation_url.filter(|value| !value.trim().is_empty()) {
            self.documentation_url = value.trim().to_string();
        }
        if let Some(value) = payload.download_url.filter(|value| !value.trim().is_empty()) {
            self.download_url = value.trim().to_string();
        }
        if let Some(value) = payload.support_url.filter(|value| !value.trim().is_empty()) {
            self.support_url = value.trim().to_string();
        }
        if let Some(value) = payload.bug_report_url.filter(|value| !value.trim().is_empty()) {
            self.bug_report_url = value.trim().to_string();
        }
    }

    fn apply_release_map(&mut self, values: &HashMap<String, String>) {
        if let Some(value) = values.get("NAME").filter(|value| !value.is_empty()) {
            self.name = value.clone();
        }
        if let Some(value) = values.get("PRETTY_NAME").filter(|value| !value.is_empty()) {
            self.pretty_name = value.clone();
        }
        if let Some(value) = values.get("LAYER").filter(|value| !value.is_empty()) {
            self.layer = value.clone();
        }
        if let Some(value) = values.get("LAYER_NAME").filter(|value| !value.is_empty()) {
            self.layer_name = value.clone();
        }
        if let Some(value) = values.get("BRAND_LINE").filter(|value| !value.is_empty()) {
            self.brand_line = value.clone();
        }
        if let Some(value) = values.get("CHANNEL").filter(|value| !value.is_empty()) {
            self.channel = value.clone();
        }
        if let Some(value) = values.get("BUILD_ID").filter(|value| !value.is_empty()) {
            self.build_id = value.clone();
        }
        if let Some(value) = values.get("ACCENT_COLOR").filter(|value| !value.is_empty()) {
            self.accent_color = value.clone();
        }
        if let Some(value) = values.get("HOME_URL").filter(|value| !value.is_empty()) {
            self.home_url = value.clone();
        }
        if let Some(value) = values.get("DOCUMENTATION_URL").filter(|value| !value.is_empty()) {
            self.documentation_url = value.clone();
        }
        if let Some(value) = values.get("DOWNLOAD_URL").filter(|value| !value.is_empty()) {
            self.download_url = value.clone();
        }
        if let Some(value) = values.get("SUPPORT_URL").filter(|value| !value.is_empty()) {
            self.support_url = value.clone();
        }
        if let Some(value) = values.get("BUG_REPORT_URL").filter(|value| !value.is_empty()) {
            self.bug_report_url = value.clone();
        }
    }

    fn normalize(&mut self) {
        if self.layer_name.is_empty() && !self.layer.is_empty() {
            self.layer_name = format!("Layer {}", self.layer);
        }
        if self.brand_line.is_empty() {
            self.brand_line = if self.pretty_name.is_empty() {
                self.name.clone()
            } else {
                self.pretty_name.clone()
            };
        }
        if self.pretty_name.is_empty() {
            self.pretty_name = self.brand_line.clone();
        }
    }

    pub fn spaced_name(&self) -> String {
        self.name
            .to_uppercase()
            .chars()
            .map(|ch| ch.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn read_release_map(path: &Path) -> HashMap<String, String> {
    let mut values = HashMap::new();
    let Ok(contents) = fs::read_to_string(path) else {
        return values;
    };

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let cleaned = value.trim().trim_matches('"').to_string();
        values.insert(key.trim().to_string(), cleaned);
    }

    values
}

pub fn load_branding() -> Branding {
    let mut branding = Branding::default();

    if let Ok(contents) = fs::read_to_string(BRANDING_JSON_PATH) {
        if let Ok(payload) = serde_json::from_str::<JsonBranding>(&contents) {
            branding.apply_json(payload);
        }
    }

    branding.apply_release_map(&read_release_map(Path::new(RELEASE_PATH)));
    branding.normalize();
    branding
}
