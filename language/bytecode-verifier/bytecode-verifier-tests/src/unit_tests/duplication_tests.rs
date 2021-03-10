// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use bytecode_verifier::DuplicationChecker;
use proptest::prelude::*;
use vm::file_format::*;

#[test]
fn duplicated_friend_decls() {
    let mut m = basic_test_module();
    let handle = ModuleHandle {
        address: AddressIdentifierIndex::new(0),
        name: IdentifierIndex::new(0),
    };
    m.friend_decls.push(handle.clone());
    m.friend_decls.push(handle);
    DuplicationChecker::verify_module(&m.freeze().unwrap()).unwrap_err();
}

proptest! {
    #[test]
    fn valid_duplication(module in CompiledModule::valid_strategy(20)) {
        prop_assert!(DuplicationChecker::verify_module(&module).is_ok());
    }
}
