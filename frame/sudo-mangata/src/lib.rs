// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Sudo Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! The Sudo pallet allows for a single account (called the "sudo key")
//! to execute dispatchable functions that require a `Root` call
//! or designate a new account to replace them as the sudo key.
//! Only one account can be the sudo key at a time.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! Only the sudo key can call the dispatchable functions from the Sudo pallet.
//!
//! * `sudo` - Make a `Root` call to a dispatchable function.
//! * `set_key` - Assign a new account to be the sudo key.
//!
//! ## Usage
//!
//! ### Executing Privileged Functions
//!
//! The Sudo pallet itself is not intended to be used within other pallets.
//! Instead, you can build "privileged functions" (i.e. functions that require `Root` origin) in
//! other pallets. You can execute these privileged functions by calling `sudo` with the sudo key
//! account. Privileged functions cannot be directly executed via an extrinsic.
//!
//! Learn more about privileged functions and `Root` origin in the [`Origin`] type documentation.
//!
//! ### Simple Code Snippet
//!
//! This is an example of a pallet that exposes a privileged function:
//!
//! ```
//! #[frame_support::pallet]
//! pub mod pallet {
//! 	use super::*;
//! 	use frame_support::pallet_prelude::*;
//! 	use frame_system::pallet_prelude::*;
//!
//! 	#[pallet::pallet]
//! 	pub struct Pallet<T>(_);
//!
//! 	#[pallet::config]
//! 	pub trait Config: frame_system::Config {}
//!
//! 	#[pallet::call]
//! 	impl<T: Config> Pallet<T> {
//! 		#[pallet::weight(0)]
//!         pub fn privileged_function(origin: OriginFor<T>) -> DispatchResult {
//!             ensure_root(origin)?;
//!
//!             // do something...
//!
//!             Ok(())
//!         }
//! 	}
//! }
//! # fn main() {}
//! ```
//!
//! ## Genesis Config
//!
//! The Sudo pallet depends on the [`GenesisConfig`].
//! You need to set an initial superuser account as the sudo `key`.
//!
//! ## Related Pallets
//!
//! * [Democracy](../pallet_democracy/index.html)
//!
//! [`Origin`]: https://docs.substrate.io/main-docs/build/origins/

#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{traits::StaticLookup, DispatchResult};
use sp_std::prelude::*;

use frame_support::{dispatch::GetDispatchInfo, traits::UnfilteredDispatchable};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub(crate) const LOG_TARGET: &'static str = "sudo-mangata";
pub(crate) const ALERT_STRING: &'static str = "ALERT!ALERT!ALERT!";

// syntactic sugar for logging.
#[macro_export]
macro_rules! alert_log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] {:?} ", $patter), <frame_system::Pallet<T>>::block_number(), crate::ALERT_STRING $(, $values)*
		)
	};
}

pub use pallet::*;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

