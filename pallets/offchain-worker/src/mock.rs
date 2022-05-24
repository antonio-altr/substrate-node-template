use crate as pallet_example_offchain_worker;
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
use frame_system as system;
use parity_scale_codec::alloc::sync::Arc;
use parking_lot::RwLock;
use sp_core::{
    H256,
    offchain::{
		testing::{OffchainState, PoolState, TestOffchainExt, TestTransactionPoolExt},
		OffchainDbExt, TransactionPoolExt,
	},
};
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_io::TestExternalities;
use sp_runtime::{
    MultiSignature,
	testing::{Header, TestXt},
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestOCWRuntime>;
type Block = frame_system::mocking::MockBlock<TestOCWRuntime>;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// For testing the module, we construct a mock runtime.
frame_support::construct_runtime!(
	pub enum TestOCWRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		OffChainWorker: pallet_example_offchain_worker::{Pallet, Call, Storage, Event<T>, ValidateUnsigned},
	}
);

impl frame_system::Config for TestOCWRuntime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

frame_support::parameter_types! {
	pub const UnsignedPriority: u64 = 1 << 20;
}

impl pallet_example_offchain_worker::Config for TestOCWRuntime {
	type AuthorityId = pallet_example_offchain_worker::crypto::TestAuthId;
	type Event = Event;
	type Call = Call;
	type GracePeriod = ConstU64<5>;
	type UnsignedInterval = ConstU64<128>;
	type UnsignedPriority = UnsignedPriority;
	type MaxPrices = ConstU32<64>;
}

impl frame_system::offchain::SigningTypes for TestOCWRuntime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

pub type Extrinsic = TestXt<Call, ()>;

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for TestOCWRuntime
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for TestOCWRuntime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as sp_runtime::traits::Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(
		Call,
		<Extrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		Some((call, (nonce, ())))
	}
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> (
		TestExternalities,
		Arc<RwLock<PoolState>>,
		Arc<RwLock<OffchainState>>,
	) {
		const PHRASE: &str =
			"expire stage crawl shell boss any story swamp skull yellow bamboo copy";

		let (offchain, offchain_state) = TestOffchainExt::new();
		let (pool, pool_state) = TestTransactionPoolExt::new();
		let keystore = KeyStore::new();
		keystore
			.sr25519_generate_new(super::KEY_TYPE, Some(&format!("{}/hunter1", PHRASE)))
			.unwrap();

		let storage = frame_system::GenesisConfig::default()
			.build_storage::<TestOCWRuntime>()
			.unwrap();

		let mut t = TestExternalities::from(storage);
		t.register_extension(OffchainDbExt::new(offchain));
		t.register_extension(TransactionPoolExt::new(pool));
		t.register_extension(KeystoreExt(Arc::new(keystore)));
		t.execute_with(|| System::set_block_number(1));
		(t, pool_state, offchain_state)
	}
}
