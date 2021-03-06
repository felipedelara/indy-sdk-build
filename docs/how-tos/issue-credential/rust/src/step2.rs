indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

println!("1. Creating a new local pool ledger configuration that can be used later to connect pool nodes");
let pool_config_file = create_genesis_txn_file_for_pool(pool_name);
let pool_config = json!({
        "genesis_txn" : &pool_config_file
    });
Pool::create_ledger_config(&pool_name, Some(&pool_config.to_string())).unwrap();

println!("2. Open pool ledger and get the pool handle from libindy");
let pool_handle: i32 = Pool::open_ledger(&pool_name, None).unwrap();

println!("3. Creates a new wallet");
let config = json!({ "id" : wallet_name.to_string() }).to_string();
Wallet::create(&config, USEFUL_CREDENTIALS).unwrap();

println!("4. Open wallet and get the wallet handle from libindy");
let wallet_handle: i32 = Wallet::open(&config, USEFUL_CREDENTIALS).unwrap();

println!("5. Generating and storing steward DID and Verkey");
let first_json_seed = json!({
"seed":"000000000000000000000000Steward1"
}).to_string();
let (steward_did, _steward_verkey) = Did::new(wallet_handle, &first_json_seed).unwrap();

println!("6. Generating and storing Trust Anchor DID and Verkey");
let (trustee_did, trustee_verkey) = Did::new(wallet_handle, &"{}".to_string()).unwrap();

println!("7. Build NYM request to add Trust Anchor to the ledger");
let build_nym_request: String = Ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).unwrap();

println!("8. Sending the nym request to ledger");
let _build_nym_sign_submit_result: String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).unwrap();

println!("9. Create Schema and Build the SCHEMA request to add new schema to the ledger as a Steward");
let name = "gvt";
let version = "1.0";
let attributes = r#"["age", "sex", "height", "name"]"#;
let (_schema_id, schema_json) = Issuer::create_schema(&steward_did, name, version, attributes).unwrap();

let build_schema_request: String = Ledger::build_schema_request(&steward_did, &schema_json).unwrap();

println!("10. Sending the SCHEMA request to the ledger");
let _signed_schema_request_response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_schema_request).unwrap();

println!("11. Creating and storing CREDENTIAL DEFINITION using anoncreds as Trust Anchor, for the given Schema");
let config_json = r#"{ "support_revocation": false }"#;
let tag = r#"TAG1"#;

let (cred_def_id, cred_def_json) = Issuer::create_and_store_credential_def(wallet_handle, &trustee_did, &schema_json, tag, None, config_json).unwrap();
