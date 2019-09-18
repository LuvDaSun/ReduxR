pub type Middleware<Action> = fn(action: &Action) -> ();
