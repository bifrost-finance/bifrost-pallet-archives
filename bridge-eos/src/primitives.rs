// This file is part of Bifrost.

// Copyright (C) 2019-2022 Liebi Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use codec::{Decode, Encode};
use sp_std::prelude::*;

/// Blockchain types
#[derive(PartialEq, Debug, Clone, Encode, Decode)]
pub enum BlockchainType {
	BIFROST,
	EOS,
	IOST,
}

impl Default for BlockchainType {
	fn default() -> Self {
		BlockchainType::BIFROST
	}
}

/// Symbol type of bridge asset
#[derive(Clone, Default, Encode, Decode)]
pub struct BridgeAssetSymbol<Precision> {
	pub blockchain: BlockchainType,
	pub symbol: Vec<u8>,
	pub precision: Precision,
}

impl<Precision> BridgeAssetSymbol<Precision> {
	pub fn new(blockchain: BlockchainType, symbol: Vec<u8>, precision: Precision) -> Self {
		BridgeAssetSymbol { blockchain, symbol, precision }
	}
}

/// Bridge asset type
#[derive(Clone, Default, Encode, Decode)]
pub struct BridgeAssetBalance<AccountId, AssetId, Precision, Balance> {
	pub symbol: BridgeAssetSymbol<Precision>,
	pub amount: Balance,
	pub memo: Vec<u8>,
	// store the account who send transaction to EOS
	pub from: AccountId,
	// which token type is sent to EOS
	pub asset_id: AssetId,
}

/// Bridge asset from Bifrost to other blockchain
pub trait BridgeAssetTo<AccountId, CurrencyId, Precision, Balance> {
	type Error;
	fn bridge_asset_to(
		target: Vec<u8>,
		bridge_asset: BridgeAssetBalance<AccountId, CurrencyId, Precision, Balance>,
	) -> Result<(), Self::Error>;
	fn redeem(
		asset_id: CurrencyId,
		amount: Balance,
		validator_address: Vec<u8>,
	) -> Result<(), Self::Error>;
	fn stake(
		asset_id: CurrencyId,
		amount: Balance,
		validator_address: Vec<u8>,
	) -> Result<(), Self::Error>;
	fn unstake(
		asset_id: CurrencyId,
		amount: Balance,
		validator_address: Vec<u8>,
	) -> Result<(), Self::Error>;
}
