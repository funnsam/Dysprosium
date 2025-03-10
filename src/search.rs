use core::sync::atomic::Ordering;

use crate::{*, eval::*, trans_table::*};
use chess::{BoardStatus, ChessMove, MoveGen, Piece};
use move_order::KillerTable;
use node::{Cut, NodeType, Pv};

impl Engine {
    pub fn best_move<F: FnMut(&Self, (ChessMove, Eval, usize)) -> bool>(&mut self, mut cont: F) -> (ChessMove, Eval, usize) {
        self.time_ref = Instant::now();
        self.total_nodes_searched.store(0, Ordering::Relaxed);
        self.debug.clear();

        let mut main_thread = self.new_thread::<true>(0);

        let can_time_out = self.can_time_out.swap(false, Ordering::Relaxed);
        let prev = main_thread.root_search(1, Eval::MIN, Eval::MAX);
        self.can_time_out.store(can_time_out, Ordering::Relaxed);
        let mut prev = (prev.0, prev.1, 1);
        if !cont(self, prev) || self.soft_times_up() { return prev };

        *self.smp_prev.lock() = prev.1;
        self.smp_abort.initiate_wait();

        let mut sum = 0;
        while sum < self.smp_count {
            sum += self.smp_start.notify_all();
        }

        for depth in 2..=255 {
            let this = main_thread.root_aspiration(depth, prev.1);

            if self.hard_times_up() { break };

            prev = (this.0, this.1, depth);
            if !cont(self, prev) || self.soft_times_up() { break };
        }

        self.smp_abort.initiate();
        prev
    }
}

impl SmpThread<'_, false> {
    pub fn start(mut self) {
        while !self.smp_exit.initiated() {
            self.smp_abort.always_wait();

            let mut prev = {
                let mut lock = self.smp_prev.lock();
                self.smp_start.wait(&mut lock);

                *lock
            };

            for depth in 2..=255 {
                prev = self.root_aspiration(depth, prev).1;
                if self.abort() { break };
            }
        }
    }
}

