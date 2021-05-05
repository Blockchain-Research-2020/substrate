// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implementation of trait `StoragesInfo` on module structure.

use proc_macro2::TokenStream;
use quote::quote;
use super::{DeclStorageDefExt, StorageLineTypeDef};

pub fn impl_storages_info(scrate: &TokenStream, def: &DeclStorageDefExt) -> TokenStream {
	let mut entries = TokenStream::new();

	for line in def.storage_lines.iter() {
		let storage_struct = &line.storage_struct;
		let value_type = &line.value_type;
		let max_values = &line.max_values;
		let storage_generator_trait = &line.storage_generator_trait;
		let entry = match &line.storage_type {
			StorageLineTypeDef::Simple(_) => quote!(
				#scrate::traits::StorageInfo {
					prefix: <
						#storage_struct as #scrate::#storage_generator_trait
					>::storage_value_final_key(),
					max_values: #max_values,
				},
			),
			StorageLineTypeDef::Map(_)| StorageLineTypeDef::DoubleMap(_) => quote!(
				#scrate::traits::StorageInfo {
					prefix: <
						#storage_struct as #scrate::StoragePrefixedMap<#value_type>
					>::final_prefix(),
					max_values: #max_values,
				},
			),
		};
		entries.extend(entry);
	}

	let module_struct = &def.module_struct;
	let module_impl = &def.module_impl;
	let where_clause = &def.where_clause;

	quote!(
		impl#module_impl #scrate::traits::StoragesInfo for #module_struct #where_clause {
			fn storages_info() -> #scrate::sp_std::vec::Vec<#scrate::traits::StorageInfo> {
				#scrate::sp_std::vec![
					#entries
				]
			}
		}
	)
}
