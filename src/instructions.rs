use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 | 1 => {
                if rest.len() < 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let args = u32::from_le_bytes(rest[..4].try_into().unwrap());
                match variant {
                    0 => Self::Increment(args),
                    1 => Self::Decrement(args),
                    _ => unreachable!(),
                }
            },
            2 => {
                Self::Update(UpdateArgs::try_from_slice(rest).unwrap())
            },
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
