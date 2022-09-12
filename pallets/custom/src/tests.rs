use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const CLUB_ID: u128 = 1;

#[test]
fn it_works_simply() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(CustomModule::add_member(Origin::root(), CLUB_ID, 123));
		// Read pallet storage and assert an expected result.
		assert_eq!(CustomModule::clubs(CLUB_ID), Some(vec![123]));

		// Dispatch a signed extrinsic.
		assert_ok!(CustomModule::remove_member(Origin::root(), CLUB_ID, 123));
		// Read pallet storage and assert an expected result.
		assert_eq!(CustomModule::clubs(CLUB_ID), Some(vec![]));
	});
}

#[test]
fn correct_error_for_none_root() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when non-`Root` calling the pallet.
		assert_noop!(
			CustomModule::add_member(Origin::signed(1), CLUB_ID, 2),
			frame_support::error::BadOrigin,
		);
		assert_noop!(
			CustomModule::remove_member(Origin::signed(1), CLUB_ID, 2),
			frame_support::error::BadOrigin,
		);
	});
}

#[test]
fn correct_error_for_no_club_exist() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no `club` exists.
		assert_noop!(
			CustomModule::remove_member(Origin::root(), CLUB_ID, 2),
			Error::<Test>::ClubNotExists,
		);
	});
}

#[test]
fn correct_error_when_no_member_to_remove() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(CustomModule::add_member(Origin::root(), CLUB_ID, 123));

		// Ensure the expected error is thrown when no `club` exists.
		assert_noop!(
			CustomModule::remove_member(Origin::root(), CLUB_ID, 234),
			Error::<Test>::NoSuchMember,
		);
	});
}

#[test]
fn correct_error_when_already_member_exist() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(CustomModule::add_member(Origin::root(), CLUB_ID, 123));

		// Ensure the expected error is thrown when no `club` exists.
		assert_noop!(
			CustomModule::add_member(Origin::root(), CLUB_ID, 123),
			Error::<Test>::AlreadyMember,
		);
	});
}
