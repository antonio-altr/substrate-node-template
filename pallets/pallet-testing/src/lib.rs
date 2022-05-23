#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

    #[pallet::error]
    pub enum Error<T> {
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// Value must be <= MaxValue constant
        ValueBiggerThanMax
    }

    #[pallet::storage]
    #[pallet::getter(fn total)]
    pub type Total<T> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        TotalUpdated(u32, T::AccountId),
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        #[pallet::constant]
        type MaxValue: Get<u32>;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]

        pub fn add_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(value <= T::MaxValue::get(), Error::<T>::ValueBiggerThanMax);

            // Read the total from storage.
            match <Total<T>>::get() {
                // Return an error if there is no total.
                None => {
                    // If None, set storage to value
                    <Total<T>>::put(value);
                    // Emit an event.
                    Self::deposit_event(Event::TotalUpdated(value, who));
                    // Return a successful DispatchResultWithPostInfo
                    Ok(())
                },
                // If total already exists, then get sum of value and total
                Some(curr_total) => {
                    // Add the value to total
                    let new_total = curr_total.checked_add(value).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the total in storage
                    <Total<T>>::put(new_total);
                    // Emit an event.
                    Self::deposit_event(Event::TotalUpdated(new_total, who));
                    // Return a successful DispatchResultWithPostInfo
                    Ok(())
                },
            }
        }
    }
}