use super::{engine, random, ChessEngine};
use crate::domain::game::GameStatus;
use crate::domain::{chess_set, game, rulebook};
use std::collections::HashMap;

const UPPER_CONFIDENCE_BOUND_BIAS: f32 = 1.4;

pub struct MonteCarloTreeSearch;

impl ChessEngine for MonteCarloTreeSearch {
    fn generate_next_move(
        &self,
        game: &game::Game,
    ) -> Result<rulebook::Move, engine::SuggestNextMoveError> {
        let Some(to_play_colour) = game.get_status().to_play_colour() else {
            return Err(engine::SuggestNextMoveError::GameHasAlreadyEnded);
        };

        let mut mcts_tree = MCTSTree::new(to_play_colour, game.clone());

        let number_of_simulations = 10;

        for _ in 0..number_of_simulations {
            let selected_node_id = mcts_tree.select(&MCTSTree::root_node_id());
            let selected_node = mcts_tree.get_node(&selected_node_id);

            if !selected_node.is_terminal_node() {
                let child_node_id = mcts_tree.expand(&selected_node_id);
                let payout = mcts_tree.simulate(child_node_id);
                mcts_tree.backpropagate(child_node_id, payout);
            } else {
                let payout = mcts_tree.simulate(selected_node_id);
                mcts_tree.backpropagate(selected_node_id, payout);
            }
        }

        Ok(mcts_tree.get_best_move())
    }
}

impl MonteCarloTreeSearch {
    pub fn new() -> Self {
        Self {}
    }
}

struct MCTSTree {
    to_play_colour: chess_set::Colour,
    nodes: HashMap<u32, MCTSNode>,
    max_node_id: u32,
}

#[derive(Clone)]
struct MCTSNode {
    id: u32,
    parent_node_id: Option<u32>,
    game_state_at_node: game::Game,
    child_nodes: HashMap<u32, rulebook::Move>,
    unvisited_legal_moves: Vec<rulebook::Move>,
    // Stats.
    average_evaluation: f32,
    number_of_visits: f32,
}

impl MCTSTree {
    // Factories.
    fn new(to_play_colour: chess_set::Colour, game: game::Game) -> Self {
        let mut nodes = HashMap::new();

        let root_node_id = MCTSTree::root_node_id();
        let root_node = MCTSNode::new(root_node_id, None, game);
        nodes.insert(root_node_id, root_node);

        Self {
            to_play_colour,
            nodes,
            max_node_id: root_node_id + 1,
        }
    }

    // MCTS algorithm.

    /// Select the next node to expand using the UCB formula.
    fn select(&self, parent_node_id: &u32) -> u32 {
        let parent_node = self.get_node(parent_node_id);

        if parent_node.is_leaf_node() || parent_node.is_terminal_node() {
            return parent_node.id;
        }

        let mut child_node_id_with_max_uct = None;
        let mut max_uct = f32::MIN;

        for child_node_id in parent_node.child_nodes.keys() {
            let child_node = self.get_node(&child_node_id);
            let uct_child_value = child_node.upper_confidence_bound(&parent_node);

            if uct_child_value >= max_uct {
                child_node_id_with_max_uct = Some(child_node.id);
                max_uct = uct_child_value
            }
        }
        match child_node_id_with_max_uct {
            Some(child_node_id) => self.select(&child_node_id),
            None => panic!("No child nodes found!"),
        }
    }

    /// Expand from a selected node one level deeper.
    fn expand(&mut self, parent_node_id: &u32) -> u32 {
        let mut parent_node = self.get_node(parent_node_id).clone();
        let legal_move = parent_node.pop_unvisited_legal_move();

        let mut deeper_game = parent_node.game_state_at_node.clone();
        deeper_game.play_validated_move(&legal_move).unwrap();

        let child_node_id = self.create_node(parent_node_id, deeper_game);
        parent_node.add_child(child_node_id, legal_move);
        self.update_node(parent_node); // Since we had to clone the parent node.

        child_node_id
    }

    /// Play random moves from a given node until reaching a conclusion.
    fn simulate(&mut self, from_node_id: u32) -> f32 {
        let engine = random::Random::new();
        let mut simulated_game = self.get_node(&from_node_id).game_state_at_node.clone();

        while simulated_game.get_status().to_play_colour().is_some() {
            let suggested_move = engine.generate_next_move(&simulated_game).unwrap();
            simulated_game.play_validated_move(&suggested_move).unwrap();
        }

        let outcome = simulated_game.get_status();
        self.evaluate_simulation_outcome(outcome)
    }

    /// Backpropagate the result from the simulation to all parent_node nodes.
    fn backpropagate(&mut self, from_node_id: u32, payout: f32) {
        let mut node = self.get_node(&from_node_id).clone();
        let parent_node_id = node.parent_node_id.clone();
        node.update_evaluation(payout);
        self.update_node(node);

        match parent_node_id {
            Some(node_id) => self.backpropagate(node_id, payout),
            None => {} // We've reached the root node.
        }
    }

