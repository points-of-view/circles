use rand::{seq::SliceRandom, thread_rng, Rng};
use serde;
use std::{
    collections::{hash_map::Values, HashMap},
    fmt::{Debug, Display, Formatter},
    vec::Drain,
};

const MAX_STRENGTH: i8 = 0;
const MIN_STRENGTH: i8 = -80;
const MAX_ANTENNA: u16 = 3;
const MIN_ANTENNA: u16 = 1;

const MOCK_RFID_TAGS: [&str; 9] = [
    "abc123", "abc456", "abc789", "def123", "def456", "def789", "ghi123", "ghi456", "ghi789",
];

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    pub id: String,
    pub strength: i8,
    pub antenna: u16,
}

#[derive(Debug, serde::Serialize)]
pub struct TagError {
    pub kind: TagErrorKind,
    value: String,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub enum TagErrorKind {
    Incomplete,
    IncorrectAntenna,
    IncorrectStrength,
}

impl Display for TagError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.kind {
            TagErrorKind::Incomplete => write!(f, "Line did not have expected parts, we require exactly 3, but found {}.", self.value),
            TagErrorKind::IncorrectAntenna => write!(f, "Line did not have a correct antenna, value should be between {} and {}, but was {}.", MIN_ANTENNA, MAX_ANTENNA, self.value),
            TagErrorKind::IncorrectStrength => write!(f, "Line did not have a correct strength, value should be between {} and {}, but was {}.", MIN_STRENGTH, MAX_STRENGTH, self.value),
        }
    }
}

impl Tag {
    pub fn build(id: String, antenna: u16, strength: i8) -> Result<Tag, TagError> {
        if antenna < MIN_ANTENNA || antenna > MAX_ANTENNA {
            return Err(TagError {
                kind: TagErrorKind::IncorrectAntenna,
                value: antenna.to_string(),
            });
        }

        if strength < MIN_STRENGTH || strength > MAX_STRENGTH {
            return Err(TagError {
                kind: TagErrorKind::IncorrectStrength,
                value: strength.to_string(),
            });
        }

        Ok(Tag {
            id,
            antenna,
            strength,
        })
    }

    pub fn random() -> Tag {
        let mut rng = thread_rng();
        let id = MOCK_RFID_TAGS.choose(&mut rng).unwrap().to_string();
        let antenna = rng.gen_range(MIN_ANTENNA..=MAX_ANTENNA);
        let strength = rng.gen_range(MIN_STRENGTH..MAX_STRENGTH);
        Tag {
            id,
            antenna,
            strength,
        }
    }
}

impl Tag {
    pub fn from_report_data(
        tag_report_data: llrp::parameters::TagReportData,
    ) -> Result<Tag, TagError> {
        let id_bytes = match tag_report_data.epc_parameter.clone() {
            llrp::choices::EPCParameter::EPCData(data) => data.epc.bytes,
            llrp::choices::EPCParameter::EPC_96(data) => data.to_vec(),
        };
        let id = id_bytes
            .iter()
            .map(|byte| format!("{:02X?}", byte))
            .collect();
        let antenna = tag_report_data.antenna_id.unwrap();
        let strength = tag_report_data.peak_rssi.unwrap();

        Self::build(id, antenna, strength)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TagsMap(HashMap<String, Tag>);

impl TagsMap {
    pub fn new() -> Self {
        TagsMap(HashMap::new())
    }

    pub fn add_tag(&mut self, new_tag: Tag) {
        self.0.insert(new_tag.clone().id, new_tag);
    }

    pub fn add_tags(&mut self, new_tags: Vec<Tag>) {
        new_tags.into_iter().for_each(|tag| self.add_tag(tag));
    }
}

impl From<Drain<'_, Tag>> for TagsMap {
    fn from(tags: Drain<'_, Tag>) -> Self {
        let mut new = Self::new();
        tags.for_each(|tag| new.add_tag(tag));
        new
    }
}

impl TagsMap {
    pub fn values(&self) -> Values<'_, String, Tag> {
        self.0.values()
    }

    pub fn random(size: usize) -> Self {
        let tags = vec![Tag::random(); size];
        let mut new = Self::new();
        new.add_tags(tags);
        new
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::messages::construct_tag_report;

    use super::*;

    #[test]
    fn should_create_tag_from_line() {
        let tag_report = construct_tag_report(1, -30);
        let result = Tag::from_report_data(tag_report);

        assert!(result.is_ok());
        let tag = result.unwrap();

        assert_eq!("000000000000000000000000", tag.id);
        assert_eq!(-30, tag.strength);
        assert_eq!(1, tag.antenna);
    }

    #[test]
    fn should_err_if_antenna_is_unexpected_value() {
        let result = Tag::build("abc123".into(), 0, -31);

        assert!(result.is_err_and(|x| x.kind == TagErrorKind::IncorrectAntenna));

        let result = Tag::build("abc123".into(), 4, -31);

        assert!(result.is_err_and(|x| x.kind == TagErrorKind::IncorrectAntenna));
    }

    #[test]
    fn should_err_if_strength_is_out_of_bounds() {
        let result = Tag::build("abc123".into(), 1, 1);

        assert!(result.is_err_and(|x| x.kind == TagErrorKind::IncorrectStrength));

        let result = Tag::build("abc123".into(), 1, -81);

        assert!(result.is_err_and(|x| x.kind == TagErrorKind::IncorrectStrength));
    }

    #[test]
    fn should_create_map_from_vector() {
        let tag = Tag {
            id: String::from("abc123"),
            antenna: 1,
            strength: -30,
        };
        let mut tags = vec![tag];

        let map = TagsMap::from(tags.drain(..));

        assert_eq!(1, map.0.keys().len());
        assert_eq!(map.0.contains_key("abc123"), true);
        assert_eq!(map.0["abc123"].antenna, 1);
    }

    #[test]
    fn should_replace_tag_when_updating() {
        let tag1 = Tag {
            id: String::from("abc123"),
            antenna: 2,
            strength: -35,
        };
        let mut tags = vec![tag1];

        let mut map = TagsMap::from(tags.drain(..));

        map.add_tag(Tag {
            id: String::from("abc123"),
            antenna: 1,
            strength: -65,
        });

        assert_eq!(1, map.0.keys().len());
        assert_eq!(map.0.contains_key("abc123"), true);
        assert_eq!(map.0["abc123"].antenna, 1);
        assert_eq!(map.0["abc123"].strength, -65);
    }
}