#[frame_support::pallet]
pub mod pallet {
	use super::{DispatchResult, *};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A sudo-able call.
		type RuntimeCall: Parameter
			+ UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ GetDispatchInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Authenticates the sudo key and dispatches a function call with `Root` origin.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// # <weight>
		/// - O(1).
		/// - Limited storage reads.
		/// - One DB write (event).
		/// - Weight of derivative `call` execution + 10,000.
		/// # </weight>
		#[pallet::call_index(0)]
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(dispatch_info.weight, dispatch_info.class)
		})]
		pub fn sudo(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			// This is a public call, so we ensure that the origin is some signed account.
			let sender = ensure_signed(origin)?;
			ensure!(Self::key().map_or(false, |k| sender == k), Error::<T>::RequireSudo);

			let res = call.clone().dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
			Self::deposit_event(Event::Sudid {
				sudo_result: res.clone().map(|_| ()).map_err(|e| e.error),
			});
			alert_log!(info, "A sudo action was performed: Call - {:?}, Result - {:?}!", call, res);
			// Sudo user does not pay a fee.
			Ok(Pays::No.into())
		}

		/// Authenticates the sudo key and dispatches a function call with `Root` origin.
		/// This function does not check the weight of the call, and instead allows the
		/// Sudo user to specify the weight of the call.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// # <weight>
		/// - O(1).
		/// - The weight of this call is defined by the caller.
		/// # </weight>
		#[pallet::call_index(1)]
		#[pallet::weight((*_weight, call.get_dispatch_info().class))]
		pub fn sudo_unchecked_weight(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
			_weight: Weight,
		) -> DispatchResultWithPostInfo {
			// This is a public call, so we ensure that the origin is some signed account.
			let sender = ensure_signed(origin)?;
			ensure!(Self::key().map_or(false, |k| sender == k), Error::<T>::RequireSudo);

			let res = call.clone().dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
			Self::deposit_event(Event::Sudid {
				sudo_result: res.clone().map(|_| ()).map_err(|e| e.error),
			});
			alert_log!(
				info,
				"A sudo action was performed with unchecked weight: Call - {:?}, Result - {:?}!",
				call,
				res
			);
			// Sudo user does not pay a fee.
			Ok(Pays::No.into())
		}

		/// Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo
		/// key.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// # <weight>
		/// - O(1).
		/// - Limited storage reads.
		/// - One DB change.
		/// # </weight>
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn set_key(
			origin: OriginFor<T>,
			new: AccountIdLookupOf<T>,
		) -> DispatchResultWithPostInfo {
			// This is a public call, so we ensure that the origin is some signed account.
			let sender = ensure_signed(origin)?;
			ensure!(Self::key().map_or(false, |k| sender == k), Error::<T>::RequireSudo);
			let new = T::Lookup::lookup(new)?;

			Self::deposit_event(Event::KeyChanged { old_sudoer: Key::<T>::get() });
			alert_log!(info, "sudo key was changed: New Key - {:?}!", new.clone(),);
			Key::<T>::put(&new);
			// Sudo user does not pay a fee.
			Ok(Pays::No.into())
		}

		/// Authenticates the sudo key and dispatches a function call with `Signed` origin from
		/// a given account.
		///
		/// The dispatch origin for this call must be _Signed_.
		///
		/// # <weight>
		/// - O(1).
		/// - Limited storage reads.
		/// - One DB write (event).
		/// - Weight of derivative `call` execution + 10,000.
		/// # </weight>
		#[pallet::call_index(3)]
		#[pallet::weight({
			let dispatch_info = call.get_dispatch_info();
			(
				dispatch_info.weight
					// AccountData for inner call origin accountdata.
					.saturating_add(T::DbWeight::get().reads_writes(1, 1)),
				dispatch_info.class,
			)
		})]
		pub fn sudo_as(
			origin: OriginFor<T>,
			who: AccountIdLookupOf<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResultWithPostInfo {
			// This is a public call, so we ensure that the origin is some signed account.
			let sender = ensure_signed(origin)?;
			ensure!(Self::key().map_or(false, |k| sender == k), Error::<T>::RequireSudo);

			let who = T::Lookup::lookup(who)?;

			let res = call
				.clone()
				.dispatch_bypass_filter(frame_system::RawOrigin::Signed(who.clone()).into());

			Self::deposit_event(Event::SudoAsDone {
				sudo_result: res.clone().map(|_| ()).map_err(|e| e.error),
			});
			alert_log!(
				info,
				"A sudo_as action was performed: Who - {:?}, Call - {:?}, Result - {:?}!",
				who,
				call,
				res
			);
			// Sudo user does not pay a fee.
			Ok(Pays::No.into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A sudo just took place. \[result\]
		Sudid { sudo_result: DispatchResult },
		/// The \[sudoer\] just switched identity; the old key is supplied if one existed.
		KeyChanged { old_sudoer: Option<T::AccountId> },
		/// A sudo just took place. \[result\]
		SudoAsDone { sudo_result: DispatchResult },
	}

	#[pallet::error]
	/// Error for the Sudo pallet
	pub enum Error<T> {
		/// Sender must be the Sudo account
		RequireSudo,
	}

	/// The `AccountId` of the sudo key.
	#[pallet::storage]
	#[pallet::getter(fn key)]
	pub(super) type Key<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// The `AccountId` of the sudo key.
		pub key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref key) = self.key {
				Key::<T>::put(key);
			}
		}
	}
}
