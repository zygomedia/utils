#[derive(
	Clone, Copy, Debug, Hash,
	PartialOrd, Ord,
	PartialEq, Eq,
	shrinkwraprs::Shrinkwrap,
	smart_default::SmartDefault,
	serde::Serialize, serde::Deserialize,
	derive_more::Display,
	derive_more::Add, derive_more::Sub,
)]
pub struct Duration(
	#[serde(with = "crate::serde_utils::chrono_duration")]
	#[default(chrono::Duration::zero())]
	chrono::Duration
);

impl rkyv::Archive for Duration {
	type Archived = Duration;
	type Resolver = ((), ());

	#[inline]
	unsafe fn resolve(&self, _: usize, _: ((), ()), out: *mut Self::Archived) {
		out.write(*self);
	}
}

impl<D: rkyv::Fallible + ?Sized> rkyv::Deserialize<Duration, D> for rkyv::Archived<Duration> {
	#[inline]
	fn deserialize(&self, _: &mut D) -> Result<Duration, D::Error> {
		Ok(*self)
	}
}

impl<S: rkyv::Fallible + ?Sized> rkyv::Serialize<S> for Duration {
	#[inline]
	fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		rkyv::Serialize::serialize(unsafe { std::mem::transmute::<_, &(i64, i32)>(self) }, serializer)
	}
}

#[test]
fn rkyv_test() {
	let duration = Duration(chrono::Duration::seconds(15));
	let bytes = rkyv::to_bytes::<_, 256>(&duration).unwrap();
	println!("bytes: {bytes:?}");
	let archived = unsafe { rkyv::archived_root::<Duration>(&bytes) };
	println!("archived: {archived:?}");
	let duration_again: Duration = rkyv::Deserialize::deserialize(archived, &mut rkyv::Infallible).unwrap();
	assert_eq!(duration, duration_again);
}
