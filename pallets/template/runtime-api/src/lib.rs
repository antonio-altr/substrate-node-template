#![cfg_attr(not(feature = "std"), no_std)]

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait PalletTemplateRuntimeApi {
		fn get_num() -> u32;
	}
}
