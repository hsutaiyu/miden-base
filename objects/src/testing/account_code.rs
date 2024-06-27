use assembly::{ast::ModuleAst, Assembler};

use crate::accounts::AccountCode;

// The MAST root of the default account's interface. Use these constants to interact with the
// account's procedures.
const MASTS: [&str; 11] = [
    "0xc4fd3df6cc5f643b0e47b392b847486858a8c8609f459c185aa591974799b32b",
    "0x6781e03de3f7b0607e53ceb9bd1ab6192ea4b5e97c094cbf5876a70a63804435",
    "0xc2684bbd1c4aaefb825ee4e3a8144763d5c4dd25a697d93ab7b1d601660d0d0e",
    "0xee383c4c6dbd69ca63cf0e49145b9eeda1e8d9af97355975647c19600d4d1837",
    "0x984f8813133166ab411c8443dc0589d56a84ca837737c3459222e85f25ec701a",
    "0x7191b3e62d06c9ba597d660099b34cb7e26780adba4ddd277ab1f507bd0b7dfb",
    "0xd65d87033e43a4ce0805fbd02945e5500554ba5830644e48fd13da97417c278c",
    "0x63bac36fa0332a5f9838c61eda6a2c06e72d653b30396b1e3f67b6e66e760ad2",
    "0x599d7a6e28c35585860e301e2d8a241cad6da8214dbf8b7e896ce95459308591",
    "0xff06b90f849c4b262cbfbea67042c4ea017ea0e9c558848a951d44b23370bec5",
    "0x8ef0092134469a1330e3c468f57c7f085ce611645d09cc7516c786fefc71d794",
];
pub const ACCOUNT_SEND_ASSET_MAST_ROOT: &str = MASTS[1];
pub const ACCOUNT_INCR_NONCE_MAST_ROOT: &str = MASTS[2];
pub const ACCOUNT_SET_ITEM_MAST_ROOT: &str = MASTS[3];
pub const ACCOUNT_SET_MAP_ITEM_MAST_ROOT: &str = MASTS[4];
pub const ACCOUNT_SET_CODE_MAST_ROOT: &str = MASTS[5];
pub const ACCOUNT_CREATE_NOTE_MAST_ROOT: &str = MASTS[6];
pub const ACCOUNT_ADD_ASSET_TO_NOTE_MAST_ROOT: &str = MASTS[7];
pub const ACCOUNT_REMOVE_ASSET_MAST_ROOT: &str = MASTS[8];
pub const ACCOUNT_ACCOUNT_PROCEDURE_1_MAST_ROOT: &str = MASTS[9];
pub const ACCOUNT_ACCOUNT_PROCEDURE_2_MAST_ROOT: &str = MASTS[10];

// ACCOUNT ASSEMBLY CODE
// ================================================================================================

pub const DEFAULT_ACCOUNT_CODE: &str = "
    use.miden::contracts::wallets::basic->basic_wallet
    use.miden::contracts::auth::basic->basic_eoa

    export.basic_wallet::receive_asset
    export.basic_wallet::send_asset
    export.basic_eoa::auth_tx_rpo_falcon512
";

pub const DEFAULT_AUTH_SCRIPT: &str = "
    use.miden::contracts::auth::basic->auth_tx

    begin
        call.auth_tx::auth_tx_rpo_falcon512
    end
";

pub fn mock_account_code(assembler: &Assembler) -> AccountCode {
    let account_code = "\
            use.miden::account
            use.miden::tx
            use.miden::contracts::wallets::basic->wallet

            # acct proc 0
            export.wallet::receive_asset
            # acct proc 1
            export.wallet::send_asset

            # acct proc 2
            export.incr_nonce
                push.0 swap
                # => [value, 0]

                exec.account::incr_nonce
                # => [0]
            end

            # acct proc 3
            export.set_item
                exec.account::set_item
                # => [R', V, 0, 0, 0]

                movup.8 drop movup.8 drop movup.8 drop
                # => [R', V]
            end

            # acct proc 4
            export.set_map_item
                exec.account::set_map_item
                # => [R', V, 0, 0, 0]

                movup.8 drop movup.8 drop movup.8 drop
                # => [R', V]
            end

            # acct proc 5
            export.set_code
                padw swapw
                # => [CODE_ROOT, 0, 0, 0, 0]

                exec.account::set_code
                # => [0, 0, 0, 0]
            end

            # acct proc 6
            export.create_note
                exec.tx::create_note
                # => [note_idx]

                swapw dropw swap drop
            end

            # acct proc 7
            export.add_asset_to_note
                exec.tx::add_asset_to_note
                # => [note_idx]

                swap drop swap drop swap drop
            end

            # acct proc 8
            export.remove_asset
                exec.account::remove_asset
                # => [ASSET]
            end

            # acct proc 9
            export.account_procedure_1
                push.1.2
                add
            end

            # acct proc 10
            export.account_procedure_2
                push.2.1
                sub
            end
            ";
    let account_module_ast = ModuleAst::parse(account_code).unwrap();
    let code = AccountCode::new(account_module_ast, assembler).unwrap();

    // Ensures the mast root constants match the latest version of the code.
    //
    // The constants will change if the library code changes, and need to be updated so that the
    // tests will work properly. If these asserts fail, copy the value of the code (the left
    // value), into the constants.
    //
    // Comparing all the values together, in case multiple of them change, a single test run will
    // detect it.
    let current = [
        code.procedures()[0].0.to_hex(),
        code.procedures()[1].0.to_hex(),
        code.procedures()[2].0.to_hex(),
        code.procedures()[3].0.to_hex(),
        code.procedures()[4].0.to_hex(),
        code.procedures()[5].0.to_hex(),
        code.procedures()[6].0.to_hex(),
        code.procedures()[7].0.to_hex(),
        code.procedures()[8].0.to_hex(),
        code.procedures()[9].0.to_hex(),
        code.procedures()[10].0.to_hex(),
    ];
    assert!(current == MASTS, "const MASTS: [&str; 11] = {:#?};", current);

    code
}

pub const CODE: &str = "
        export.foo
            push.1 push.2 mul
        end

        export.bar
            push.1 push.2 add
        end
    ";

pub fn make_account_code() -> AccountCode {
    let mut module = ModuleAst::parse(CODE).unwrap();
    // clears are needed since they're not serialized for account code
    module.clear_imports();
    module.clear_locations();
    AccountCode::new(module, &Assembler::default()).unwrap()
}
