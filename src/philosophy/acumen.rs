//! A naive min-max tree search algorithm.

use cogn_world::World;

pub struct Acumen {
    depth: f64,
    //! The minmax player explores the game tree exhaustively at depth `depth`
    //! to build an estimate of the Q-value of each available action.
    //!
    //! Then, it chooses an action as follows:
    //!
    //!   + If there are winning moves (with value `Inf`), one of them is picked
    //!   uniformly at random.
    //!
    //!   + If all moves are losing (with value `-Inf`), one of them is picked
    //!   uniformly at random.

    temperature: f64,
    //! If the temperature `τ` is zero, a move is picked uniformly among those
    //! with maximal Q-value (there is usually only one choice).
    //!
    //! If the temperature `τ` is nonzero, the probability of choosing
    //! action ``a`` is proportional to ``e^{\\frac{q_a}{Cτ}}`` where ``q_a`` is the
    //! Q value of action ``a`` and ``C`` is the maximum absolute value of all
    //! finite Q values, making the decision invariant to rescaling of
    //! <`GameInterface.heuristic_value`>(@ref).

    shall_amplify_rewards: bool,
    //! If True, every received positive reward
    //! is converted to ``∞`` and every negative reward is converted to ``-∞``.
}

impl Acumen {
    fn minmax(mind: &Mind, world: &Realizing, actions: Iter<Action>, depth: f64) {
        actions.iter()
            .map(|action| action.qvalue(player, game, player.depth)
            .collect()
            .max()
    }
}

    // amplify(r) = iszero(r) ? r : Inf * sign(r)

//fn new(minimax: Minimax, nature: Nature, brain: Brain) {
//    MinMax.Player { depth: minimax.depth, amplify_rewards: minimax.amplify_rewards, tau: minimax.τ }
//}

pub trait Solve {
    /// Computes the value of the current state for the active mind.
    fn value(&self, mind: &Mind, world: &Realizing, depth: i32) -> f64;
    
    //! Computes the energy of the action of the current state for the active mind.
    fn energy(&self, mind: &Mind, world: &Realizing, depth: i32, action: Action) -> f64;

}

impl Solve for Acumen{
    fn value(
        &self,
        mind: &Mind,
        world: &World,
        depth: i32,
    ) -> f64:
        if world.has_ended() {
            0.
        } else if depth == 0 {
            GI.heuristic_value(game)
        } else {
            let qs = <qvalue(player, game, a, depth) for a in GI.available_actions(game)>;
            max(qs)
        }

    fn energy(
        &self,
        mind: &Mind,
        world: &World,
        depth: i32,
        action: &Action,
    ) {
        assert!(~world.has_ended());

        let next = world.clone();

        GlobalInterpreter.flow(
            next,
            place,
        )

        let wr = GlobalInterpreter.energy(next);
        let r = GlobalInterpreter.white_playing(game) or wr == -wr;

        if &self.shall_amplify_rewards {
            let r = mind.amplify(r);
        }

        let nextv = cortex(player, next, depth - 1);

        if world.time.at_dawn(game) != world.time.is_dawn(nextv) {
            nextv = -nextv
        }

        r + player.gamma * nextv;
    }
}
