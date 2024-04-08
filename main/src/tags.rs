use rand::{seq::SliceRandom, thread_rng, Rng};
use serde;
use std::{collections::HashMap, fmt::{Display, Formatter}, vec::Drain};

const MAX_STRENGTH: i8 = -30;
const MIN_STRENGTH: i8 = -80;
const MAX_ANTENNA: u8 = 8;
const MIN_ANTENNA: u8 = 1;
const MOCK_RFID_TAGS: [&str; 9] = [
    "abc123", "abc456", "abc789", "def123", "def456", "def789", "ghi123", "ghi456", "ghi789",
];

#[derive(Debug, Clone, serde::Serialize)]
pub struct Tag {
    pub id: String,
    pub strength: i8,
    pub antenna: u8,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    line: String,
    value: String,
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorKind {
    Incomplete,
    IncorrectAntenna,
    IncorrectStrength,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.kind {
            ParseErrorKind::Incomplete => write!(f, "Line did not have expected parts, we require exactly 3, but found {}. (Full line: {})", self.value, self.line),
            ParseErrorKind::IncorrectAntenna => write!(f, "Line did not have a correct antenna, value should be between {} and {}, but was {}. (Full line: {})", MIN_ANTENNA, MAX_ANTENNA, self.value, self.line),
            ParseErrorKind::IncorrectStrength => write!(f, "Line did not have a correct strength, value should be between {} and {}, but was {}. (Full line: {})", MIN_STRENGTH, MAX_STRENGTH, self.value, self.line),
        }
    }
}

impl Tag {
    pub fn from_reader(line: String) -> Result<Tag, ParseError> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return Err(ParseError {
                kind: ParseErrorKind::Incomplete,
                line: line.to_owned(),
                value: parts.len().to_string(),
            });
        }
        let id = parts[0].to_string();
        let antenna = match parts[1].parse() {
            Ok(value) => {
                if value >= MIN_ANTENNA && value <= MAX_ANTENNA {
                    value
                } else {
                    return Err(ParseError {
                        kind: ParseErrorKind::IncorrectAntenna,
                        line: line.to_owned(),
                        value: parts[1].to_string(),
                    });
                }
            }
            Err(_) => {
                return Err(ParseError {
                    kind: ParseErrorKind::IncorrectAntenna,
                    line: line.to_owned(),
                    value: parts[1].to_string(),
                });
            }
        };
        let strength = match parts[2].parse() {
            Ok(value) => {
                if value >= MIN_STRENGTH && value <= MAX_STRENGTH {
                    value
                } else {
                    return Err(ParseError {
                        kind: ParseErrorKind::IncorrectStrength,
                        line: line.to_owned(),
                        value: parts[2].to_string(),
                    });
                }
            }
            Err(_) => MIN_STRENGTH,
        };

        Ok(Tag {
            id,
            antenna,
            strength,
        })
    }
}

pub fn create_mock_tag() -> String {
    let mut rng = thread_rng();
    let tag_id = MOCK_RFID_TAGS.choose(&mut rng).unwrap();
    let antenna = rng.gen_range(MIN_ANTENNA..MAX_ANTENNA);
    let strength = rng.gen_range(MIN_STRENGTH..MAX_STRENGTH);
    format!("{}|{}|{}", tag_id, antenna, strength)
}

#[cfg(test)]
impl Tag {
    // To help tests, we add a `Tag::random()` method
    pub fn random() -> Tag {
        Self::from_reader(create_mock_tag()).unwrap()
    }
}

#[derive(serde::Serialize, Clone)]
pub struct TagsMap(HashMap<String, Tag>);

impl TagsMap {
    pub fn from(tags: Drain<'_, Tag>) -> Self {
        tags.fold(TagsMap(HashMap::new()), |mut acc, new_tag| {
            acc.0.entry(new_tag.id.clone())
                .and_modify(|old_tag: &mut Tag| {
                    // If there is a current tag, we update this if the new one has a stronger signal
                    if old_tag.strength < new_tag.strength.clone() {
                        *old_tag = new_tag.clone();
                    }
                })
                // If there isn't a new tag, we insert it
                .or_insert(new_tag.clone());
    
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_tag_from_line() {
        let result = Tag::from_reader("abc123|1|-31".into());

        assert!(result.is_ok());
        let tag = result.unwrap();

        assert_eq!("abc123", tag.id);
        assert_eq!(-31, tag.strength);
        assert_eq!(1, tag.antenna);
    }

    #[test]
    fn should_err_if_line_is_incomplete() {
        let result = Tag::from_reader("abc123|1".into());

        assert!(result.is_err_and(|x| x.kind == ParseErrorKind::Incomplete));
    }

    #[test]
    fn should_err_if_antenna_is_unexpected_value() {
        let result = Tag::from_reader("abc123|abc|-31".into());

        assert!(result.is_err_and(|x| x.kind == ParseErrorKind::IncorrectAntenna));

        let result = Tag::from_reader("abc123|0|-31".into());

        assert!(result.is_err_and(|x| x.kind == ParseErrorKind::IncorrectAntenna));

        let result = Tag::from_reader("abc123|9|-31".into());

        assert!(result.is_err_and(|x| x.kind == ParseErrorKind::IncorrectAntenna));
    }

    #[test]
    fn should_set_min_strength_if_unexpected_value() {
        let result = Tag::from_reader("abc123|1|anc".into());

        assert!(result.is_ok());

        let tag = result.unwrap();

        assert!(tag.strength == MIN_STRENGTH);
    }

    #[test]
    fn should_err_if_strength_is_out_of_bounds() {
        let result = Tag::from_reader("abc123|1|-29".into());

        assert!(result.is_err_and(|x| x.kind == ParseErrorKind::IncorrectStrength));

        let result = Tag::from_reader("abc123|1|-81".into());

        assert!(result.is_err_and(|x| x.kind == ParseErrorKind::IncorrectStrength));
    }

    #[test]
    fn should_parse_random_tag() {
        let result = Tag::from_reader(create_mock_tag());

        assert!(result.is_ok())
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
    fn should_keep_strongest_of_two_tokens() {
        let tag1 = Tag {
            id: String::from("abc123"),
            antenna: 2,
            strength: -35,
        };
        let tag2 = Tag {
            id: String::from("abc123"),
            antenna: 1,
            strength: -30,
        };
        let tag3 = Tag {
            id: String::from("abc123"),
            antenna: 1,
            strength: -65,
        };
        let mut tags = vec![tag1, tag2, tag3];

        let map = TagsMap::from(tags.drain(..));

        assert_eq!(1, map.0.keys().len());
        assert_eq!(map.0.contains_key("abc123"), true);
        assert_eq!(map.0["abc123"].antenna, 1);
        assert_eq!(map.0["abc123"].strength, -30);
    }
}
