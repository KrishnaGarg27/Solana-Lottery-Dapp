# Project Description

**Deployed Frontend URL:** https://solana-lottery-dapp-seven.vercel.app/

**Solana Program ID:** GCoAnH3cGsUAz9vFChaujX9LpBien56N7PTaDuvgHex7

## Project Overview

### Description

A simple decentralized lottery application built on Solana. Anyone can connect their wallet, create a new lottery or enter into lotteries that have already been created. The winner of the lottery can then claim their prize.

### Key Features

- Feature 1: Anyone can create a lottery.
- Feature 2: Anyone can participate.
- Feature 2: The creator can pick a random winner.
- Feature 2: Winners can claim their prizes

### How to Use the dApp

1. **Connect Wallet**
2. **Create Lottery:** Fill the lottery creation form and create a new lottery. The inputs are checked for validity and appropriate errors are displayed otherwise. Lottery IDs should be unique for a wallet and between 0 and 255, entry price and max entries cannot be 0.
3. **Enter a lottery:** Click the button to Enter a lottery. The lottery should be running and the total number of entries should not be equal to the maximum entries.
4. **Pick a winner:** To pick winner of your lottery, click on "Pick Winner" button. The lottery should have atleast one entry to pick a winner
5. **Claim Prize:** If you have won a lottery, you can click on "Claim Prize" button to get the prize transferred to your wallet.

## Program Architecture

The Lottery Dapp has a simple structure with three types of accounts - the lottery account, the entry account and the prize vault account(which does not store any data, it only stores the prize amount). The lottery account stores all the details of the lottery and the winning entry. The entry account stores the details of the user account that created the entry.
There are four instructions - to initialize a lottery, to enter into a lottery, to pick winner of the lottery and to claim prize of the lottery.

### PDA Usage

The project uses PDAs for all the three accounts - lottery account, entry account and prize vault account. The lottery account seeds use the creator's pubkey and the lottery id. The prize vault uses the lottery PDA as a seed. The entry account uses the lottery PDA and the entry number as seeds.

**PDAs Used:**

- **Lottery Account:** User creates lottery with unique index and their public key
- **Prize Vault Account:** Automatically created along with the lottery account, using the lottery account as seed
- **Entry Account:** User created entry

### Program Instructions

**Instructions Implemented:**

- **Initialize**: Creates a new lottery
- **Enter Lottery**: Creates a new entry in a lottery
- **Pick Winner**: Picks winner of a lottery
- **Claim Prize**: Transfers prize from prize vault to user account if they have won the lottery

### Account Structure

```rust
#[account]
#[derive(InitSpace)]
pub struct Lottery {            // Lottery account
    pub creator: Pubkey,        // Creator of Lottery
    pub prize_vault: Pubkey,    // Prize vault of Lottery
    pub lottery_id: u8,         // Unique id of Lottery (0 - 255)
    pub prize_pot: u64,         // Prize amount (changes based on new entries)
    pub entry_price: u64,       // Amount required to enter the lottery
    pub max_entries: u64,       // Max number of entries
    pub total_entries: u64,     // Total number of entries till now
    pub winner: Option<Pubkey>, // Winner of the lottery (if selected) or None
    pub claimed: bool,          // Whether prize has been claimed by the winner
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Entry {
    pub owner: Pubkey,          // The user who made this entry
    pub lottery: Pubkey,        // The lottery to which this entry belongs
    pub entry_id: u64,          // Unique number of the entry
    pub bump: u8,
}

```

## Testing

### Test Coverage

**Happy Path Tests:**

- **Initialize**: Successfully creates a new lottery account with correct values
- **Enter Lottery**: Successfully creates a new entry in a lottery
- **Pick Winner**: Successfully picks winner of a lottery
- **Claim Prize**: Successfully claims prize of a lottery by the winner

**Unhappy Path Tests:**

- **Initialize**: Fails to create new lottery if entry price is 0
- **Initialize**: Fails to create new lottery if max entries is 0
- **Enter Lottery**: Fails to enter a lottery if max entries has been reached
- **Pick Winner**: Fails to pick winner of a lottery when winner has already been picked once
- **Claim Prize**: Fails to claim prize of a lottery more than once
- **Claim Prize**: Fails to claim prize of a lottery by someone other than the winner

### Running Tests

```bash
# Commands to run your tests
npm install
anchor test
```

### Additional Notes for Evaluators

This is a simple implementation of the Lottery concept on blockchain. Hence, I have not implemented Switchboard or Oracle VRF and have gone with simple randomness implementation. This is definitely not good for production, but to keep this project simpler, I have chosen this.
I have tried to account for all possible situations and errors in the smart contract.
The frontend was a struggle and so I have kept it simple with main functions.
