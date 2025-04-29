use core::sync::atomic::Ordering;

use crate::{eval::*, line::{EvalCell, PrevMove}, trans_table::TransTableEntry, *};
use bound::Bound;
use chess::{BoardStatus, ChessMove, MoveGen};
use node::{NodeType, Pv};

mod bound;
pub mod move_ord;
pub mod params;
mod quiescence;

impl Engine {
    pub fn best_move<F: FnMut(&Self, (ChessMove, Eval, usize)) -> bool>(&mut self, mut cont: F) -> (ChessMove, Eval, usize) {
        self.time_ref = Instant::now();
        self.total_nodes_searched.store(0, Ordering::Relaxed);
        self.debug.clear();

        let mut main_thread = self.new_thread::<true>(0);

        let can_time_out = self.can_time_out.swap(false, Ordering::Relaxed);
        let prev = main_thread.root_search(1);
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
            let this = main_thread.root_search(depth);

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

            // let mut prev = {
            //     let mut lock = self.smp_prev.lock();
            //     self.smp_start.wait(&mut lock);

            //     *lock
            // };

            for depth in 2..=255 {
                self.root_search(depth).1;
                if self.abort() { break };
            }
        }
    }
}

impl<const MAIN: bool> SmpThread<'_, MAIN> {
    fn root_search(
        &mut self,
        depth: usize,
    ) -> (ChessMove, Eval, NodeType) {
        self.nodes_searched = 0;

        let game: Game = self.game.read().clone();
        let line = PrevMove {
            mov: ChessMove::default(),
            static_eval: EvalCell::new(game.board()),
            prev_move: None,
        };

        let ret = self._evaluate_search::<Pv, true>(&line, &game, depth, 0, Bound::MIN_MAX);
        self.total_nodes_searched.fetch_add(self.nodes_searched, Ordering::Relaxed);

        ret
    }

    fn abort(&self) -> bool {
        if !MAIN {
            self.smp_abort.initiated()
        } else {
            self.hard_times_up()
        }
    }

    #[inline]
    fn evaluate_search<Node: node::Node>(
        &mut self,
        prev_move: &PrevMove,
        game: &Game,
        depth: usize,
        ply: usize,
        bound: Bound,
    ) -> Eval {
        let (next, eval, nt) = self._evaluate_search::<Node, false>(
            prev_move,
            game,
            depth,
            ply,
            bound,
        );

        if self.trans_table.get_place(game.board().get_hash())
            .is_none_or(|e| e.depth <= depth as u8)
        {
            self.trans_table.insert(game.board().get_hash(), TransTableEntry {
                depth: depth as _,
                eval,
                next,
                flags: TransTableEntry::new_flags(nt),
            });
        }

        eval
    }

    fn _evaluate_search<Node: node::Node, const ROOT: bool>(
        &mut self,
        prev_move: &PrevMove,
        game: &Game,
        depth: usize,
        ply: usize,
        mut bound: Bound,
    ) -> (ChessMove, Eval, NodeType) {
        if game.can_declare_draw() {
            return (ChessMove::default(), Eval(0), NodeType::None);
        }

        match game.board().status() {
            BoardStatus::Ongoing => {},
            BoardStatus::Checkmate => return (ChessMove::default(), -Eval::M0, NodeType::None),
            BoardStatus::Stalemate => return (ChessMove::default(), Eval(0), NodeType::None),
        }

        if self.abort() {
            return (ChessMove::default(), Eval(0), NodeType::None);
        }

        let tt = self.trans_table.get(game.board().get_hash());

        if let Some(tt) = tt {
            let eval = tt.eval;
            let cutoff_ok = !ROOT
                && tt.depth >= depth as _
                && (match tt.node_type() {
                    NodeType::Pv => true,
                    NodeType::All => eval < bound.alpha,
                    NodeType::Cut => eval >= bound.beta,
                    NodeType::None => false,
                });

            if cutoff_ok {
                return (tt.next, eval, NodeType::None);
            }
        }

        if depth == 0 {
            return (ChessMove::default(), self.quiescence_search(game, bound), NodeType::None);
        }

        let in_check = game.board().checkers().0 != 0;

        // reverse futility pruning
        if !Node::PV && !in_check && depth == 1 {
            let eval = *prev_move.static_eval;
            let margin = self.sparams.rfp_margin_coeff * depth as i16;

            if eval >= bound.beta + margin {
                return (ChessMove::default(), eval, NodeType::None);
            }
        }

        // null move pruning
        if !in_check && depth > 3 {
            let r = 3;
            let next_game = game.make_null_move().unwrap();
            let prev_move = PrevMove {
                mov: ChessMove::default(),
                static_eval: EvalCell::new(next_game.board()),
                prev_move: Some(prev_move),
            };

            let eval = -self.evaluate_search::<Node>(&prev_move, &next_game, depth - r, ply + 1, bound.neg_beta_zw());

            if eval >= bound.beta {
                return (ChessMove::default(), eval, NodeType::None);
            }
        }

        let mut children_searched = 0;
        let mut best_eval = Eval::MIN;
        let mut best_move = ChessMove::default();
        let mut node_type = NodeType::All;

        let mut moves = MoveGen::new_legal(game.board())
            .map(|m| (m, self.move_score(game, tt, m)))
            .collect::<arrayvec::ArrayVec<_, 256>>();
        moves.sort_unstable_by_key(|i| !i.1);

        for (m, _) in moves {
            let next_game = game.make_move(m);
            let line = PrevMove {
                mov: m,
                static_eval: EvalCell::new(next_game.board()),
                prev_move: Some(prev_move),
            };

            // principal variation search
            let mut eval = None;

            if !Node::PV || children_searched > 0 {
                eval = Some(-self.evaluate_search::<Node::Zw>(&line, &next_game, depth - 1, ply + 1, bound.neg_alpha_zw()));
            }

            if Node::PV && (children_searched == 0 || eval.is_some_and(|e| e > bound.alpha)) {
                eval = Some(-self.evaluate_search::<Pv>(&line, &next_game, depth - 1, ply + 1, -bound));
            }

            let eval = eval.unwrap();

            // timeout detection
            if self.abort() {
                return (best_move, best_eval.incr_mate(), NodeType::None);
            }

            self.nodes_searched += 1;

            if eval >= bound.beta {
                best_eval = eval;
                best_move = m;
                node_type = NodeType::Cut;

                self.hist_table.add_bonus(m, depth);
                break;
            }

            if eval > best_eval || best_move == ChessMove::default() {
                best_eval = eval;
                best_move = m;

                if eval > bound.alpha {
                    bound.alpha = eval;
                    node_type = NodeType::Pv;
                }
            }

            children_searched += 1;
        }

        (best_move, best_eval.incr_mate(), node_type)
    }
}
