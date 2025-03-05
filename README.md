# Elrond Smart Contract Exercise

# Organization

You have one week to complete this exercise.

Once your project is submitted, we will review it, and if it meets our expectations, we will schedule an interview with you for a technical review.

This interview will be an opportunity for you to explain your submission: code, design, etc. We will also take this opportunity to ask you technical questions.

# Critères d’évaluation

You will be evaluated on the following criteria:

- **Compliance with requirements**: Does the smart contract meet the expectations?
- **Design and architecture**: Understanding that there is not just one correct solution, your choices will need to be defended during your interview at our offices. The evaluation will focus on your arguments and explanations.
- **Best practices**: Tools used, versioning, tests, etc.
- **Bonus**: Feel free to add additional elements to showcase the breadth of your skills.

# Mission

## Part 1: Token

You must develop and deploy your own token on the Elrond testnet. You are free to configure it as you see fit (name, symbol, burn, etc.).

## Part 2: Loterie

You must develop a lottery smart contract that allows multiple participants (a parameter set during the smart contract's creation) to bet a fixed amount of your new token on a number between 0 and 9 (inclusive). Once the last participant has placed their bet, the smart contract generates a random number. Participants who bet on this number share the locked tokens. If there are no winners, the tokens are returned to the players.

### Example of a game:

- Nicolas deploys the smart contract, specifying that the number of participants is 3 and the bet amount is 10 tokens.
- Nathaël bets 10 tokens on the number 2.
- Alexandre bets 10 tokens on the number 4.
- Étienne bets 10 tokens on the number 1.
- The smart contract draws the number 2. Nathaël, therefore, receives the 30 tokens.

### Example of a second game:

- Nathaël bets 10 tokens on the number 2.
- Alexandre bets 10 tokens on the number 4.
- Étienne bets 10 tokens on the number 1.
- The smart contract draws the number 3. Each player gets their bet back.

## Part 3: AMM

So far, your token is difficult to trade. Deploy a liquidity pool so that anyone can buy or sell your token.