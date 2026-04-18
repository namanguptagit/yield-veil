// mod state;
//
// use anchor_lang::prelude::*;
// use arcium_anchor::prelude::*;
//
// const COMP_DEF_OFFSET_ADD_TOGETHER: u32 = comp_def_offset("add_together");
//
// declare_id!("CJAexGmZWBZSaMQdrFjC3rCWSyQre1J41P668WW1Tr32");
//
// #[arcium_program]
// pub mod onchain_arcium_struct {
//     use super::*;
//
//     pub fn init_add_together_comp_def(ctx: Context<InitAddTogetherCompDef>) -> Result<()> {
//         init_comp_def(ctx.accounts, None, None)?;
//         Ok(())
//     }
//
//     pub fn add_together(
//         ctx: Context<AddTogether>,
//         computation_offset: u64,
//         ciphertext_0: [u8; 32],
//         ciphertext_1: [u8; 32],
//         pubkey: [u8; 32],
//         nonce: u128,
//     ) -> Result<()> {
//         ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;
//         let args = ArgBuilder::new()
//             .x25519_pubkey(pubkey)
//             .plaintext_u128(nonce)
//             .encrypted_u8(ciphertext_0)
//             .encrypted_u8(ciphertext_1)
//             .build();
//
//         queue_computation(
//             ctx.accounts,
//             computation_offset,
//             args,
//             vec![AddTogetherCallback::callback_ix(
//                 computation_offset,
//                 &ctx.accounts.mxe_account,
//                 &[]
//             )?],
//             1,
//             0,
//         )?;
//         Ok(())
//     }
//
//     #[arcium_callback(encrypted_ix = "add_together")]
//     pub fn add_together_callback(
//         ctx: Context<AddTogetherCallback>,
//         output: SignedComputationOutputs<AddTogetherOutput>,
//     ) -> Result<()> {
//         let o = match output.verify_output(&ctx.accounts.cluster_account, &ctx.accounts.computation_account) {
//             Ok(AddTogetherOutput { field_0 }) => field_0,
//             Err(_) => return Err(ErrorCode::AbortedComputation.into()),
//         };
//
//         emit!(SumEvent {
//             sum: o.ciphertexts[0],
//             nonce: o.nonce.to_le_bytes(),
//         });
//         Ok(())
//     }
// }
//
