use anchor_lang::prelude::*;

declare_id!("J44idcLntYU7xPmcxX89zCmaPVvLgGtiQ5q1vz1ki4GT");

#[program]
pub mod tienda_discos {
    use super::*;

    pub fn crear_disquera(ctx: Context<CrearDisquera>, nombre: String) -> Result<()> {
        require!(nombre.len() <= 50, ErrorCode::TextoMuyLargo);

        let disquera = &mut ctx.accounts.disquera;
        disquera.nombre = nombre;
        disquera.owner = ctx.accounts.owner.key();
        disquera.discos = Vec::new();
        disquera.bump = ctx.bumps.disquera; // Guardamos el bump por buena práctica

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
        
        // No es necesario el require de owner aquí porque Anchor ya lo 
        // valida implícitamente con las seeds en la estructura ModificarDisquera
        
        let disco = Disco { nombre, artista, anio };
        disquera.discos.push(disco);

        Ok(())
    }

    pub fn eliminar_registro_disco(ctx: Context<ModificarDisquera>, indice: u64) -> Result<()> {
        let disquera = &mut ctx.accounts.disquera;

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
    #[account(
        init,
        payer = owner,
        // 8 (discriminator) + 4 (string len) + 50 (nombre) + 32 (pubkey) 
        // + 4 (vec len) + (100 * (4+50 + 4+50 + 2)) (espacio para ~20 discos aprox) + 1 (bump)
        space = 8 + 60 + 32 + 2000, 
        seeds = [b"disquera", owner.key().as_ref()],
        bump
    )]
    pub disquera: Account<'info, Disquera>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarDisquera<'info> {
    #[account(
        mut,
        seeds = [b"disquera", owner.key().as_ref()],
        bump = disquera.bump, // Usamos el bump guardado
    )]
    pub disquera: Account<'info, Disquera>,

    pub owner: Signer<'info>,
}

#[account]
pub struct Disquera {
    pub nombre: String,
    pub owner: Pubkey,
    pub discos: Vec<Disco>,
    pub bump: u8, // Añadido para mayor seguridad
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
