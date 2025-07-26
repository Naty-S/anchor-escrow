# Anchor escrow

Is a place (black box) that controls exchanges based on conditions, checks those are met.
>Mediator for trustless transactions.

Exchange between 2, A(maker) and B(taker), gonna make an agreetment and set some conditions,
they're gonna exchage one thing for another (A gives `a` to B and B gives `b` to A):

1. A sticks `a` to the box
2. The box checks
   1. Is the `a` said
   2. A is A
   3. A has a slot that holds `a`
   4. B has a slot for `a`
   5. The `b` is the one agreed upon
   6. B is B
3. If all is met, then the box release `a` to B and `b` to A

The box has:

- A and B indentities.
- A's `a` tokens
- B's `b` tokens
- Conditions/constraints

**A swap**:

1. A send token `a` to a vault and wait for B to send `b` token
2. Escrow checks
   1. The tokens sent (`a`) are the correct ones
   2. The amount is the correct one
3. If all is right then it will send `b` tokens to A (from B's ATA to A's ATA) and the `a` tokens from the vault to B's ATA

## Concepts

**Maker**: creates the escrow. Initializes the deal saying:
"I want `b` tokens and I'm willing to give `a` tokens".

>Can refund if he wants to claim his funds back, before the taker takes the deal tho.

**Taker**: takes the deal.
>Any taker can take the deal.

>Needs to know the offer of the maker.

**InterfaceAccount**: Allows the use of old (spl) and new (2022) token types.

**associated_token_program**: Allows custom tokens. Owns the token accounts.

**TokenAccount**: Manages the tokens (vault trades them).

**.to_account_info()**: when doing cpi

**transfer_checked**: Ensures the correct ammount and token gets transfered, checks the token decimals.

**init_if_needed**: Inits the ATA for Maker if not having one yet.

>Be aware of security implications, program design

**has_one**: Verifies account/struct (Escrow) field with the account/pubkey in the instruction (Take),
so they are the same and the right escrow is called, the same the maker made.

> like having: assert!(self.mint_a.key() == self.escrow.mint_a);

**.set_iiner**:

## Functions

- Make
- Deposit
- Take
- Refund

## Macros

- `#[instruction(...)]`: Allows to pass arguments to the program.

## Important notes

token_interface
>Use for token type management, spl or 2022. It will handle wich one to use unless is specified
>(have 2 token interfaces, **associated_token_program**) so the program can use both at the same time.

btw guys: when we do 
`#[instruction(seed: u64)]`
>The first argument for the init function must be `seed`, after self; if not it will fail

`anchor_spl::token::transfer_checked`
>Old token program and will not be able to handle the new one

Deposit
>Doesn't check de ammount when transfer 'cuz the token program fails if the ATA doesn't have enough.
