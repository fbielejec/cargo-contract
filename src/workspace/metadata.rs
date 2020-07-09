// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of cargo-contract.
//
// cargo-contract is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cargo-contract is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cargo-contract.  If not, see <http://www.gnu.org/licenses/>.

use anyhow::Result;
use std::{fs, path::Path};
use toml::value;

/// Generates a cargo workspace package which will be invoked to generate contract metadata.
///
/// # Note
///
/// `ink!` dependencies are copied from the containing contract workspace to ensure the same
/// versions are utilized.
pub(super) fn generate_package<P: AsRef<Path>>(
    target_dir: P,
    contract_package_name: &str,
    ink_lang_dependency: value::Table,
    mut ink_abi_dependency: value::Table,
) -> Result<()> {
    let dir = target_dir.as_ref();
    log::debug!(
        "Generating metadata package for {} in {}",
        contract_package_name,
        dir.display()
    );

    let cargo_toml = include_str!("../../templates/tools/generate-metadata/_Cargo.toml");
    let main_rs = include_str!("../../templates/tools/generate-metadata/main.rs");

    let mut cargo_toml: value::Table = toml::from_str(cargo_toml)?;
    let deps = cargo_toml
        .get_mut("dependencies")
        .expect("[dependencies] section specified in the template")
        .as_table_mut()
        .expect("[dependencies] is a table specified in the template");

    // initialize contract dependency
    let contract = deps
        .get_mut("contract")
        .expect("contract dependency specified in the template")
        .as_table_mut()
        .expect("contract dependency is a table specified in the template");
    contract.insert("package".into(), contract_package_name.into());

    // make ink_abi dependency use default features
    ink_abi_dependency.remove("default-features");
    ink_abi_dependency.remove("features");
    ink_abi_dependency.remove("optional");

    // add ink dependencies copied from contract manifest
    deps.insert("ink_lang".into(), ink_lang_dependency.into());
    deps.insert("ink_abi".into(), ink_abi_dependency.into());
    let cargo_toml = toml::to_string(&cargo_toml)?;

    fs::write(dir.join("Cargo.toml"), cargo_toml)?;
    fs::write(dir.join("main.rs"), main_rs)?;
    Ok(())
}