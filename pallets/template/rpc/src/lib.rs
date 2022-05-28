//! RPC interface for the template pallet.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use pallet_template_runtime_api::PalletTemplateRuntimeApi;

#[rpc]
pub trait PalletTemplateRPCApi<BlockHash> {
    #[rpc(name = "template_getnum")]
    fn get_num(
        &self,
        at: Option<BlockHash>
    ) -> Result<u32>;
}

/// A struct that implements the `PalletTemplateRPCApi`.
pub struct PalletTemplate<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> PalletTemplate<C, M> {
    /// Create new `Template` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self { client, _marker: Default::default() }
    }
}

impl<C, Block> PalletTemplateRPCApi<<Block as BlockT>::Hash>
    for PalletTemplate<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: PalletTemplateRuntimeApi<Block>,
{
    fn get_num(
        &self,
        at: Option<<Block as BlockT>::Hash>
    ) -> Result<u32> {

        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

        let runtime_api_result = api.get_num(&at);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}