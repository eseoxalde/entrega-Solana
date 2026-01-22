use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Config {
    pub owner: Pubkey,
    pub total_proposals: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub votes: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Vote {
    pub voter: Pubkey,
}

pub enum Instruction {
    Initialize,
    CreateProposal { title: String },
    Vote { proposal_id: u32 },
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input[0] {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::CreateProposal {
                title: String::try_from_slice(&input[1..])?,
            }),
            2 => Ok(Self::Vote {
                proposal_id: u32::from_le_bytes(input[1..5].try_into().unwrap()),
            }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(data)?;
    match instruction {
        Instruction::Initialize => initialize(accounts),
        Instruction::CreateProposal { title } => create_proposal(accounts, title),
        Instruction::Vote { proposal_id } => vote(accounts, proposal_id),
    }
}

fn initialize(accounts: &[AccountInfo]) -> ProgramResult {
    let acc = &mut accounts.iter();
    let config = next_account_info(acc)?;
    let owner = next_account_info(acc)?;

    let mut cfg = Config {
        owner: *owner.key,
        total_proposals: 0,
    };

    cfg.serialize(&mut &mut config.data.borrow_mut()[..])?;
    Ok(())
}

fn create_proposal(accounts: &[AccountInfo], title: String) -> ProgramResult {
    let acc = &mut accounts.iter();
    let config = next_account_info(acc)?;
    let owner = next_account_info(acc)?;
    let proposal_acc = next_account_info(acc)?;

    let cfg = Config::try_from_slice(&config.data.borrow())?;
    if cfg.owner != *owner.key {
        return Err(ProgramError::IllegalOwner);
    }

    let proposal = Proposal {
        id: cfg.total_proposals,
        title,
        votes: 0,
    };

    proposal.serialize(&mut &mut proposal_acc.data.borrow_mut()[..])?;
    Ok(())
}

fn vote(accounts: &[AccountInfo], _proposal_id: u32) -> ProgramResult {
    let acc = &mut accounts.iter();
    let proposal_acc = next_account_info(acc)?;
    let vote_acc = next_account_info(acc)?;
    let voter = next_account_info(acc)?;

    let mut proposal = Proposal::try_from_slice(&proposal_acc.data.borrow())?;
    proposal.votes += 1;
    proposal.serialize(&mut &mut proposal_acc.data.borrow_mut()[..])?;

    let vote = Vote { voter: *voter.key };
    vote.serialize(&mut &mut vote_acc.data.borrow_mut()[..])?;

    Ok(())
}
