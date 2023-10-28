#[cfg(test)]
mod test_module {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{/* coin, coins,*/ from_binary, Coin, Deps, DepsMut, Empty, Addr};
    use cw4_group::msg::{LookUpResponse, MemberNamed};
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
//    use crate::state::Config;
    use cw_multi_test::{
        App, Contract, ContractWrapper, Executor,
    };

/* 
a lot to add in here


*/

    pub fn contract_cw69() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }
    pub fn contract_cw4_group_named() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw4_group::contract::execute,
            cw4_group::contract::instantiate,
            cw4_group::contract::query,
        );
        Box::new(contract)
    }

    fn mock_app() -> App {
        App::default()//new(api, env.block, bank, Box::new(MockStorage::new()))
    }
    
    fn assert_name_owner(deps: Deps, name: &str, owner: &str) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::LookUp {
                name: name.to_string(),
            },
        )
        .unwrap();

        let value: LookUpResponse = from_binary(&res).unwrap();
        assert_eq!(Some(owner.to_string()), value.addr);
    }
/* 
    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Price {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value, expected);
    }
 */
/* 
 fn mock_init_with_price(deps: DepsMut, purchase_price: Coin) {
        let msg = InstantiateMsg {
            price: Some(purchase_price),
            admin: None,
            owner_can_update: true,
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }
 */
    fn mock_init_no_price(deps: DepsMut) {
        let msg = InstantiateMsg {
            price: None,
            admin: None,
            owner_can_update: true,
        };

        let info = mock_info("creator", &[]);
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }

    fn mock_alice_registers_name(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "alice".to_string(),
        };
        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract successfully handles Register message");
    }
    
    #[test]
    fn register_indexed_second_layer() {
        let mut router = mock_app();
        let alice = Addr::unchecked("alice_address");

        let indexer_contract_code_id = router.store_code(contract_cw69());
        let init_msg = InstantiateMsg {
            price: None,            
            admin: None, 
            owner_can_update: true,
        };
        let root_indexer = router.instantiate_contract(
            indexer_contract_code_id, 
            alice.clone(), 
            &init_msg, 
            &[], 
            "root_indexer", 
            Some(alice.clone().to_string())
        ).unwrap();

        let branch_indexer = router.instantiate_contract(
            indexer_contract_code_id, 
            alice.clone(), 
            &init_msg, 
            &[], 
            "branch_indexer", 
            Some(alice.clone().to_string())
        ).unwrap();

        // index branch in root and alice in branch        
        let index_msg_root = ExecuteMsg::Register { 
            address: branch_indexer.to_string(), 
            name: "guildhub".to_string(), 
        };
        let _ = router.execute_contract(
            alice.clone(), 
            root_indexer.clone(), 
            &index_msg_root, 
            &[]
        ).unwrap();
        let index_msg_branch = ExecuteMsg::Register { 
            address: alice.clone().to_string(), 
            name: "alice".to_string() 
        };
        let _ = router.execute_contract(
            alice.clone(),
            branch_indexer,
            &index_msg_branch,
            &[]
        ).unwrap();
        
        // check second layer names (and inter-contract queries)
        let query_2nd_layer_msg = QueryMsg::LookUp { name: "alice.guildhub".to_string() };                
        let queried_alice: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer, 
            &query_2nd_layer_msg
        ).unwrap();
        assert_eq!(queried_alice.addr.unwrap(), alice.to_string())
    }
    
    #[test]
    fn register_group_named() {        
        let mut router = mock_app();
        let alice = Addr::unchecked("alice_address");
        let bob = Addr::unchecked("bob_address");
        let carol = Addr::unchecked("carol_address");
        // launch group-named
        let group_contract_code_id = router.store_code(contract_cw4_group_named());
        let init_group_msg = cw4_group::msg::InstantiateMsg {
            admin: Some(alice.to_string()),
            members: vec![
                MemberNamed {
                    addr: alice.clone().into(),
                    name: "alice".to_string(),
                    weight: 1 
                },
                MemberNamed {
                    addr: bob.clone().into(),
                    name: "bob".to_string(),
                    weight: 1 
                },
                MemberNamed {
                    addr: carol.clone().into(),
                    name: "carol".to_string(),
                    weight: 1 
                }
            ]
        };
        let group_contract = router.instantiate_contract(
            group_contract_code_id, 
            alice.clone(), 
            &init_group_msg, 
            &[], 
            "guildhub", 
            Some(alice.to_string())
        ).unwrap();

        // launch indexer
        let indexer_contract_code_id = router.store_code(contract_cw69());
        let init_msg = InstantiateMsg {
            price: None,
            admin: None,
            owner_can_update: true,            
        };
        let root_indexer = router.instantiate_contract(
            indexer_contract_code_id, 
            alice.clone(), 
            &init_msg, 
            &[], 
            "root_indexer", 
            Some(alice.clone().to_string())
        ).unwrap();

        let index_msg_root = ExecuteMsg::Register { 
            address: group_contract.to_string(), 
            name: "guildhub".to_string(), 
        };
        let _ = router.execute_contract(
            alice.clone(), 
            root_indexer.clone(), 
            &index_msg_root, 
            &[]
        ).unwrap();
        
        // check second layer names (and inter-contract queries)
        let query_alice = QueryMsg::LookUp { name: "alice.guildhub".to_string() };                
        let query_bob = QueryMsg::LookUp { name: "bob.guildhub".to_string() };                
        let query_carol = QueryMsg::LookUp { name: "carol.guildhub".to_string() };                
        let query_unknown = QueryMsg::LookUp { name: "unknown.guildhub".to_string() };
        let queried_alice: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_alice
        ).unwrap();
        assert_eq!(queried_alice.addr.unwrap(), alice.to_string());

        let queried_bob: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_bob
        ).unwrap();
        assert_eq!(queried_bob.addr.unwrap(), bob.clone().to_string());

        let queried_carol: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_carol
        ).unwrap();
        assert_eq!(queried_carol.addr.unwrap(), carol.clone().to_string());

        let queried_unknown: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_unknown
        ).unwrap();
        assert!(queried_unknown.addr.is_none());
    } 

    #[test]
    fn register_indexed_more_layers() {
        // (essentially a map of mappings where each contract is an indexer)
        // make any number of indexed indexers and read through all of them << this
        
        let mut router = mock_app();
        let alice = Addr::unchecked("alice_address");
        let bob = Addr::unchecked("bob_address");
        let carol = Addr::unchecked("carol_address");
        // launch group-named
        let group_contract_code_id = router.store_code(contract_cw4_group_named());
        let init_group_msg = cw4_group::msg::InstantiateMsg {
            admin: Some(alice.to_string()),
            members: vec![
                MemberNamed {
                    addr: alice.clone().into(),
                    name: "alice".to_string(),
                    weight: 1 
                },
                MemberNamed {
                    addr: bob.clone().into(),
                    name: "bob".to_string(),
                    weight: 1 
                },
                MemberNamed {
                    addr: carol.clone().into(),
                    name: "carol".to_string(),
                    weight: 1 
                }
            ]
        };
        let group_contract = router.instantiate_contract(
            group_contract_code_id, 
            alice.clone(), 
            &init_group_msg, 
            &[], 
            "guildhub", 
            Some(alice.to_string())
        ).unwrap();

        // launch indexer
        let indexer_contract_code_id = router.store_code(contract_cw69());
        let init_msg = InstantiateMsg {
            price: None,
            admin: None,
            owner_can_update: true,
        };
        let root_indexer = router.instantiate_contract(
            indexer_contract_code_id, 
            alice.clone(), 
            &init_msg, 
            &[], 
            "root_indexer", 
            Some(alice.clone().to_string())
        ).unwrap();

        let branch_indexer = router.instantiate_contract(
            indexer_contract_code_id, 
            alice.clone(), 
            &init_msg, 
            &[], 
            "branch_indexer", 
            Some(alice.clone().to_string())
        ).unwrap();
        
        // index branch in root and group in branch        
        let index_msg_branch = ExecuteMsg::Register { 
            address: group_contract.to_string(), 
            name: "gamers_for_life".to_string(), 
        };
        let _ = router.execute_contract(
            alice.clone(), 
            branch_indexer.clone(), 
            &index_msg_branch, 
            &[]
        ).unwrap();
        let index_msg_root = ExecuteMsg::Register { 
            address: branch_indexer.to_string(), 
            name: "guildhub".to_string(), 
        };
        let _ = router.execute_contract(
            alice.clone(), 
            root_indexer.clone(), 
            &index_msg_root, 
            &[]
        );

        // check second layer names (and inter-contract queries)
        let query_alice = QueryMsg::LookUp { name: "alice.gamers_for_life.guildhub".to_string() };                
        let query_bob = QueryMsg::LookUp { name: "bob.gamers_for_life.guildhub".to_string() };                
        let query_carol = QueryMsg::LookUp { name: "carol.gamers_for_life.guildhub".to_string() };                
        let query_unknown = QueryMsg::LookUp { name: "unknown.gamers_for_life.guildhub".to_string() };
        let queried_alice: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_alice
        ).unwrap();
        assert_eq!(queried_alice.addr.unwrap(), alice.to_string());

        let queried_bob: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_bob
        ).unwrap();
        assert_eq!(queried_bob.addr.unwrap(), bob.clone().to_string());

        let queried_carol: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_carol
        ).unwrap();
        assert_eq!(queried_carol.addr.unwrap(), carol.clone().to_string());

        let queried_unknown: LookUpResponse = router.wrap().query_wasm_smart(
            root_indexer.clone(), 
            &query_unknown
        ).unwrap();
        assert!(queried_unknown.addr.is_none());
    }
    
    // others:
    // create two groups with a same user address, verify it is read as both names (ie: alice.abc & alice.xyz)

    #[test]
    fn register_available_name_and_query_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        
        mock_alice_registers_name(deps.as_mut(), &[]);
        
        // querying for name resolves to correct address
        assert_name_owner(deps.as_ref(), "alice", "test");
    }
