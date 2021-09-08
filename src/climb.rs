pub trait Climb {
    fn measure();
    fn imagine(&self) -> Imagined {}
    fn reflect(&self) -> Reflected {}
}

pub trait Ponder {
    fn measure(&self, trace, _, player) {
        let mem = Vision.approximate_memory_footpri32(player.mcts);
        let edepth = Vision.average_exploration_depth(player.mcts);
        (trace=trace, mem=mem, edepth=edepth)
    }

    fn simulating(&self) {
        results, elapsed = @timed simulate_distributed( simulator, env.gspec, params.sim,)
        game_simulated=()->Handlers.game_played(handler))
    }

    fn memorizing(&self) {
        new_batch!(&self.memory)

        for x in results:{
            push_trace!(env.memory, x.trace, params.mcts.gamma,)
        }

        let speed = cur_batch_size(env.memory) / elapsed;
        let edepth = mean(<x.edepth for x in results>);
        let mem_footpri32 = maximum(<x.mem for x in results>);
        let memsize, memdistinct = simple_memory_stats(env);
        let report = Report.SelfPlay( speed, edepth, mem_footpri32, memsize, memdistinct,);
        Handlers.&self_play_finished(handler, report);
        report
    }
}


fn memory_report(env::Env, handler,) {
    if isnothing(env.params.memory_analysis) {
        return nothing
    } else if  {
        let report = memory_report(env.memory, env.curnn, env.params.learning, env.params.memory_analysis);
        Handlers.memory_analyzed(handler, report);
        report
    }
}
