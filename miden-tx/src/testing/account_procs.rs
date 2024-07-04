use std::println;

use alloc::collections::BTreeMap;

use miden_lib::transaction::TransactionKernelError;
use vm_processor::{AdviceProvider, Digest, Felt, ProcessState};

// ACCOUNT PROCEDURE INDEX MAP
// ================================================================================================

/// A map of proc_root |-> proc_index for all known procedures of an account interface.
pub struct AccountProcedureIndexMap(BTreeMap<Digest, u8>);

// Same as in host/account_procs.rs
impl AccountProcedureIndexMap {
    /// Returns a new [AccountProcedureIndexMap] instantiated with account procedures present in
    /// the provided advice provider.
    ///
    /// This function assumes that the account procedure tree (or a part thereof) is loaded into the
    /// Merkle store of the provided advice provider.
    pub fn new<A: AdviceProvider>(account_code_root: Digest, adv_provider: &A) -> Self {
        // get the account procedures from the advice_map
        let procs = adv_provider.get_mapped_values(&account_code_root).unwrap();

        // iterate over all possible procedure indexes
        let mut result = BTreeMap::new();

        println!("Procs: {:?}", procs);
        for (proc_idx, proc_info) in procs[1..].chunks_exact(8).enumerate() {
            let root: [Felt; 4] = proc_info[0..4].try_into().expect("Slice with incorrect len.");
            result.insert(Digest::from(root), proc_idx.try_into().unwrap());
            println!(
                "Index map inserted root: {}, elements: {:?}",
                Digest::from(root).to_hex(),
                root
            );
        }

        Self(result)
    }

    /// Returns index of the procedure whose root is currently at the top of the operand stack in
    /// the provided process.
    ///
    /// # Errors
    /// Returns an error if the procedure at the top of the operand stack is not present in this
    /// map.
    pub fn get_proc_index<S: ProcessState>(
        &self,
        process: &S,
    ) -> Result<u8, TransactionKernelError> {
        let proc_root = process.get_stack_word(0).into();
        // mock account method for testing from root context
        // TODO: figure out if we can get rid of this
        if proc_root == Digest::default() {
            return Ok(255);
        }
        self.0
            .get(&proc_root)
            .cloned()
            .ok_or(TransactionKernelError::UnknownAccountProcedure(proc_root))
    }
}
