//! Provides structures and traits for decoding and processing instructions
//! within transactions.
//!
//! The module includes the following main components:
//! - **`InstructionMetadata`**: Metadata associated with an instruction,
//!   capturing transaction context.
//! - **`DecodedInstruction`**: Represents an instruction that has been decoded,
//!   with associated program ID, data, and accounts.
//! - **`InstructionDecoder`**: A trait for decoding instructions into specific
//!   types.
//! - **`InstructionPipe`**: A structure that processes instructions using a
//!   decoder and a processor.
//! - **`InstructionPipes`**: An async trait for processing instructions within
//!   nested contexts.
//! - **`NestedInstruction`**: Represents instructions with potential nested
//!   inner instructions, allowing for recursive processing.
//!
//! These components enable the `carbon-core` framework to handle Solana
//! transaction instructions efficiently, decoding them into structured types
//! and facilitating hierarchical processing.

use {
    serde::Deserialize,
    solana_sdk::{instruction::AccountMeta, pubkey::Pubkey},
};


/// A decoded instruction containing program ID, data, and associated accounts.
///
/// The `DecodedInstruction` struct represents the outcome of decoding a raw
/// instruction, encapsulating its program ID, parsed data, and the accounts
/// involved.
///
/// # Type Parameters
///
/// - `T`: The type representing the decoded data for the instruction.
///
/// # Fields
///
/// - `program_id`: The program ID that owns the instruction.
/// - `data`: The decoded data payload for the instruction, of type `T`.
/// - `accounts`: A vector of `AccountMeta`, representing the accounts involved
///   in the instruction.

#[derive(Debug, Clone, Deserialize)]
pub struct DecodedInstruction<T> {
    pub program_id: Pubkey,
    pub data: T,
    pub accounts: Vec<AccountMeta>,
}



// /// Represents a nested instruction with metadata, including potential inner
// /// instructions.
// ///
// /// The `NestedInstruction` struct allows for recursive instruction handling,
// /// where each instruction may have associated metadata and a list of nested
// /// instructions.
// ///
// /// # Fields
// ///
// /// - `metadata`: The metadata associated with the instruction.
// /// - `instruction`: The Solana instruction being processed.
// /// - `inner_instructions`: A vector of `NestedInstruction`, representing any
// ///   nested instructions.
// #[derive(Debug, Clone)]
// pub struct NestedInstruction {
//     pub instruction: solana_sdk::instruction::Instruction,
//     pub inner_instructions: Vec<NestedInstruction>,
// }

// #[derive(Debug)]
// pub struct NestedInstructions(pub Vec<NestedInstruction>);

// impl NestedInstructions {
//     pub fn iter(&self) -> std::slice::Iter<NestedInstruction> {
//         self.0.iter()
//     }
// }

// impl Deref for NestedInstructions {
//     type Target = [NestedInstruction];

//     fn deref(&self) -> &[NestedInstruction] {
//         &self.0[..]
//     }
// }

// /// Nests instructions based on stack height, producing a hierarchy of
// /// `NestedInstruction`.
// ///
// /// This function organizes instructions into a nested structure, enabling
// /// hierarchical transaction analysis. Instructions are nested according to
// /// their stack height, forming a tree-like structure.
// ///
// /// # Parameters
// ///
// /// - `instructions`: A list of tuples containing `InstructionMetadata` and
// ///   instructions.
// ///
// /// # Returns
// ///
// /// A vector of `NestedInstruction`, representing the instructions organized by
// /// stack depth.
// impl From<Vec<Instruction>> for NestedInstructions {
//     fn from(instructions: Vec<Instruction>) -> Self {
//         log::trace!("from(instructions: {:?})", instructions);
//         let mut result = Vec::<NestedInstruction>::new();
//         let mut stack = Vec::<(Vec<usize>, usize)>::new();

//         for (instruction) in instructions {
//             let nested_instruction = NestedInstruction {
//                 instruction,
//                 inner_instructions: Vec::new(),
//             };

//             while let Some((_, parent_stack_height)) = stack.last() {
//                 if metadata.stack_height as usize > *parent_stack_height {
//                     break;
//                 }
//                 stack.pop();
//             }

//             if let Some((path_to_parent, _)) = stack.last() {
//                 let mut current_instructions = &mut result;
//                 for &index in path_to_parent {
//                     current_instructions = &mut current_instructions[index].inner_instructions;
//                 }
//                 current_instructions.push(nested_instruction);
//                 let mut new_path = path_to_parent.clone();
//                 new_path.push(current_instructions.len() - 1);
//                 stack.push((new_path, metadata.stack_height as usize));
//             } else {
//                 result.push(nested_instruction);
//                 let new_path = vec![result.len() - 1];
//                 stack.push((new_path, metadata.stack_height as usize));
//             }
//         }

//         NestedInstructions(result)
//     }
// }
