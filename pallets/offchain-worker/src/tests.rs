use crate as pallet_example_offchain_worker;
use crate::mock::*;

#[test]
fn test_send_raw_unsigned() {
	let (mut t, pool_state, _offchain_state) = ExternalityBuilder::build();

	t.execute_with(|| {
		// when
		let block_num: u64 = 32;
        let dummy_price: u32 = 100;
		OffChainWorker::send_raw_unsigned(block_num, dummy_price).unwrap();
		// then
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());
        let ex: Extrinsic = parity_scale_codec::Decode::decode(&mut &*tx).unwrap();
		assert_eq!(ex.signature, None);
		assert_eq!(
			ex.call,
			Call::OffChainWorker(pallet_example_offchain_worker::Call::submit_price_unsigned { block_number: block_num, price: dummy_price })
		);
	});
}