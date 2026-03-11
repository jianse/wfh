use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use crate::api::BizError;
use crate::api::BizResult;

#[derive(Debug, Clone)]
pub struct Extracted {
    pub ua: Option<ExtractedUa>,
    pub os: Option<ExtractedOs>,
    pub device: Option<ExtractedDevice>,
}

type OUa<'a> = Option<ua_parser::user_agent::ValueRef<'a>>;
type OOs<'a> = Option<ua_parser::os::ValueRef<'a>>;
type ODevice<'a> = Option<ua_parser::device::ValueRef<'a>>;

impl<'a> From<(OUa<'a>, OOs<'a>, ODevice<'a>)> for Extracted {
    fn from(value: (OUa<'a>, OOs<'a>, ODevice<'a>)) -> Self {
        Extracted {
            ua: value.0.map(Into::into),
            os: value.1.map(Into::into),
            device: value.2.map(Into::into),
        }
    }
}

impl Extracted {
    pub fn browser_family(&self) -> Option<String> {
        self.ua.as_ref().map(|ua| ua.family.to_string())
    }

    pub fn browser_version(&self) -> Option<String> {
        self.ua.as_ref().and_then(|ua| ua.version())
    }

    pub fn os_family(&self) -> Option<String> {
        self.os.as_ref().map(|os| os.os.to_string())
    }

    pub fn os_version(&self) -> Option<String> {
        self.os.as_ref().and_then(|ua| ua.version())
    }

    pub fn device_model(&self) -> Option<String> {
        self.device.as_ref().and_then(|device| device.model.clone())
    }

    #[allow(unused)]
    pub fn device_brand(&self) -> Option<String> {
        self.device.as_ref().and_then(|device| device.brand.clone())
    }

    #[allow(unused)]
    pub fn device(&self) -> Option<String> {
        self.device.as_ref().map(|device| device.device.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedUa {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl From<ua_parser::user_agent::ValueRef<'_>> for ExtractedUa {
    fn from(value: ua_parser::user_agent::ValueRef<'_>) -> Self {
        Self {
            family: value.family.to_string(),
            major: value.major.map(ToString::to_string),
            minor: value.minor.map(ToString::to_string),
            patch: value.patch.map(ToString::to_string),
            patch_minor: value.patch_minor.map(ToString::to_string),
        }
    }
}

impl ExtractedUa {
    pub fn version(&self) -> Option<String> {
        if let Some(major) = &self.major {
            if let Some(minor) = &self.minor {
                if let Some(patch) = &self.patch {
                    if let Some(patch_minor) = &self.patch_minor {
                        Some(format!("{}.{}.{}.{}", major, minor, patch, patch_minor))
                    } else {
                        Some(format!("{}.{}.{}", major, minor, patch))
                    }
                } else {
                    Some(format!("{}.{}", major, minor))
                }
            } else {
                Some(major.to_string())
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedOs {
    pub os: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl From<ua_parser::os::ValueRef<'_>> for ExtractedOs {
    fn from(value: ua_parser::os::ValueRef<'_>) -> Self {
        Self {
            os: value.os.to_string(),
            major: value.major.map(String::from),
            minor: value.minor.map(String::from),
            patch: value.patch.map(String::from),
            patch_minor: value.patch_minor.map(String::from),
        }
    }
}

impl ExtractedOs {
    pub fn version(&self) -> Option<String> {
        if let Some(major) = &self.major {
            if let Some(minor) = &self.minor {
                if let Some(patch) = &self.patch {
                    if let Some(patch_minor) = &self.patch_minor {
                        Some(format!("{}.{}.{}.{}", major, minor, patch, patch_minor))
                    } else {
                        Some(format!("{}.{}.{}", major, minor, patch))
                    }
                } else {
                    Some(format!("{}.{}", major, minor))
                }
            } else {
                Some(major.to_string())
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedDevice {
    pub device: String,
    pub brand: Option<String>,
    pub model: Option<String>,
}

impl From<ua_parser::device::ValueRef<'_>> for ExtractedDevice {
    fn from(value: ua_parser::device::ValueRef<'_>) -> Self {
        Self {
            device: value.device.to_string(),
            brand: value.brand.map(String::from),
            model: value.model.map(String::from),
        }
    }
}

#[derive(Clone)]
pub struct UaExtractor(Arc<RwLock<ua_parser::Extractor<'static>>>);

impl UaExtractor {
    pub fn extract<'a>(&'a self, ua: &'a str) -> Extracted {
        let inner = self.0.read().unwrap();
        inner.extract(ua.as_ref()).into()
    }

    pub fn new_from_embed() -> Self {
        let f = include_str!("regexes.yaml");

        let regs: ua_parser::Regexes = serde_yaml::from_str(f).unwrap();
        let ua_extractor = ua_parser::Extractor::try_from(regs).unwrap();
        UaExtractor(Arc::new(RwLock::new(ua_extractor)))
    }

    #[allow(unused)]
    pub fn load_config<P: AsRef<Path>>(&self, conf_path: P) -> BizResult<()> {
        let file = fs::File::open(conf_path)?;
        let rdr = BufReader::new(file);
        let regs: ua_parser::Regexes =
            serde_yaml::from_reader(rdr).map_err(|e| BizError::custom(e.to_string()))?;
        let uae =
            ua_parser::Extractor::try_from(regs).map_err(|e| BizError::custom(e.to_string()))?;
        let mut inner = self.0.write().unwrap();
        *inner = uae;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_ua_extractor() -> eyre::Result<()> {
        let f = include_str!("regexes.yaml");

        let time = std::time::Instant::now();
        let regs: ua_parser::Regexes = serde_yaml::from_str(f)?;
        let ua_extractor = ua_parser::Extractor::try_from(regs)?;

        println!("time: {:?}", time.elapsed());

        let ua_ex = ua_extractor.ua.extract(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:147.0) Gecko/20100101 Firefox/147.0",
        );

        dbg!(ua_ex);
        Ok(())
    }
}
