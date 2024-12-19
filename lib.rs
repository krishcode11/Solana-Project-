use anchor_lang::prelude::*; //allow access everything need from Anchor
// Our program's address!
// This matches the key in the target/deploy directory

declare_id!("E72p3UnXjfuESV1UbHaN1FUt4BDnUzAzFEH4HGn8G4Ho");  //setup smart contract address

pub const  ANCHOR_DISCRIMINATOR_SIZE: usize = 8;   // Anchor programs always use 8 bits for the discriminator


#[program]
pub mod favorites {
    use super::*;
    
    // Our instruction handler! It sets the user's favorite number and color
    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        
        let _user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);

        msg!("User {user_public_key}'s favorite number is {number},  favorite  color is {color},");

        msg!(
            "User's hobbies are: {:?}",
            hobbies
        );

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
     // We can also add a get_favorites instruction handler to return the user's favorite number and color
}


#[account]    // What we will put inside the Favorites PDA
#[derive(InitSpace)]
pub struct Favorites {

    pub number: u64,

    #[max_len(50)] 
    pub color: String,
    
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

// When people call the set_favorites instruction, they will need to provide the accounts that will be modifed. This keeps Solana fast!

#[derive(Accounts)]
pub struct SetFavorites<'info> {         //liftime method of rust used
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump 
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}