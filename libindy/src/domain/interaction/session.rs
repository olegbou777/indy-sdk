
use serde::{Serialize,Serializer,Deserialize,Deserializer};

pub type Id = String;

pub struct Timestamp
{
    v: u64
}

pub type Time = Timestamp;

impl<'de> Deserialize<'de> for Timestamp {
         fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
         where
            D: Deserializer<'de>,
         {
             let sec = u64::deserialize(deserializer)?;
             Ok(Timestamp{ v : sec} )
         }

}

impl Serialize for Timestamp {
     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
     where
         S: Serializer,
     {
         serializer.serialize_u64(self.v)
     }
}


