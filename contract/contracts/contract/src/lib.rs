#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VoteChoice {
    Yes,
    No,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub title: String,
    pub description: String,
    pub yes_votes: i128,
    pub no_votes: i128,
    pub executed: bool,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    TotalSupply,
    ProposalCount,
    Balance(Address),
    Proposal(u32),
    Vote(u32, Address),
}

#[contract]
pub struct GovernanceTokenPlatform;

#[contractimpl]
impl GovernanceTokenPlatform {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("admin not set"));

        admin.require_auth();

        let mut current_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        current_balance += amount;

        let mut total_supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        total_supply += amount;

        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &current_balance);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &total_supply);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("amount must be positive");
        }

        from.require_auth();

        let mut from_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            panic!("insufficient balance");
        }

        let mut to_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        from_balance -= amount;
        to_balance += amount;

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &from_balance);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &to_balance);
    }

    pub fn create_proposal(
        env: Env,
        proposer: Address,
        title: String,
        description: String,
        voting_period_secs: u64,
    ) -> u32 {
        proposer.require_auth();

        if voting_period_secs == 0 {
            panic!("voting period must be greater than zero");
        }

        let mut proposal_count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0);

        proposal_count += 1;

        let proposal = Proposal {
            id: proposal_count,
            proposer: proposer.clone(),
            title,
            description,
            yes_votes: 0,
            no_votes: 0,
            executed: false,
            deadline: env.ledger().timestamp() + voting_period_secs,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_count), &proposal);
        env.storage()
            .instance()
            .set(&DataKey::ProposalCount, &proposal_count);

        proposal_count
    }

    pub fn vote(env: Env, voter: Address, proposal_id: u32, support: VoteChoice) {
        voter.require_auth();

        let vote_key = DataKey::Vote(proposal_id, voter.clone());
        if env.storage().persistent().has(&vote_key) {
            panic!("already voted");
        }

        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal not found"));

        if proposal.executed {
            panic!("proposal already executed");
        }

        if env.ledger().timestamp() > proposal.deadline {
            panic!("voting period ended");
        }

        let voting_power: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(voter.clone()))
            .unwrap_or(0);

        if voting_power <= 0 {
            panic!("no voting power");
        }

        match support {
            VoteChoice::Yes => proposal.yes_votes += voting_power,
            VoteChoice::No => proposal.no_votes += voting_power,
        }

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage().persistent().set(&vote_key, &true);
    }

    pub fn execute(env: Env, proposal_id: u32) -> bool {
        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal not found"));

        if proposal.executed {
            panic!("already executed");
        }

        if env.ledger().timestamp() <= proposal.deadline {
            panic!("voting still active");
        }

        let passed = proposal.yes_votes > proposal.no_votes;
        proposal.executed = true;

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);

        passed
    }

    pub fn balance_of(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal not found"))
    }

    pub fn proposal_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }
}