pub trait GameMdp {
    type Action;
    type State;

    fn proceed_game(
        state: &Self::State,
        action: &Self::Action,
    ) -> (f64, Self::State) where Self: Sized;
}
