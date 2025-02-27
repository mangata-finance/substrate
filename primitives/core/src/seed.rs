use crate::hash::{H256, H512};
use codec::{Decode, Encode};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use scale_info::TypeInfo;

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, Default, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// stores information needed to verify if
/// shuffling seed was generated properly
pub struct ShufflingSeed {
	/// shuffling seed for the previous block
	pub seed: H256,
	/// seed signature
	pub proof: H512,
}

#[cfg(feature = "std")]
impl parity_util_mem::MallocSizeOf for ShufflingSeed {
	fn size_of(&self, ops: &mut parity_util_mem::MallocSizeOfOps) -> usize {
		self.seed.size_of(ops) + self.proof.size_of(ops)
	}
}
