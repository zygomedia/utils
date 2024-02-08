// TODO: error type
pub trait SerdeJsonValueExt {
	fn clone_pointer<T: serde::de::DeserializeOwned>(&self, pointer: &str) -> anyhow::Result<T>;
	fn take_pointer<T: serde::de::DeserializeOwned>(&mut self, pointer: &str) -> anyhow::Result<T>;
}

impl SerdeJsonValueExt for serde_json::Value {
	fn clone_pointer<T: serde::de::DeserializeOwned>(&self, pointer: &str) -> anyhow::Result<T> {
		use anyhow::Context;

		self
			.pointer(pointer)
			.context(format!("missing {}", pointer))
			.and_then(|x| Ok(serde_json::from_value(x.clone())?))
	}

	fn take_pointer<T: serde::de::DeserializeOwned>(&mut self, pointer: &str) -> anyhow::Result<T> {
		use anyhow::Context;

		Ok(self
			.pointer_mut(pointer)
			.map(serde_json::Value::take)
			.map(serde_json::from_value)
			.context(format!("can't extract {}", pointer))??)
	}
}

pub fn string_or_number<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
	use serde::{Deserialize, de::Error};

	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Value {
		String(String),
		U64(u64),
	}

	match Value::deserialize(d)? {
		Value::String(x) => x.parse::<u64>().map_err(|e| D::Error::custom(e.to_string())),
		Value::U64(x) => Ok(x),
	}
}

pub mod chrono_duration_minutes {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<chrono::Duration, D::Error> {
		Ok(chrono::Duration::minutes(i64::deserialize(deserializer)?))
	}
	pub fn serialize<S: serde::Serializer>(value: &chrono::Duration, serializer: S) -> Result<S::Ok, S::Error> {
		i64::serialize(&value.num_minutes(), serializer)
	}
}

pub mod chrono_duration_seconds {
	use serde::{Deserialize, Serialize};
	pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<chrono::Duration, D::Error> {
		Ok(chrono::Duration::seconds(i64::deserialize(deserializer)?))
	}
	pub fn serialize<S: serde::Serializer>(value: &chrono::Duration, serializer: S) -> Result<S::Ok, S::Error> {
		i64::serialize(&value.num_seconds(), serializer)
	}
}
