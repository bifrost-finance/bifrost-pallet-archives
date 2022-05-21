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

pub trait BancorHandler<Balance> {
    fn add_token(currency_id: super::CurrencyId, amount: Balance) -> DispatchResult;
}

impl<Balance> BancorHandler<Balance> for () {
    fn add_token(_currency_id: super::CurrencyId, _amount: Balance) -> DispatchResult {
        DispatchResult::from(DispatchError::Token(TokenError::NoFunds))
    }
}
