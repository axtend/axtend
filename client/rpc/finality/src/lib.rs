// Copyright 2019-2021 PureStake Inc.
// This file is part of Axtend.

// Axtend is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axtend is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axtend.  If not, see <http://www.gnu.org/licenses/>.
use fc_rpc::frontier_backend_client::{self, is_canon};
use futures::{future::BoxFuture, FutureExt as _};
use jsonrpc_core::Result as RpcResult;
use jsonrpc_derive::rpc;
use sp_core::H256;
use std::{marker::PhantomData, sync::Arc};
//TODO ideally we wouldn't depend on BlockId here. Can we change frontier
// so it's load_hash helper returns an H256 instead of wrapping it in a BlockId?
use fc_db::Backend as FrontierBackend;
use sp_api::BlockId;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block;

/// An RPC endpoint to check for finality of blocks and transactions in Axtend
#[rpc(server)]
pub trait AxtendFinalityApi {
	/// Reports whether a Axlib or Ethereum block is finalized.
	/// Returns false if the block is not found.
	#[rpc(name = "moon_isBlockFinalized")]
	fn is_block_finalized(&self, block_hash: H256) -> BoxFuture<'static, RpcResult<bool>>;

	/// Reports whether an Ethereum transaction is finalized.
	/// Returns false if the transaction is not found
	#[rpc(name = "moon_isTxFinalized")]
	fn is_tx_finalized(&self, tx_hash: H256) -> BoxFuture<'static, RpcResult<bool>>;
}

pub struct AxtendFinality<B: Block, C> {
	pub backend: Arc<FrontierBackend<B>>,
	pub client: Arc<C>,
	_phdata: PhantomData<B>,
}

impl<B: Block, C> AxtendFinality<B, C> {
	pub fn new(client: Arc<C>, backend: Arc<FrontierBackend<B>>) -> Self {
		Self {
			backend,
			client,
			_phdata: Default::default(),
		}
	}
}

impl<B, C> AxtendFinalityApi for AxtendFinality<B, C>
where
	B: Block<Hash = H256>,
	C: HeaderBackend<B> + Send + Sync + 'static,
{
	fn is_block_finalized(&self, raw_hash: H256) -> BoxFuture<'static, RpcResult<bool>> {
		let backend = self.backend.clone();
		let client = self.client.clone();
		async move { is_block_finalized_inner::<B, C>(&backend, &client, raw_hash) }.boxed()
	}

	fn is_tx_finalized(&self, tx_hash: H256) -> BoxFuture<'static, RpcResult<bool>> {
		let backend = self.backend.clone();
		let client = self.client.clone();
		async move {
			if let Some((ethereum_block_hash, _ethereum_index)) =
				frontier_backend_client::load_transactions::<B, C>(
					&client,
					backend.as_ref(),
					tx_hash,
					true,
				)? {
				is_block_finalized_inner::<B, C>(&backend, &client, ethereum_block_hash)
			} else {
				Ok(false)
			}
		}
		.boxed()
	}
}

fn is_block_finalized_inner<B: Block<Hash = H256>, C: HeaderBackend<B> + 'static>(
	backend: &FrontierBackend<B>,
	client: &C,
	raw_hash: H256,
) -> RpcResult<bool> {
	let axlib_hash = match frontier_backend_client::load_hash::<B>(backend, raw_hash)? {
		// If we find this hash in the frontier data base, we know it is an eth hash
		Some(BlockId::Hash(hash)) => hash,
		Some(BlockId::Number(_)) => panic!("is_canon test only works with hashes."),
		// Otherwise, we assume this is a Axlib hash.
		None => raw_hash,
	};

	// First check whether the block is in the best chain
	if !is_canon(client, axlib_hash) {
		return Ok(false);
	}

	// At this point we know the block in question is in the current best chain.
	// It's just a question of whether it is in the finalized prefix or not
	let query_height = client
		.number(axlib_hash)
		.expect("No sp_blockchain::Error should be thrown when looking up hash")
		.expect("Block is already known to be canon, so it must be in the chain");
	let finalized_height = client.info().finalized_number;

	Ok(query_height <= finalized_height)
}
