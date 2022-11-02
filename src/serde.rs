use crate::BigNumber;
use std::fmt;

impl serde::ser::Serialize for BigNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.collect_str(&self)
    }
}

struct BigNumberVisitor;

impl<'de> serde::de::Visitor<'de> for BigNumberVisitor {
    type Value = BigNumber;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a number or formatted decimal string")
    }

    fn visit_str<E>(self, value: &str) -> Result<BigNumber, E>
    where
        E: serde::de::Error,
    {
        BigNumber::of(value).map_err(|err| E::custom(format!("{}", err)))
    }
}

impl<'de> serde::de::Deserialize<'de> for BigNumber {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        d.deserialize_str(BigNumberVisitor)
    }
}
