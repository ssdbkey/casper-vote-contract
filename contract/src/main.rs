#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// This code imports necessary aspects of external crates that we will use in our contract code.
extern crate alloc;

// Importing Rust types.
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
// Importing aspects of the Casper platform.
use casper_contract::{
    contract_api::{
        runtime::{self, call_contract, get_caller},
        storage::{self, dictionary_get, dictionary_put},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
// Importing specific Casper types.
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, Parameter, RuntimeArgs,
};

// Constants for the keys pointing to values stored in the account's named keys.
const CONTRACT_PACKAGE_NAME: &str = "vote_package_name";
const CONTRACT_ACCESS_UREF: &str = "vote_access_uref";
const CONTRACT_VERSION_KEY: &str = "version";
const CONTRACT_KEY: &str = "vote";

const PROJECT_DICTIONARY: &str = "project_dictionary";
const PROJECT_ID: &str = "project_id";

#[no_mangle]
pub extern "C" fn constructor() {
    storage::new_dictionary(PROJECT_DICTIONARY).unwrap_or_revert();
}

// Entry point that register a project with unique project id.
#[no_mangle]
pub extern "C" fn register_project() {
    let project_id: String = runtime::get_named_arg(PROJECT_ID);
    let dictionary_uref = match runtime::get_key(PROJECT_DICTIONARY) {
        Some(uref_key) => uref_key.into_uref().unwrap_or_revert(),
        None => runtime::revert(ApiError::ValueNotFound),
    };
    if let Some(Some(_)) =
        dictionary_get::<Option<i32>>(dictionary_uref, &project_id).unwrap_or_revert()
    {
        runtime::revert(ApiError::InvalidArgument)
    } else {
        let state: i32 = 0;
        dictionary_put(dictionary_uref, &project_id, Some(state))
    }
}

// Entry point that vote to a project.
#[no_mangle]
pub extern "C" fn vote() {
    let project_id: String = runtime::get_named_arg(PROJECT_ID);
    let voter_hash = get_caller();
    let voter = voter_hash.to_string();
    let dictionary_uref = match runtime::get_key(PROJECT_DICTIONARY) {
        Some(uref_key) => uref_key.into_uref().unwrap_or_revert(),
        None => runtime::revert(ApiError::ValueNotFound),
    };
    if let Some(Some(mut vote_count)) =
        dictionary_get::<Option<i32>>(dictionary_uref, &project_id).unwrap_or_revert()
    {
        let key = project_id.clone() + ":" + &vote_count.to_string();
        if dictionary_get::<Option<String>>(dictionary_uref, &key)
            .unwrap_or_revert()
            .is_none()
        {
            dictionary_put(dictionary_uref, &key, Some(voter));
            vote_count += 1;
            dictionary_put(dictionary_uref, &project_id, Some(vote_count))
        }
    } else {
        runtime::revert(ApiError::InvalidArgument)
    }
}

// Entry point that returns the total vote count of a project.
#[no_mangle]
pub extern "C" fn get_total_vote_count() {
    let project_id: String = runtime::get_named_arg(PROJECT_ID);
    let dictionary_uref = match runtime::get_key(PROJECT_DICTIONARY) {
        Some(uref_key) => uref_key.into_uref().unwrap_or_revert(),
        None => runtime::revert(ApiError::ValueNotFound),
    };
    if let Some(Some(vote_count)) =
        dictionary_get::<Option<i32>>(dictionary_uref, &project_id).unwrap_or_revert()
    {
        let typed_result = CLValue::from_t(vote_count).unwrap_or_revert();
        runtime::ret(typed_result); // Return the total vote count value.
    } else {
        runtime::revert(ApiError::InvalidArgument)
    }
}

// Entry point that executes automatically when a caller installs the contract.
#[no_mangle]
pub extern "C" fn call() {
    // Create the entry points for this contract.
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "register_project",
        alloc::vec![Parameter::new(PROJECT_ID, CLType::String)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "vote",
        alloc::vec![Parameter::new(PROJECT_ID, CLType::String)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_total_vote_count",
        alloc::vec![Parameter::new(PROJECT_ID, CLType::String)],
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Create a new contract package
    let (stored_contract_hash, contract_version) = storage::new_contract(
        entry_points,
        None,
        Some(CONTRACT_PACKAGE_NAME.to_string()),
        Some(CONTRACT_ACCESS_UREF.to_string()),
    );

    // Store the contract version in the context's named keys.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());

    call_contract(stored_contract_hash, "constructor", RuntimeArgs::new())
}
