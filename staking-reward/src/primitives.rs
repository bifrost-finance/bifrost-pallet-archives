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

pub trait RewardTrait<Balance, AccountId, CurrencyId> {
    type Error;
    fn record_reward(
        v_token_id: CurrencyId,
        vtoken_mint_amount: Balance,
        referer: AccountId,
    ) -> Result<(), Self::Error>;
    fn dispatch_reward(v_token_id: CurrencyId, staking_profit: Balance) -> Result<(), Self::Error>;
}