/* 
    #[test]
    fn register_available_name_and_query_works_with_fees() {
        let mut deps = mock_dependencies();
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

        // anyone can register an available name with more fees than needed
        let info = mock_info("bob_key", &coins(5, "token"));
        let msg = ExecuteMsg::Register {
            address: "test2".into(),
            name: "bob".to_string(),
        };
        
        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Register message");
        
        // querying for name resolves to correct address
        assert_name_owner(deps.as_ref(), "alice", "test");
        assert_name_owner(deps.as_ref(), "bob", "test2");
    }
 */
/* 
    #[test]
    fn proper_init_no_fees() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: None,
            },
        );
    }
 */
/*     #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        mock_init_with_price(deps.as_mut(), coin(3, "token"), coin(4, "token"));

        assert_config_state(
            deps.as_ref(),
            Config {
                purchase_price: Some(coin(3, "token")),
            },
        );
    }
 */
/*     #[test]
fn fails_on_register_already_taken_name() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // bob can't register the same name
        let info = mock_info("bob_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }
        // alice can't register the same name again
        let info = mock_info("alice_key", &coins(2, "token"));
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "alice".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTaken { .. }) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
 */
/*     #[test]
    fn register_available_name_fails_with_invalid_name() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        let info = mock_info("bob_key", &coins(2, "token"));

        // hi is too short
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "hi".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTooShort { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }

        // 65 chars is too long
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "01234567890123456789012345678901234567890123456789012345678901234".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameTooLong { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }

        // no upper case...
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "LOUD".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info.clone(), msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::InvalidCharacter { c }) => assert_eq!(c, 'L'),
            Err(_) => panic!("Unknown error"),
        }
        // ... or spaces
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "two words".to_string(),
        };
        match execute(deps.as_mut(), mock_env(), info, msg) {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::InvalidCharacter { .. }) => {}
            Err(_) => panic!("Unknown error"),
        }
    }
 */
