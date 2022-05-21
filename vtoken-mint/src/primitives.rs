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

/// Trait for others module to access vtoken-mint module
pub trait VtokenMintExt {
    /// The currency identifier.
    type CurrencyId: FullCodec
    + Eq
    + PartialEq
    + Copy
    + MaybeSerializeDeserialize
    + Debug
    + CurrencyIdExt;

    /// The balance of an account.
    type Balance: AtLeast32BitUnsigned
    + FullCodec
    + Copy
    + MaybeSerializeDeserialize
    + Debug
    + Default;

    /// Get mint pool by currency id
    fn get_mint_pool(currency_id: Self::CurrencyId) -> Self::Balance;

    /// Expand mint pool
    fn expand_mint_pool(currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult;

    /// Reduce mint pool
    fn reduce_mint_pool(currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult;
}
