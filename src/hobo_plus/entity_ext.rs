use hobo::prelude::*;
use futures::future::FutureExt;

pub trait AsEntityExt: AsEntity {
	fn bundle<C: 'static>(self, x: C) -> Self where Self: Sized { self.add_bundle(x); self }
	fn add_bundle<C: 'static>(&self, x: C) { self.get_cmp_mut_or_default::<Vec<C>>().push(x) }

	fn get_mutable_write<T: 'static>(&self) -> hobo::owning_ref::OwningHandle<
		hobo::owning_ref::OwningRef<
			StorageGuard<
				hobo::signals::signal::Mutable<T>,
				hobo::owning_ref::OwningRef<
					std::cell::Ref<'static, Box<(dyn DynStorage + 'static)>>,
					SimpleStorage<hobo::signals::signal::Mutable<T>>
				>,
			>,
			hobo::signals::signal::Mutable<T>,
		>,
		hobo::signals::signal::MutableLockMut<'static, T>,
	> { hobo::owning_ref::OwningHandle::new_with_fn(self.get_cmp::<hobo::signals::signal::Mutable<T>>(), |x| unsafe { (*x).lock_mut() }) }

	fn get_mutable_read<T: 'static>(&self) -> hobo::owning_ref::OwningHandle<
		hobo::owning_ref::OwningRef<
			StorageGuard<
				hobo::signals::signal::Mutable<T>,
				hobo::owning_ref::OwningRef<
					std::cell::Ref<'static, Box<(dyn DynStorage + 'static)>>,
					SimpleStorage<hobo::signals::signal::Mutable<T>>
				>,
			>,
			hobo::signals::signal::Mutable<T>,
		>,
		hobo::signals::signal::MutableLockRef<'static, T>,
	> { hobo::owning_ref::OwningHandle::new_with_fn(self.get_cmp::<hobo::signals::signal::Mutable<T>>(), |x| unsafe { (*x).lock_ref() }) }

	#[track_caller]
	fn spawn_complain<T>(&self, f: impl std::future::Future<Output = anyhow::Result<T>> + 'static) {
		let caller = std::panic::Location::caller();
		let (handle, fut) = hobo::signals::cancelable_future(f.map(|res| if let Err(e) = res {
			let lvl = log::Level::Error;
			if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
				log::__private_api_log(
					log::__log_format_args!("{:?}", e),
					lvl,
					&(log::__log_module_path!(), log::__log_module_path!(), caller.file(), caller.line()),
					log::__private_api::Option::None,
				);
			}
		}), Default::default);
		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<hobo::entity::FutureHandlesCollection>().0.push(handle);
	}

	fn spawn_complain_in<F: FnOnce(&Self) -> Fut, Fut: std::future::Future<Output = anyhow::Result<T>> + 'static, T>(self, f: F) -> Self where Self: Sized { self.spawn_complain(f(&self)); self }
}

impl<T: AsEntity> AsEntityExt for T {}