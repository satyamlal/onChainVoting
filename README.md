# onChainVoting
Lightweight on-chain voting program built with Anchor on Solana.

What this is
- An Anchor (Rust) program that implements an on-chain voting flow for Solana.
- Includes the program source (`programs/on_chain_voting/src`)

Tech stack
- Rust + Anchor for the on-chain program
- Solana CLI and Anchor CLI for local/devnet deployment
 - Yarn (package manager) for JavaScript dependencies (this repo includes `yarn.lock`)

Prerequisites
- Install Solana CLI: https://docs.solana.com/cli
- Install Anchor: https://book.anchor-lang.com/
- Install Rust, solanaCLI, and AnchorCLI toolchain
- Node.js & npm (for `app/` and migrations)
 - Yarn (preferred) or npm to install JS dependencies (see `yarn.lock`)

Clone

```bash
git clone <your-repo-url>
cd onChainVoting
```

JavaScript / Yarn

```bash
yarn                       # install JS dependencies (uses yarn.lock)
yarn build                 # build the client (if defined in package.json)
yarn start                 # run the client or scripts (if defined)
```

Solana commands

```bash
solana address                     # show current wallet address
solana balance                     # show SOL balance
solana airdrop 2                   # request 2 SOL (devnet)
solana-keygen new -o target/deploy/on_chain_voting-keypair.json --force  # create program keypair
anchor keys sync                   # sync keys to Anchor (Anchor.toml)
anchor build                       # build the Anchor program
anchor program deploy --provider.cluster devnet   # deploy program to devnet
```

If you hit timeout / signing errors

```bash
anchor program deploy --provider.cluster devnet -- --max-sign-attempts 30
```

Or deploy the generated program file with the Solana CLI directly

```bash
solana program deploy target/deploy/on_chain_voting.so \
  --url devnet \
  --keypair ~/.config/solana/id.json
```