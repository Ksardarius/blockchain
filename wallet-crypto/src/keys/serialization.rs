pub mod signing_key_serde {
    use k256::ecdsa::SigningKey;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(key: &SigningKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(key.to_bytes()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SigningKey, D::Error>
    where
        D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
            SigningKey::from_slice(&bytes).map_err(serde::de::Error::custom)
        }
}

pub mod verifying_key_serde {
    use k256::ecdsa::VerifyingKey;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(key: &VerifyingKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded_point = key.to_encoded_point(true);
        serializer.serialize_str(&hex::encode(encoded_point.as_bytes()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<VerifyingKey, D::Error>
    where
        D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
            VerifyingKey::from_sec1_bytes(&bytes).map_err(serde::de::Error::custom)
        }
}

#[allow(dead_code)]
pub mod signature_key_serde {
    use k256::ecdsa::Signature;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(key: &Signature, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(key.to_bytes()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Signature, D::Error>
    where
        D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
            Signature::from_slice(&bytes).map_err(serde::de::Error::custom)
        }
}