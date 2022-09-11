use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

#[test]
fn it_works_simply() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(CustomModule::add_member(Origin::root(), H256::default(), 123));
		// Read pallet storage and assert an expected result.
		assert_eq!(CustomModule::clubs(H256::default()), Some(vec![123]));

		// Dispatch a signed extrinsic.
		assert_ok!(CustomModule::remove_member(Origin::root(), H256::default(), 123));
		// Read pallet storage and assert an expected result.
		assert_eq!(CustomModule::clubs(H256::default()), Some(vec![]));
	});
}

#[test]
fn correct_error_for_none_root() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when non-`Root` calling the pallet.
		assert_noop!(
			CustomModule::add_member(Origin::signed(1), H256::default(), 2),
			frame_support::error::BadOrigin,
		);
		assert_noop!(
			CustomModule::remove_member(Origin::signed(1), H256::default(), 2),
			frame_support::error::BadOrigin,
		);
	});
}

#[test]
fn correct_error_for_no_club_exist() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no `club` exists.
		assert_noop!(
			CustomModule::remove_member(Origin::root(), H256::default(), 2),
			Error::<Test>::ClubNotExists,
		);
	});
}
