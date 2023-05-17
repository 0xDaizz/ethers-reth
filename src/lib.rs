pub mod middleware;
mod utils;
use init::{init_client, init_eth_api, init_eth_filter};
use jsonrpsee::types::ErrorObjectOwned;
use reth_rpc::{EthFilter, TraceApi};
use std::sync::Arc;
use thiserror::Error;
pub mod init;

use ethers::providers::{Middleware, MiddlewareError};

use reth_beacon_consensus::BeaconConsensus;
use reth_blockchain_tree::ShareableBlockchainTree;
use reth_db::mdbx::{Env, NoWriteMap};
use reth_network_api::test_utils::NoopNetwork;
use reth_provider::providers::BlockchainProvider;
use reth_revm::Factory;
use reth_rpc::EthApi;
use reth_transaction_pool::{CostOrdering, EthTransactionValidator, Pool, PooledTransaction};
use std::path::Path;

pub type RethClient = BlockchainProvider<
    Arc<Env<NoWriteMap>>,
    ShareableBlockchainTree<Arc<Env<NoWriteMap>>, Arc<BeaconConsensus>, Factory>,
>;

pub type RethTxPool =
    Pool<EthTransactionValidator<RethClient, PooledTransaction>, CostOrdering<PooledTransaction>>;

pub type RethApi = EthApi<RethClient, RethTxPool, NoopNetwork>;
pub type RethFilter = EthFilter<RethClient, RethTxPool>;
pub type RethTrace = TraceApi<RethClient, RethTxPool>;

#[derive(Clone)]
pub struct RethMiddleware<M> {
    inner: M,
    reth_api: RethApi,
    reth_filter: RethFilter,
    reth_trace: RethTrace,
}

impl<M: std::fmt::Debug> std::fmt::Debug for RethMiddleware<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RethMiddleware").field("inner", &self.inner).finish_non_exhaustive()
    }
}

#[derive(Error, Debug)]
pub enum RethMiddlewareError<M: Middleware> {
    /// An error occured in one of the middlewares.
    #[error("{0}")]
    MiddlewareError(M::Error),

    /// An error occurred in the Reth API.
    #[error(transparent)]
    RethApiError(#[from] ErrorObjectOwned),
}

impl<M: Middleware> MiddlewareError for RethMiddlewareError<M> {
    type Inner = M::Error;

    fn from_err(e: Self::Inner) -> Self {
        RethMiddlewareError::MiddlewareError(e)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            RethMiddlewareError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}

impl<M> RethMiddleware<M>
where
    M: Middleware,
{
    pub fn new(inner: M, db_path: &Path) -> Self {
        let client = init_client(db_path);
        // EthApi -> EthApi<Client, Pool, Network>
        let api = init_eth_api(client.clone());
        // EthFilter -> EthFilter<Client, Pool>
        // TODO: figure out default max_logs
        let filter = init_eth_filter(client.clone(), 1000);

        let trace = TraceApi::new(client.clone(), api.clone(), todo!());

        Self { inner, reth_api: api, reth_filter: filter }
    }

    pub fn reth_api(&self) -> &NodeEthApi {
        &self.reth_api
    }
}
