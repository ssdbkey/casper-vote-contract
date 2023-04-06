#[cfg(test)]
mod tests {
    // Outlining aspects of the Casper test support crate to include.
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        DEFAULT_RUN_GENESIS_REQUEST,
    };
    // Custom Casper types that will be used within this test.
    use casper_types::{runtime_args, ContractHash, RuntimeArgs};

    const VOTE_WASM: &str = "vote.wasm"; // The first version of the contract
    const CONTRACT_KEY: &str = "vote"; // Named key referencing this contract

    #[test]
    /// Install vote contract and check its available entry points.
    /// Test summary:
    /// - Install the vote.wasm contract.
    /// - Check the contract hash.
    /// - Register a project.
    /// - Verify that the total vote count for that project is 0.
    /// - Vote to that project.
    /// - Verify that the total vote count for that project is now 1.
    /// - Register the same project, which should fail.
    /// - Vote for a non-existent project, which should fail.
    fn install_and_check_entry_points() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        // Install the contract.
        let contract_installation_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, VOTE_WASM, runtime_args! {})
                .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        // Check the contract hash.
        let contract_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Call the register_project entry point.
        let register_project_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            "register_project",
            runtime_args! {
                "project_id" => "test_project_id"
            },
        )
        .build();

        // Try executing the register_project entry point.
        builder
            .exec(register_project_request)
            .expect_success()
            .commit();

        // Verify that the total vote count for that project is 0.
        let contract = builder
            .get_contract(contract_hash)
            .expect("this contract should exist");

        let project_dictionary_key = *contract
            .named_keys()
            .get("project_dictionary")
            .expect("project_dictionary uref should exist in the contract keys");

        let vote_count = builder
            .query_dictionary_item(
                None,
                project_dictionary_key.into_uref().unwrap(),
                "test_project_id",
            )
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<Option<i32>>()
            .expect("should be Option<i32>.")
            .unwrap();

        assert_eq!(vote_count, 0);

        // Call the vote entry point.
        let vote_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            "vote",
            runtime_args! {
                "project_id" => "test_project_id"
            },
        )
        .build();

        // Try executing the vote entry point.
        builder.exec(vote_request).expect_success().commit();

        // Verify that the total vote count for that project is now 1.
        let new_vote_count = builder
            .query_dictionary_item(
                None,
                project_dictionary_key.into_uref().unwrap(),
                "test_project_id",
            )
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<Option<i32>>()
            .expect("should be Option<i32>.")
            .unwrap();

        assert_eq!(new_vote_count, 1);

        // Register the same project and expect an error.
        let register_project_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            "register_project",
            runtime_args! {
                "project_id" => "test_project_id"
            },
        )
        .build();

        builder
            .exec(register_project_request)
            .expect_failure()
            .commit();

        // Vote for a non-existent project and expect an error.
        let vote_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_hash,
            "vote",
            runtime_args! {
                "project_id" => "another_test_project_id"
            },
        )
        .build();

        // Try executing the vote entry point.
        builder.exec(vote_request).expect_failure().commit();
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