/*     #[test]
    fn fails_on_register_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // anyone can register an available name with sufficient fees
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn fails_on_register_wrong_fee_denom() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));

        // anyone can register an available name with sufficient fees
        let info = mock_info("alice_key", &coins(2, "earth"));
        let msg = ExecuteMsg::Register {
            address: "test".into(),
            name: "alice".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
 */
/* 
    #[test]
    fn transfer_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // alice can transfer her name successfully to bob
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Transfer message");
        // querying for name resolves to correct address (bob_key)
        assert_name_owner(deps.as_ref(), "alice", "bob_key");
    }
 */
/* 
 #[test]
    fn transfer_works_with_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

        // alice can transfer her name successfully to bob
        let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Transfer message");
        // querying for name resolves to correct address (bob_key)
        assert_name_owner(deps.as_ref(), "alice", "bob_key");
    }
 */
/* 
 #[test]
    fn fails_on_transfer_non_existent() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // alice can transfer her name successfully to bob
        let info = mock_info("frank_key", &coins(2, "token"));
        let msg = ExecuteMsg::Transfer {
            name: "alice42".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::NameNotExists { name }) => assert_eq!(name, "alice42"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        // querying for name resolves to correct address (alice_key)
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }
 */
/* 
    #[test]
    fn fails_on_transfer_from_nonowner() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_registers_name(deps.as_mut(), &[]);

        // alice can transfer her name successfully to bob
        let info = mock_info("frank_key", &coins(2, "token"));
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("Must return error"),
            Err(ContractError::Unauthorized { .. }) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        // querying for name resolves to correct address (alice_key)
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }
 */
/* 
    #[test]
    fn fails_on_transfer_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(5, "token"));
        mock_alice_registers_name(deps.as_mut(), &coins(2, "token"));

        // alice can transfer her name successfully to bob
        let info = mock_info("alice_key", &[coin(1, "earth"), coin(2, "token")]);
        let msg = ExecuteMsg::Transfer {
            name: "alice".to_string(),
            to: "bob_key".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        match res {
            Ok(_) => panic!("register call should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }

        // querying for name resolves to correct address (bob_key)
        assert_name_owner(deps.as_ref(), "alice", "alice_key");
    }
 */
/* 
    #[test]
    fn returns_empty_on_query_unregistered_name() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());

        // querying for unregistered name results in NotFound error
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ResolveRecord {
                name: "alice".to_string(),
            },
        )
        .unwrap();
        let value: ResolveRecordResponse = from_binary(&res).unwrap();
        assert_eq!(None, value.address);
    }
    */
}