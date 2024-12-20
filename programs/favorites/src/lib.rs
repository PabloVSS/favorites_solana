//importando o anchor
use anchor_lang::prelude::*;

//Endereço público do programa no blockchain solana
declare_id!("5oq5zcrgthw1wjPprz6sAkTeGSYDQmUKkiRDV334Vndm");

// O Anchor adiciona um discriminador de 8 bites no ínicio de cada conta armazenada no blockchain
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

//Definindo que é um programa solano
#[program]
//Define o modulo do programa que contém a lógica do programa
pub mod favorites {
    use super::*;
    //A função que definirá os favoritos do usuario
    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greentings from {}", context.program_id);
        let _user_public_key = context.accounts.user.key();

        msg!("User {_user_public_key}'s favorite number is {number}, favorite color is {color}, and their hobbies are {hobbies:?} ");

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

//Definir que o struct é uma conta solana
#[account]
//Implementa o InitSpace, que é necessario para calcular o espaço para a conta
#[derive(InitSpace)]
pub struct Favorites {
    //número positivo de 64 bits
    pub number: u64,

    //Cor que será uma String de tamanho 50
    #[max_len(50)]
    pub color: String,

    //E os hobbies do usuario, do tipo verto e serão no máximo 5 hobbies de 50 caracteres cada
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
//A estrutura da função
pub struct SetFavorites<'info> {
    //Definará o úsurio que fará a assinatura da transação
    #[account(mut)]
    pub user: Signer<'info>,

    //Representa a conta de dados que será criada ou modificada para armazenas os favoritos
    #[account(
        init_if_needed,
        //usuario responsavel
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump

    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
