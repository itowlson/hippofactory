use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};
use std::convert::TryFrom;

type AnnotationMap = BTreeMap<String, String>;

// Raw on-disk forms, used only for deserialisation

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct RawHippoFacts {
    pub bindle: BindleSpec,
    pub annotations: Option<AnnotationMap>,
    pub handler: Option<Vec<RawHandler>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct RawHandler {
    name: Option<String>,
    external: Option<String>,
    pub route: String,
    pub files: Option<Vec<String>>,
}

// A 'safe to use' form

pub struct HippoFacts {
    pub bindle: BindleSpec,
    pub annotations: Option<AnnotationMap>,
    pub handler: Vec<Handler>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BindleSpec {
    pub name: String,
    pub version: String, // not semver::Version because this could be a template
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
}

pub struct Handler {
    pub handler_module: HandlerModule,
    pub route: String,
    pub files: Option<Vec<String>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum HandlerModule {
    File(String),
    External(ParcelReference),
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParcelReference {
    pub bindle_id: bindle::Id,
    pub name: String,
}

impl HippoFacts {
    fn parse(raw: RawHippoFacts) -> anyhow::Result<Self> {
        let handlers = match raw.handler {
            None => Err(anyhow::anyhow!("Artifact spec must specify at least one handler")),
            Some(h) => {
                if h.len() == 0 {
                    Err(anyhow::anyhow!("Artifact spec must specify at least one handler"))
                } else {
                    h.into_iter().map(|r| Handler::parse(r)).collect::<anyhow::Result<Vec<Handler>>>()
                }
            },
        }?;
        Ok(Self {
            bindle: raw.bindle,
            annotations: raw.annotations,
            handler: handlers,
        })
    }

    pub fn read_from_file(path: impl AsRef<Path>) -> anyhow::Result<HippoFacts> {
        let toml_text = std::fs::read_to_string(path)?;
        let raw: RawHippoFacts = toml::from_str(&toml_text)?;
        Self::parse(raw)
    }
}

impl Handler {
    fn parse(raw: RawHandler) -> anyhow::Result<Self> {
        let handler_module = raw.handler_module()?;
        Ok(Self {
            handler_module,
            route: raw.route,
            files: raw.files,
        })
    }
}

impl RawHandler {
    pub fn handler_module(&self) -> anyhow::Result<HandlerModule> {
        match (&self.name, &self.external) {
            (Some(name), None) => Ok(HandlerModule::File(name.to_owned())),
            (None, Some(parcel_ref)) => Ok(HandlerModule::External(ParcelReference::parse(parcel_ref)?)),
            (None, None) => Err(anyhow::anyhow!("You must specify one of 'name' or 'external' in handler for {}", self.route)),
            (Some(_), Some(_)) => Err(anyhow::anyhow!("You cannot specify both 'name' and 'external' in handler for {}", self.route)),
        }
    }
}

impl ParcelReference {
    pub fn parse(text: &str) -> anyhow::Result<Self> {
        let bits = text.split(':').collect_vec();
        if bits.len() == 2 {
            Ok(Self {
                bindle_id: bindle::Id::try_from(bits[0])?,
                name: bits[1].to_owned()
            })
        } else {
            Err(anyhow::anyhow!("External reference must be of the form 'bindle_id:parcel_name'"))
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    pub fn read_hippofacts_from_string(text: &str) -> anyhow::Result<HippoFacts> {
        let raw: RawHippoFacts = toml::from_str(text)?;
        HippoFacts::parse(raw)
    }

    #[test]
    fn test_can_read_hippo_facts() {
        let facts = read_hippofacts_from_string(
            r#"
        # HIPPO FACT: the North American house hippo is found across Canada and the Eastern US
        [bindle]
        name = "birds"
        version = "1.2.4"

        [[handler]]
        name = "penguin.wasm"
        route = "/birds/flightless"
        files = ["adelie.png", "rockhopper.png", "*.jpg"]

        [[handler]]
        external = "foo/bar/1.0.0:cassowary.wasm"
        route = "/birds/savage/rending"
        "#,
        )
        .expect("error parsing test TOML");

        assert_eq!("birds", &facts.bindle.name);
        assert_eq!(&None, &facts.annotations);

        let handlers = &facts.handler;

        assert_eq!(2, handlers.len());

        assert_eq!(&HandlerModule::File("penguin.wasm".to_owned()), &handlers[0].handler_module);
        assert_eq!("/birds/flightless", &handlers[0].route);
        let files0 = handlers[0].files.as_ref().expect("Expected files");
        assert_eq!(3, files0.len());

        let expected_ref = ParcelReference {
            bindle_id: bindle::Id::from_str("foo/bar/1.0.0").expect("malformed bindle id"),
            name: "cassowary.wasm".to_owned(),
        };
        assert_eq!(&HandlerModule::External(expected_ref), &handlers[1].handler_module);
        assert_eq!("/birds/savage/rending", &handlers[1].route);
        assert_eq!(None, handlers[1].files);
    }
}
