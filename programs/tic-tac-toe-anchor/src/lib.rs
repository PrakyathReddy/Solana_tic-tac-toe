use anchor_lang::prelude::*; // imports all items from the 'prelude' module of the 'anchor_lang' crate
use num_derive::*; // this crate provides procedural macros to derive numeric traits in Rust like FromPrimitive and ToPrimitive
use num_traits::*; // this crate provides a collection of numeric traits that describe properties of primitive numeric types

use crate::program::TicTacToeAnchor; // is a path pointing to the 'TicTacToeAchor' item inside the program module of the current state


declare_id!("BwAT2NVQuxS4wuvzSd4MjPUbxMZm4yv791C7E62yYJUp"); // this macro defines the unique program id of a given solana program. Anchor provides a local development environment where it automatically handles the program ID for you, so you don't have to worry about it.

#[program] // this anchor attribute is a procedural macro that denotes the program's main entry point. similar to main function. Under this attribute u define fn's that represent the different instructions or operations that our Solana program can handle.
pub mod tic_tac_toe_anchor { // modules in Rust are used to organize code into namespaces
    use super::*; // brings all public items from the parent module into scope

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    } // initialize fn is an instruction handler for a solana program written using anchor. 
    // ctx is a struct that contains the accounts and client information involved in the transaction. 'Context' struct is a generic type provided by anchor, and the initialize type inside the brackets is defined elsewhere in the program. This initialize type represents the specific accounts that the 'initialize' instruction expects.
    // Result is a function that return a result type - success (Ok) or failure (Err). If there was an error, the funtion will return an 'Err' variant that contains info about what went wrong
    // Ok(()) - this is the body of the function. It simply returns 'Ok(())' indicating that this function always succeeds. 
}

#[derive(Accounts)]
pub struct Initialize {}


#[account]
pub struct Game {
    players: [Pubkey; 2],
    turn: u8,
    board: [[Option<Sign>; 3]; 3], 
    state: GameState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    FromPrimitive,
    ToPrimitive,
    Copy,
    Clone,
    PartialEq,
    Eq
)]
pub enum Sign {
    X,
    O,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}

#[error_code]
pub enum TicTacToeError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
    GameAlreadyStarted,
}

impl Game {
    pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1)) + (32 + 1);

    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq!(self.turn, 0, TicTacToeError::GameAlreadyStarted);
        self.players = players;
        self.turn = 1;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        require!(self.is_active(), TicTacToeError::GameAlreadyOver);

        match tile {
            tile @ Tile{
                row: 0..=2,
                column: 0..=2,
            } => match self.board[tile.row as usize][tile.column as usize] {
                Some(_) => return Err(TicTacToeError::TileAlreadySet.into()),
                None => {
                    self.board[tile.row as usize][tile.column as usize] = 
                        Some(Sign::from_usize(self.current_player_index()).unwrap());
                }
            },
            _ => return Err(TicTacToeError::TileOutOfBounds.into()),
        }

        self.update_state();

        if GameState::Active == self.state {
            self.turn += 1;
        }

        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some() 
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    fn update_state(&mut self) {
        for i in 0..=2 {
            // three of the same in one row
            if self.is_winning_trio([(i,0), (i,1), (i,2)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
            // three of the same in one column
            if self.is_winning_trio([(0,i), (1,i), (2,i)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            } 
        }
        // three of the same in one diagonal
        if self.is_winning_trio([(0,0), (1,1), (2,2)])
            || self.is_winning_trio([(0,2), (1,1), (2,0)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
        
        // reaching this code means the game has not been won,
        // so if there are unfilled tiles left, it's still active
        for row in 0..=2 {
            for column in 0..=2 {
                if self.board[row][column].is_none() {
                    return;
                }
            }
        }

        // game has not been won 
        // game has no more free tiles
        // -> game ends in a tie
        self.state = GameState::Tie;
    }
}