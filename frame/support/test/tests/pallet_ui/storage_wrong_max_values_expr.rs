#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::{Hooks, Twox64Concat, StorageMap};
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage(max_values = 4u64)]
	type Foo<T> = StorageMap<_, Twox64Concat, u32, u32>;
}

fn main() {
}
