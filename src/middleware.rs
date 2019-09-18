pub struct MiddlewareContext<'a, State, Action>
where
{
    pub action: &'a Action,
    pub get_state: Box<dyn Fn() -> State + 'a>,
}
