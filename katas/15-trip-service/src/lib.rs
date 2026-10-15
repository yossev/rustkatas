//! Kata 15 — Trip Service
//!
//! A travel startup is about to be acquired, and the buyers want test
//! coverage before they sign. One module has none: the bit that decides
//! whether you can see a friend's holiday photos. It reaches straight into
//! a live session and a live database, so every test either crashes or
//! hits production. The founder is nervous. The lawyers are waiting.
//!
//! Whisper: you can't test it until you can get between it and its friends.
//!
//! Read README.md first.
//!
//! RULES: you may NOT change the `user_session` or `trip_dao` modules.
//! In `TripService`, until it's covered by tests, only make small, safe,
//! mechanical changes (extract a function, introduce a parameter, add a trait).

#[derive(Debug, Clone, PartialEq)]
pub struct Trip {
    pub destination: String,
}

impl Trip {
    pub fn to(destination: &str) -> Trip {
        Trip {
            destination: destination.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    friends: Vec<User>,
    trips: Vec<Trip>,
}

impl User {
    pub fn new(name: &str) -> User {
        User {
            name: name.to_string(),
            friends: Vec::new(),
            trips: Vec::new(),
        }
    }

    pub fn add_friend(&mut self, friend: User) {
        self.friends.push(friend);
    }

    pub fn add_trip(&mut self, trip: Trip) {
        self.trips.push(trip);
    }

    pub fn get_friends(&self) -> &[User] {
        &self.friends
    }

    pub fn trips(&self) -> &[Trip] {
        &self.trips
    }
}

#[derive(Debug, PartialEq)]
pub struct UserNotLoggedIn;

/// Talks to the real web session. DO NOT CHANGE.
pub mod user_session {
    use crate::User;

    pub fn get_logged_user() -> Option<User> {
        panic!("user_session::get_logged_user() should not be called in a unit test");
    }
}

/// Talks to the real database. DO NOT CHANGE.
pub mod trip_dao {
    use crate::{Trip, User};

    pub fn find_trips_by_user(_user: &User) -> Vec<Trip> {
        panic!("trip_dao::find_trips_by_user() should not be called in a unit test");
    }
}

pub struct TripService;

impl TripService {
    pub fn get_trips_by_user(&self, user: &User) -> Result<Vec<Trip>, UserNotLoggedIn> {
        let mut trip_list = Vec::new();
        let logged_user = user_session::get_logged_user();
        let mut is_friend = false;
        if let Some(logged_user) = logged_user {
            for friend in user.get_friends() {
                if *friend == logged_user {
                    is_friend = true;
                    break;
                }
            }
            if is_friend {
                trip_list = trip_dao::find_trips_by_user(user);
            }
            Ok(trip_list)
        } else {
            Err(UserNotLoggedIn)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This is the problem. Any call reaches the real session.
    #[test]
    #[should_panic(expected = "should not be called in a unit test")]
    fn the_service_cannot_be_tested_as_it_is() {
        TripService.get_trips_by_user(&User::new("bob")).unwrap();
    }

    // Once you've created seams, write these. Delete the test above when
    // it no longer makes sense.

    #[test]
    #[ignore]
    fn fails_when_nobody_is_logged_in() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn returns_no_trips_when_users_are_not_friends() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn returns_trips_when_users_are_friends() {
        todo!("YOUR TEST")
    }

    #[test]
    #[ignore]
    fn friendship_is_checked_from_the_users_friend_list() {
        todo!("YOUR TEST: the logged user has the target as a friend, but not vice versa. What happens?")
    }
}
