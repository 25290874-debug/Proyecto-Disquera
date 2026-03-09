use anchor_lang::prelude::*;

declare_id!("J44idcLntYU7xPmcxX89zCmaPVvLgGtiQ5q1vz1ki4GT");

#[program]
pub mod tienda_discos {
    use super::*;

    pub fn crear_disquera(
        ctx: Context<CrearDisquera>,
        nombre: String,
    ) -> Result<()> {

        require!(nombre.len() <= 50, ErrorCode::TextoMuyLargo);

        let disquera = &mut ctx.accounts.disquera;

        disquera.nombre = nombre;
        disquera.owner = ctx.accounts.owner.key();
        disquera.discos = Vec::new();

        Ok(())
    }

    pub fn agregar_discos(
        ctx: Context<ModificarDisquera>,
        nombre: String,
        artista: String,
        anio: u16,
    ) -> Result<()> {

        require!(nombre.len() <= 50, ErrorCode::TextoMuyLargo);
        require!(artista.len() <= 50, ErrorCode::TextoMuyLargo);

        let disquera = &mut ctx.accounts.disquera;

        require!(
            disquera.owner == ctx.accounts.owner.key(),
            ErrorCode::NoAutorizado
        );

        let disco = Disco {
            nombre,
            artista,
            anio,
        };

        disquera.discos.push(disco);

        Ok(())
    }

    pub fn eliminar_registro_disco(
        ctx: Context<ModificarDisquera>,
        indice: u64,
    ) -> Result<()> {

        let disquera = &mut ctx.accounts.disquera;

        require!(
            disquera.owner == ctx.accounts.owner.key(),
            ErrorCode::NoAutorizado
        );

        require!(
            (indice as usize) < disquera.discos.len(),
            ErrorCode::IndiceInvalido
        );

        disquera.discos.remove(indice as usize);

        Ok(())
    }

    pub fn actualizar_disco(
        ctx: Context<ModificarDisquera>,
        indice: u64,
        nombre: String,
        artista: String,
        anio: u16,
    ) -> Result<()> {

        require!(nombre.len() <= 50, ErrorCode::TextoMuyLargo);
        require!(artista.len() <= 50, ErrorCode::TextoMuyLargo);

        let disquera = &mut ctx.accounts.disquera;

        require!(
            disquera.owner == ctx.accounts.owner.key(),
            ErrorCode::NoAutorizado
        );

        require!(
            (indice as usize) < disquera.discos.len(),
            ErrorCode::IndiceInvalido
        );

        let disco = &mut disquera.discos[indice as usize];

        disco.nombre = nombre;
        disco.artista = artista;
        disco.anio = anio;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearDisquera<'info> {

    #[account(init, payer = owner, space = 8 + 2000)]
    pub disquera: Account<'info, Disquera>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarDisquera<'info> {

    #[account(mut)]
    pub disquera: Account<'info, Disquera>,

    pub owner: Signer<'info>,
}

#[account]
pub struct Disquera {
    pub nombre: String,
    pub owner: Pubkey,
    pub discos: Vec<Disco>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Disco {
    pub nombre: String,
    pub artista: String,
    pub anio: u16,
}

#[error_code]
pub enum ErrorCode {

    #[msg("El texto es demasiado largo")]
    TextoMuyLargo,

    #[msg("No estas autorizado")]
    NoAutorizado,

    #[msg("Indice invalido")]
    IndiceInvalido,
}
