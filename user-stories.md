# User Stories

Is a long-term crypto lottery where your numbers change with each new ticket sale, giving you a new chance to win.

## User Types
Different types of users on Crypto-Ticket platform:
- Platform: The Platform creator of ticket
- User: A casual visitor on the app.

1. As a user, I want to see a list of active tickets that I can participate in. 
2. I can then filter the tickets to see either upcoming or past listings.
3. As a user, I want to see the list of tickets to display the following:
    - Entry price
    - how many tickets are left
    - A real-time countdown timer showing how much time is left for the particular ticket.

## Core Ticket Functionality

**1. Buy Ticket.** As a user, I want to buy a ticket so that I may try to win the Jackpot. I’m expected to see that:

- field of numbers is displayed
- points
- My address is recorded as the last buyer in ticket history
- My balance is decreased by the ticket price

**2. Ticket Timer.** As a user, I want to see a real-time countdown timer for each auction so that I know how much time is left to bid. I’m expecting to see that:

- Timer displays (days, hours) minutes and seconds
- Timer decrements by 1 seconds every second
- Ticket ends when timer reaches zero
- System checks that when timer hits zero

**3. Winning an Jackpot in Ticket.** As the user that collect right numbers, I want to win the ticket when the timer expires so that I can claim the NFT.

- I’m presented with a “you’re the winner” type of banner and a CLAIM button I can press to transfer the tokens to my wallet

**4. Earn money if Timer hits zero.** As the user that collecting points, I want to earn the tokens when the timer expires so that I can claim the tokens which will be issued.

- I’m presented and a CLAIM button I can press to transfer the tokens to my wallet
- the tokens that will be distributed will be distributed based on how many points everyone has, or based on who has the most points

**5. Viewing Ticket History.** As a user, I want to view the bid history of an auction so that I can understand the bidding patterns. I want to:

- See a list of recent buyers with addresses (anonymized) and timestamps

## User Management

**1. Account Creation.** As a new user, I want to create an account using my Solana wallet so that I can participate in auctions: I want to:

- Connect with popular Solana wallets (e.g., Phantom, Solflare)
- A “sign-in” popup is shown asking me to sign a message confirming my account. Account is created and linked to my wallet address.
- Receive a welcome message with basic platform instructions.

**2. Buying Tokens.** As a user, I want to purchase tokens with SOL so that I can participate in auctions. I want to:

- See the SOL cost clearly before confirming.
- Tokens are immediately added to my balance after purchase.

**3. Viewing Personal Stats.** As a user, I want to view my statistics so that I can track my activity and success rate. I want to:

- See total tickets participated in
- View number of auctions won
- Check my current token balance

test commit222