impl<const MAIN: bool> SmpThread<'_, MAIN> {
    fn root_aspiration(&mut self, depth: usize, prev: Eval) -> (ChessMove, Eval) {
        let (alpha, beta) = (prev - 25, prev + 25);
        let (mov, eval, nt) = self.root_search(depth, alpha, beta);

        if nt != NodeType::Pv {
            let (mov, eval, _) = self.root_search(depth, Eval::MIN, Eval::MAX);
            (mov, eval)
        } else { (mov, eval) }
    }

    #[inline]
    fn root_search(
        &mut self,
        depth: usize,
        alpha: Eval,
        beta: Eval,
    ) -> (ChessMove, Eval, NodeType) {
        self.nodes_searched = 0;

        let game: Game = self.game.read().clone();
        let (next, eval, nt) = self._evaluate_search::<Pv, true>(ChessMove::default(), &game, &KillerTable::new(), depth, 0, alpha, beta, false);

        self.store_tt(depth, &game, (next, eval, nt));
        self.total_nodes_searched.fetch_add(self.nodes_searched, Ordering::Relaxed);

        (next, eval, nt)
    }

    fn abort(&self) -> bool {
        if !MAIN {
            self.smp_abort.initiated()
        } else {
            self.hard_times_up()
        }
    }

    #[inline]
    fn zw_search<Node: node::Node>(
        &mut self,
        prev_move: ChessMove,
        game: &Game,
        killer: &KillerTable,
        depth: usize,
        ply: usize,
        beta: Eval,
    ) -> Eval {
        self.evaluate_search::<Node>(prev_move, game, killer, depth, ply, beta - 1, beta, true)
    }

    /// Perform an alpha-beta (fail-soft) negamax search and return the evaluation
    #[inline]
    fn evaluate_search<Node: node::Node>(
        &mut self,
        prev_move: ChessMove,
        game: &Game,
        killer: &KillerTable,
        depth: usize,
        ply: usize,
        alpha: Eval,
        beta: Eval,
        in_zw: bool,
    ) -> Eval {
        let (next, eval, nt) = self._evaluate_search::<Node, false>(prev_move, game, killer, depth, ply, alpha, beta, in_zw);

        self.store_tt(depth, game, (next, eval, nt));

        eval
    }

    fn store_tt(&self, depth: usize, game: &Game, (next, eval, nt): (ChessMove, Eval, NodeType)) {
        if nt != NodeType::None && !self.abort() {
            if let Some(tte) = self.trans_table.get_place(game.board().get_hash()) {
                if tte.depth as usize > depth {
                    return;
                }
            }

            self.trans_table.insert(game.board().get_hash(), TransTableEntry {
                depth: depth as u8,
                eval,
                next,
                flags: TransTableEntry::new_flags(nt),
            });
        }
    }

    fn _evaluate_search<Node: node::Node, const ROOT: bool>(
        &mut self,
        prev_move: ChessMove,
        game: &Game,
        p_killer: &KillerTable,
        depth: usize,
        ply: usize,
        mut alpha: Eval,
        beta: Eval,
        in_zw: bool,
    ) -> (ChessMove, Eval, NodeType) {
        if game.can_declare_draw() {
            return (ChessMove::default(), Eval(0), NodeType::None);
        }

        if !Node::PV {
            if let Some(trans) = self.trans_table.get(game.board().get_hash()) {
                let eval = trans.eval;
                let node_type = trans.node_type();

                if trans.depth as usize >= depth && (node_type == NodeType::Pv
                    || (node_type == NodeType::Cut && eval >= beta)
                    || (node_type == NodeType::All && eval < alpha)) {
                    return (trans.next, eval, NodeType::None);
                }
            }
        }

        match game.board().status() {
            BoardStatus::Ongoing => {},
            BoardStatus::Checkmate => return (ChessMove::default(), -Eval::M0, NodeType::None),
            BoardStatus::Stalemate => return (ChessMove::default(), Eval(0), NodeType::None),
        }

        if self.abort() {
            return (ChessMove::default(), Eval(0), NodeType::None);
        }

        if depth == 0 {
            return (ChessMove::default(), self.quiescence_search(game, alpha, beta), NodeType::None);
        }

        let killer = KillerTable::new();

        // internal iterative reductions
        if !ROOT && depth >= 4 && self.trans_table.get(game.board().get_hash()).is_none() {
            let low = self._evaluate_search::<Node, ROOT>(prev_move, game, &killer, depth / 4, ply, alpha, beta, false);
            self.store_tt(depth / 4, game, low);

            if low.1 <= alpha {
                return (low.0, low.1, NodeType::None);
            }
        }

        let in_check = game.board().checkers().0 != 0;

        // null move pruning
        if !Node::PV && !in_check && depth >= 4 && (
            game.board().pieces(Piece::Knight).0 != 0 ||
            game.board().pieces(Piece::Bishop).0 != 0 ||
            game.board().pieces(Piece::Rook).0 != 0 ||
            game.board().pieces(Piece::Queen).0 != 0
        ) {
            let game = game.make_null_move().unwrap();
            let r = 3 + depth / 3;
            let eval = -self.zw_search::<Cut>(prev_move, &game, &killer, depth - r, ply + 1, 1 - beta);

            if eval >= beta {
                return (ChessMove::default(), eval.incr_mate(), NodeType::None);
            }
        }

        let tte = self.trans_table.get(game.board().get_hash());

        let mut moves = MoveGen::new_legal(game.board())
            .map(|m| (m, self.move_score(m, prev_move, game, &tte, &p_killer)))
            .collect::<arrayvec::ArrayVec<_, 256>>();
        moves.sort_unstable_by_key(|i| -i.1);
        if ROOT && !MAIN {
            let len = moves.len();
            moves.rotate_left((self.index / 2) % len);
        }

        let mut best = (ChessMove::default(), Eval::MIN);
        let mut children_searched = 0;
        let _game = &game;
        for (i, (m, _)) in moves.iter().copied().enumerate() {
            let game = _game.make_move(m);

            // futility pruning: kill nodes with no potential
            if !in_check && depth <= 2 {
                let eval = -evaluate_static(game.board());
                let margin = 100 * depth as i16 * depth as i16;

                if eval.0 + margin < alpha.0 {
                    if best.0 == ChessMove::default() {
                        best = (m, eval - margin);
                    }

                    continue;
                }
            }

            let can_reduce = depth >= 3 && !in_check && children_searched != 0;

            let mut eval = Eval(i16::MIN);
            let do_full_research = if can_reduce {
                eval = -self.zw_search::<Node::Zw>(m, &game, &killer, depth / 2, ply + 1, -alpha);

                if alpha < eval && depth / 2 < depth - 1 {
                    self.debug.research.inc();
                } else {
                    self.debug.no_research.inc();
                }

                alpha < eval && depth / 2 < depth - 1
            } else {
                !Node::PV || children_searched != 0
            };

            if do_full_research {
                eval = -self.zw_search::<Node::Zw>(m, &game, &killer, depth - 1, ply + 1, -alpha);
                self.debug.all_full_zw.inc();
            }

            if Node::PV && (children_searched == 0 || alpha < eval) {
                eval = -self.evaluate_search::<Pv>(m, &game, &killer, depth - 1, ply + 1, -beta, -alpha, in_zw);

                self.debug.all_full.inc();
                if do_full_research {
                    self.debug.full.inc();
                }
            }

            if self.abort() { return (best.0, best.1.incr_mate(), NodeType::None) };
            self.nodes_searched += 1;

            // if ROOT {
            //     println!(" {m} {eval} α{alpha} β{beta} {:?}", self.find_pv(m, 100).into_iter().map(|i| i.to_string()).collect::<Vec<_>>());
            // }

            if eval > best.1 || best.0 == ChessMove::default() {
                best = (m, eval);
                alpha = alpha.max(eval);
            }
            if eval >= beta {
                if !_game.is_capture(m) {
                    let bonus = 300 * depth as isize - 250;

                    for (m, _) in moves[..i].into_iter() {
                        if !_game.is_capture(*m) {
                            self.hist_table.update(*m, -bonus);
                            p_killer.update(*m, -bonus);
                        }
                    }

                    self.hist_table.update(m, bonus);
                    p_killer.update(m, bonus);
                    *self.countermove.get_mut(prev_move) = m;
                }

                return (best.0, best.1.incr_mate(), NodeType::Cut);
            }

            children_searched += 1;
        }

        (best.0, best.1.incr_mate(), if best.1 == alpha { NodeType::All } else { NodeType::Pv })
    }

    fn quiescence_search(&mut self, game: &Game, mut alpha: Eval, beta: Eval) -> Eval {
        let standing_pat = evaluate_static(game.board());
        // TODO: failing to standing pat makes sprt fail, need investigation
        if standing_pat >= beta { return beta; }
        alpha = alpha.max(standing_pat);
        let mut best = standing_pat;

        let mut moves = MoveGen::new_legal(game.board());
        moves.set_iterator_mask(*game.board().combined());

        for m in moves {
            if see(game, m) < 0 { continue };

            let game = game.make_move(m);
            let eval = -self.quiescence_search(&game, -beta, -alpha);
            self.nodes_searched += 1;

            if eval > best {
                best = eval;
                alpha = alpha.max(eval);
            }
            if eval >= beta {
                return best;
            }
        }

        best
    }
}
