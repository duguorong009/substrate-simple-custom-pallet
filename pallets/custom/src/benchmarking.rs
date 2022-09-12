//! Benchmarking setup for pallet-custom

use super::*;

#[allow(unused)]
use crate::Pallet as Custom;
use core::hash::Hash;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_member {
		let s in 0..100;
		// let caller: T::AccountId = whitelisted_caller();
		let club = T::Hashing::hash("club");
		let member = whitelisted_caller();
	}: _(RawOrigin::Root, club, member)
	verify {
		assert_eq!(Clubs::<T>::get(club).unwrap().len(), s as usize + 1);
	}

	impl_benchmark_test_suite!(Custom, crate::mock::new_test_ext(), crate::mock::Test);
}