    // Mutators.
    fn create_node(&mut self, parent_node_id: &u32, game_state_at_node: game::Game) -> u32 {
        let child_node_id = self.get_next_node_id();
        let node = MCTSNode::new(
            child_node_id,
            Some(parent_node_id.clone()),
            game_state_at_node,
        );
        self.nodes.insert(child_node_id, node);

        child_node_id
    }

    fn update_node(&mut self, node: MCTSNode) {
        self.nodes.insert(node.id, node);
    }

    fn get_next_node_id(&mut self) -> u32 {
        self.max_node_id += 1;
        self.max_node_id
    }

    // Queries.
    fn get_best_move(&self) -> rulebook::Move {
        let root_node = self.get_node(&MCTSTree::root_node_id());

        let highest_evaluation = f32::MIN;
        let mut best_move = None;

        for (child_node_id, legal_move) in root_node.child_nodes.clone().into_iter() {
            let child_node = self.get_node(&child_node_id);
            if child_node.average_evaluation > highest_evaluation {
                best_move = Some(legal_move);
            }
        }

        match best_move {
            Some(chess_move) => chess_move,
            None => panic!("MCTS failed to generate a move!"),
        }
    }

    fn evaluate_simulation_outcome(&self, outcome: &GameStatus) -> f32 {
        if outcome.is_draw() {
            return 0.;
        } else if let Some(winner) = outcome.winner() {
            return match winner == self.to_play_colour {
                true => 1.,
                false => -1.,
            };
        }

        panic!("Simulated game has no outcome!")
    }

    fn get_node(&self, id: &u32) -> &MCTSNode {
        self.nodes.get(id).unwrap()
    }

    fn root_node_id() -> u32 {
        0
    }
}

impl MCTSNode {
    // Factories.

    fn new(id: u32, parent_node_id: Option<u32>, game_state_at_node: game::Game) -> Self {
        let legal_moves = match game_state_at_node.get_status().to_play_colour() {
            Some(colour) => {
                rulebook::get_legal_moves(colour, game_state_at_node.get_chessboard_history())
            }
            None => vec![],
        };

        Self {
            id,
            game_state_at_node,
            parent_node_id,
            child_nodes: HashMap::new(),
            unvisited_legal_moves: legal_moves,
            average_evaluation: 0.0,
            number_of_visits: 0.0,
        }
    }

    // Mutators.
    fn update_evaluation(&mut self, payout: f32) {
        self.average_evaluation = (self.average_evaluation * self.number_of_visits + payout)
            / (self.number_of_visits + 1f32);
        self.number_of_visits += 1f32;
    }

    fn add_child(&mut self, child_node_id: u32, legal_move: rulebook::Move) {
        self.child_nodes.insert(child_node_id, legal_move);
    }

    fn pop_unvisited_legal_move(&mut self) -> rulebook::Move {
        self.unvisited_legal_moves.pop().unwrap()
    }

    // Queries.

    fn upper_confidence_bound(&self, parent_node: &Self) -> f32 {
        if self.number_of_visits == 0f32 {
            return f32::MIN;
        }

        self.average_evaluation
            + UPPER_CONFIDENCE_BOUND_BIAS
                * (parent_node.number_of_visits.ln() / self.number_of_visits).sqrt()
    }

    /// A leaf node has one or more child nodes from which no simulation has yet been initiated.
    fn is_leaf_node(&self) -> bool {
        self.unvisited_legal_moves.len() > 0
    }

    /// A terminal node has no visited or unvisited child nodes.
    fn is_terminal_node(&self) -> bool {
        self.unvisited_legal_moves.len() == 0 && self.child_nodes.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{ChessEngine, MonteCarloTreeSearch};
    use crate::domain::chess_set::{Chessboard, Colour, File, Piece, PieceType, Rank, Square};
    use crate::domain::game::{Game, GameStatus};
    use std::collections::BTreeMap;

    #[test] // Smokey.
    fn generates_move_for_black() {
        let mut starting_position = BTreeMap::new();

        // Set up a back rank mate on white.
        let white_king = Piece::new(Colour::White, PieceType::King);
        let white_king_square = Square::new(Rank::One, File::A);
        starting_position.insert(white_king_square, white_king);

        let black_king = Piece::new(Colour::Black, PieceType::King);
        let black_king_square = Square::new(Rank::Eight, File::A);
        starting_position.insert(black_king_square, black_king);

        let black_rook = Piece::new(Colour::Black, PieceType::Rook);
        let black_rook_square = Square::new(Rank::Eight, File::F);
        starting_position.insert(black_rook_square, black_rook);

        let chessboard = Chessboard::new(starting_position);
        let game = Game::reincarnate(1, GameStatus::ToPlayBlack, vec![chessboard]);

        let mcts_engine = MonteCarloTreeSearch::new();

        let black_move = mcts_engine.generate_next_move(&game);

        assert!(black_move.is_ok());
    }
}
