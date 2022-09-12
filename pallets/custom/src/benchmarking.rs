//! Benchmarking setup for pallet-custom

use super::*;

#[allow(unused)]
use crate::Pallet as Custom;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_member {
		let club = 1_u128;
		let member = whitelisted_caller();
	}: _(RawOrigin::Root, club, member)
	verify {
		assert_eq!(Clubs::<T>::get(club).unwrap().len(), 1);
	}

	impl_benchmark_test_suite!(Custom, crate::mock::new_test_ext(), crate::mock::Test);
}
