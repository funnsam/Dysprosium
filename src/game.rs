use core::str::FromStr;
use std::ops::Deref;

use dychess::{board::{Board, NullMoveRestorer}, color::Color, prelude::{Bitboard, Move}, square::{File, Rank, Square}};

#[derive(Clone)]
pub struct Game {
    board: Board,
    fifty_move_counter: usize,
    hash_history: HashHistory,
}

impl Game {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            fifty_move_counter: 0,
            hash_history: HashHistory::new(),
        }
    }

    pub fn board(&self) -> &Board { &self.board }

    pub fn is_capture(&self, mov: Move) -> bool {
        self.board().piece_on(mov.to()).is_some()
    }

    pub fn is_quiet(&self, mov: Move) -> bool {
        mov.promotion().is_none() && !self.is_capture(mov)
    }

    pub fn make_move(&self, mov: Move) -> Self {
        let mut fifty_move_counter = self.fifty_move_counter + 1;

        let is_pawn = (self.board.pawns() & mov.from().into()).0 != 0;
        let is_capture = (self.board.combined() & mov.to().into()).0 != 0;

        if is_pawn || is_capture {
            fifty_move_counter = 0;
        }

        let mut board = self.board;
        board.make_move(mov);

        let mut hash_history = self.hash_history.clone();
        hash_history.push(board.get_hash());

        Self { board, fifty_move_counter, hash_history }
    }

    pub fn make_null_move(&mut self) -> NullMoveRestorer {
        let restorer = self.board.null_move();
        self.fifty_move_counter += 1;
        self.hash_history.push(self.board.get_hash());

        restorer
    }

    pub fn unmake_null_move(&mut self, restorer: NullMoveRestorer) {
        self.board.restore_null_move(restorer);
        self.fifty_move_counter -= 1;
        self.hash_history.delete_last();
    }

    pub fn can_declare_draw(&self) -> bool {
        if let Some(last) = self.hash_history.last() {
            if self.hash_history.iter().filter(|i| last == **i).count() >= 3 {
                return true;
            }
        }

        self.fifty_move_counter >= 100
    }

    pub fn history_len(&self) -> usize { self.hash_history.len() }

    pub fn get_fen(&self) -> String {
        let rfen = self.board().to_string();
        format!(
            "{} {} {}",
            &rfen[..rfen.len() - 4], self.fifty_move_counter,
            self.hash_history.len() / 2 + 1,
        )
    }

    pub fn visualize(&self, bitboard: Bitboard) {
        for rank in Rank::ALL.iter().rev() {
            let get = |file| {
                let sq = Square::new(file, *rank);
                let bg = if (bitboard & sq.into()).0 != 0 { 1 } else { 232 };

                self.board().piece_on(sq).map_or_else(
                    || format!("\x1b[48;5;{bg}m \x1b[0m"),
                    |p| {
                        let c = self.board().color_on(sq).unwrap();

                        format!("\x1b[1;38;5;{};48;5;{bg}m{}\x1b[0m", 255 - c as usize * 8, p.to_char(c))
                    }
                )
            };

            let pa = get(File::A);
            let pb = get(File::B);
            let pc = get(File::C);
            let pd = get(File::D);
            let pe = get(File::E);
            let pf = get(File::F);
            let pg = get(File::G);
            let ph = get(File::H);

            if *rank == Rank::_8 {
                println!("┌───┬───┬───┬───┬───┬───┬───┬──{}┐", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][*rank as usize]);
            } else {
                println!("├───┼───┼───┼───┼───┼───┼───┼──{}┤", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][*rank as usize]);
            }
            println!("│ {pa} │ {pb} │ {pc} │ {pd} │ {pe} │ {pf} │ {pg} │ {ph} │");
        }

        println!("└ᵃ──┴ᵇ──┴ᶜ──┴ᵈ──┴ᵉ──┴ᶠ──┴ᵍ──┴ʰ──┘");
    }
}

impl core::fmt::Display for Game {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for rank in Rank::ALL.iter().rev() {
            let get = |file| {
                let sq = Square::new(file, *rank);
                let bg = if (file as u8 + *rank as u8) & 1 == 0 { 232 } else { 234 };

                self.board().piece_on(sq).map_or_else(
                    || if f.alternate() { format!("\x1b[48;5;{bg}m \x1b[0m") } else { " ".to_string() },
                    |p| {
                        let c = self.board().color_on(sq).unwrap();

                        if f.alternate() {
                            format!("\x1b[1;38;5;{};48;5;{bg}m{}\x1b[0m", 255 - c as usize * 8, p.to_char(c))
                        } else {
                            p.to_char(c).into()
                        }
                    }
                )
            };

            let pa = get(File::A);
            let pb = get(File::B);
            let pc = get(File::C);
            let pd = get(File::D);
            let pe = get(File::E);
            let pf = get(File::F);
            let pg = get(File::G);
            let ph = get(File::H);

            if *rank == Rank::_8 {
                writeln!(f, "┌───┬───┬───┬───┬───┬───┬───┬──{}┐", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][*rank as usize])?;
            } else {
                writeln!(f, "├───┼───┼───┼───┼───┼───┼───┼──{}┤", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][*rank as usize])?;
            }
            writeln!(f, "│ {pa} │ {pb} │ {pc} │ {pd} │ {pe} │ {pf} │ {pg} │ {ph} │")?;
        }

        writeln!(f, "└ᵃ──┴ᵇ──┴ᶜ──┴ᵈ──┴ᵉ──┴ᶠ──┴ᵍ──┴ʰ──┘")?;
        writeln!(f)?;

        writeln!(f, "FEN: {}", self.get_fen())?;
        writeln!(f, "Hash: 0x{:016x}", self.board().get_hash())?;
        writeln!(f)?;

        let phase = crate::eval::game_phase(self.board()) as usize;
        writeln!(f, "Phase: {0:█<1$}{0:░<phase$} end", "", 24 - phase)?;

        Ok(())
    }
}

impl FromStr for Game {
    type Err = Box<dyn core::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, moves) = s.rsplit_once(' ').ok_or("")?;
        let (_, fmc) = rest.rsplit_once(' ').ok_or("")?;
        let board = Board::from_epd(false, s).map_err(|_| "")?;

        Ok(Self {
            board,
            fifty_move_counter: fmc.parse()?,
            hash_history: HashHistory::unqiue(moves.parse::<usize>()? * 2 - (board.side_to_move() == Color::White) as usize),
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(Board::default())
    }
}

const HASH_HISTORY_LEN: usize = 128;

#[derive(Clone)]
pub struct HashHistory {
    inner: [u64; HASH_HISTORY_LEN],
    len: usize,
}

impl HashHistory {
    pub const fn new() -> Self {
        Self {
            inner: [0; HASH_HISTORY_LEN],
            len: 0,
        }
    }

    pub fn unqiue(len: usize) -> Self {
        Self {
            inner: core::array::from_fn(|i| i as u64),
            len,
        }
    }

    pub const fn len(&self) -> usize { self.len }

    pub fn push(&mut self, val: u64) {
        self.inner[self.len % HASH_HISTORY_LEN] = val;
        self.len += 1;
    }

    pub fn delete_last(&mut self) {
        self.len -= 1;
    }

    pub fn last(&self) -> Option<u64> {
        if self.len() == 0 { return None };

        let i = (self.len - 1) % HASH_HISTORY_LEN;
        Some(self.inner[i])
    }
}

impl Deref for HashHistory {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        &self.inner[..self.len().min(HASH_HISTORY_LEN)]
    }
}
