use core::str::FromStr;
use std::ops::Deref;

#[derive(Clone)]
pub struct Game {
    board: chess::Board,
    fifty_move_counter: usize,
    hash_history: HashHistory,
}

impl Game {
    pub fn new(board: chess::Board) -> Self {
        Self {
            board,
            fifty_move_counter: 0,
            hash_history: HashHistory::new(),
        }
    }

    pub fn board(&self) -> &chess::Board { &self.board }

    pub fn is_capture(&self, mov: chess::ChessMove) -> bool {
        self.board().piece_on(mov.get_dest()).is_some()
    }

    pub fn is_quiet(&self, mov: chess::ChessMove) -> bool {
        mov.get_promotion().is_none() && !self.is_capture(mov)
    }

    pub fn make_move(&self, mov: chess::ChessMove) -> Self {
        let mut fifty_move_counter = self.fifty_move_counter + 1;

        let is_pawn = (self.board.pieces(chess::Piece::Pawn) & chess::BitBoard::from_square(mov.get_source())).0 != 0;
        let is_capture = (self.board.combined() & chess::BitBoard::from_square(mov.get_dest())).0 != 0;

        if is_pawn || is_capture {
            fifty_move_counter = 0;
        }

        let board = self.board.make_move_new(mov);

        let mut hash_history = self.hash_history.clone();
        hash_history.push(board.get_hash());

        Self { board, fifty_move_counter, hash_history }
    }

    pub fn make_null_move(&self) -> Option<Self> {
        let board = self.board.null_move()?;
        let fifty_move_counter = self.fifty_move_counter + 1;

        let mut hash_history = self.hash_history.clone();
        hash_history.push(board.get_hash());

        Some(Self { board, fifty_move_counter, hash_history })
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

    pub fn visualize(&self, bitboard: chess::BitBoard) {
        for rank in chess::ALL_RANKS.iter().rev() {
            let get = |file| {
                let sq = chess::Square::make_square(*rank, file);
                let bg = if (chess::BitBoard::from_square(sq) & bitboard).0 != 0 { 1 } else { 232 };

                self.board().piece_on(sq).map_or_else(
                    || format!("\x1b[48;5;{bg}m \x1b[0m"),
                    |p| {
                        let c = self.board().color_on(sq).unwrap();

                        format!("\x1b[1;38;5;{};48;5;{bg}m{}\x1b[0m", 255 - c.to_index() * 8, p.to_string(c))
                    }
                )
            };

            let pa = get(chess::File::A);
            let pb = get(chess::File::B);
            let pc = get(chess::File::C);
            let pd = get(chess::File::D);
            let pe = get(chess::File::E);
            let pf = get(chess::File::F);
            let pg = get(chess::File::G);
            let ph = get(chess::File::H);

            if *rank == chess::Rank::Eighth {
                println!("┌───┬───┬───┬───┬───┬───┬───┬──{}┐", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][rank.to_index()]);
            } else {
                println!("├───┼───┼───┼───┼───┼───┼───┼──{}┤", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][rank.to_index()]);
            }
            println!("│ {pa} │ {pb} │ {pc} │ {pd} │ {pe} │ {pf} │ {pg} │ {ph} │");
        }

        println!("└ᵃ──┴ᵇ──┴ᶜ──┴ᵈ──┴ᵉ──┴ᶠ──┴ᵍ──┴ʰ──┘");
    }
}

impl core::fmt::Display for Game {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for rank in chess::ALL_RANKS.iter().rev() {
            let get = |file| {
                let sq = chess::Square::make_square(*rank, file);
                let bg = if (file.to_index() + rank.to_index()) & 1 == 0 { 232 } else { 234 };

                self.board().piece_on(sq).map_or_else(
                    || if f.alternate() { format!("\x1b[48;5;{bg}m \x1b[0m") } else { " ".to_string() },
                    |p| {
                        let c = self.board().color_on(sq).unwrap();

                        if f.alternate() {
                            format!("\x1b[1;38;5;{};48;5;{bg}m{}\x1b[0m", 255 - c.to_index() * 8, p.to_string(c))
                        } else {
                            p.to_string(c)
                        }
                    }
                )
            };

            let pa = get(chess::File::A);
            let pb = get(chess::File::B);
            let pc = get(chess::File::C);
            let pd = get(chess::File::D);
            let pe = get(chess::File::E);
            let pf = get(chess::File::F);
            let pg = get(chess::File::G);
            let ph = get(chess::File::H);

            if *rank == chess::Rank::Eighth {
                writeln!(f, "┌───┬───┬───┬───┬───┬───┬───┬──{}┐", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][rank.to_index()])?;
            } else {
                writeln!(f, "├───┼───┼───┼───┼───┼───┼───┼──{}┤", ['₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈'][rank.to_index()])?;
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
        let board = chess::Board::from_str(s).map_err(|_| "")?;

        Ok(Self {
            board,
            fifty_move_counter: fmc.parse()?,
            hash_history: HashHistory::unqiue(moves.parse::<usize>()? * 2 - (board.side_to_move() == chess::Color::White) as usize),
        })
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(chess::Board::default())
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
