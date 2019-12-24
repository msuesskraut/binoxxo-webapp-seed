use elsa::FrozenMap;
use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LANG_RESOURCES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("de-DE", include_str!("lang/de-DE"));
        m.insert("en-US", include_str!("lang/en-US"));
        m
    };
}

pub struct ResourceManager {
    resources: FrozenMap<String, Box<FluentResource>>,
}

impl std::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ResourceManager {{}}")
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: FrozenMap::new(),
        }
    }

    fn get_resource(&self, locale: &str) -> &FluentResource {
        if let Some(res) = self.resources.get(locale) {
            res
        } else {
            let raw_resource = LANG_RESOURCES.get(locale).unwrap();
            let res = match FluentResource::try_new(raw_resource.to_string()) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(locale.to_string(), Box::new(res))
        }
    }

    pub fn get_bundle(&self, locale_str: &str) -> FluentBundle<&FluentResource> {
        let loc: LanguageIdentifier = locale_str.parse().expect("invalid locale string");
        let mut bundle = FluentBundle::new(&[loc]);
        let res = self.get_resource(locale_str);
        bundle.add_resource(res).expect("Failed to add FTL resource");
        bundle
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trans_en() {
        let rmgr = ResourceManager::new();
        let b = rmgr.get_bundle("en-US");

        let mut errors = vec![];
        let msg = b.get_message("difficulty-Easy")
            .expect("Failed to retrieve the message");
        let value = msg.value.expect("Failed to retrieve the value of the message");

        assert_eq!("Easy", b.format_pattern(value, None, &mut errors));
        assert!(errors.is_empty());
    }

    #[test]
    fn trans_de() {
        let rmgr = ResourceManager::new();
        let b = rmgr.get_bundle("de-DE");

        let mut errors = vec![];
        let msg = b.get_message("difficulty-Easy")
            .expect("Failed to retrieve the message");
        let value = msg.value.expect("Failed to retrieve the value of the message");

        assert_eq!("Leicht", b.format_pattern(value, None, &mut errors));
        assert!(errors.is_empty());
    }
}
