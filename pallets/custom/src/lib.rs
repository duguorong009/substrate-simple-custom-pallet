#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a claim has been created.
		MemberAdded { who: T::AccountId, club: T::Hash },
		/// Event emitted when a claim is revoked by the owner.
		MemberRemoved { who: T::AccountId, club: T::Hash },
	}
	#[pallet::error]
	pub enum Error<T> {
		/// The club does not exist
		ClubNotExists,
		/// The member already exists in club.
		MemberAlreadyExists,
		/// The member does not exist, so it cannot be removed.
		NoSuchMember,
		/// Not `Root` origin caller
		NotRootOrigin,
	}
	#[pallet::storage]
	pub(super) type Clubs<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::AccountId>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn add_member(
			origin: OriginFor<T>,
			club: T::Hash,
			member: T::AccountId,
		) -> DispatchResult {
			// Check that the extrinsic was only called by `Root` origin
			ensure_root(origin).map_err(|_e| Error::<T>::NotRootOrigin)?;

			// Check if `club` exists & `member` already exists in the club
			let mut members = Clubs::<T>::get(&club).ok_or(Error::<T>::ClubNotExists)?;
			match members.iter().position(|m| m == &member) {
				Some(id) => return Err(Error::<T>::MemberAlreadyExists),
				None => {
					// Add the `member` to the `club` & save the result.
					members.push(member);
					Clubs::<T>::insert(club, members);
				},
			}

			// Emit an event that the `member` was added.
			Self::deposit_event(Event::MemberAdded { who: member, club });

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn remove_member(
			origin: OriginFor<T>,
			club: T::Hash,
			member: T::AccountId,
		) -> DispatchResult {
			// Check that the extrinsic was only called by `Root` origin
			ensure_root(origin).map_err(|_e| Error::<T>::NotRootOrigin)?;

			// Check if `member` exists in `club`, if not return an error.
			let mut members = Clubs::<T>::get(&club).ok_or(Error::<T>::ClubNotExists)?;
			match members.iter().position(|m| m == &member) {
				Some(id) => {
					// Remove the `member` from the `club` & save the result.
					members.swap_remove(id);
					Clubs::<T>::insert(club, members);
				},
				None => return Err(Error::<T>::NoSuchMember),
			}

			// Emit an event that the `member` was removed.
			Self::deposit_event(Event::MemberRemoved { who: member, club });

			Ok(())
		}
	}
}
