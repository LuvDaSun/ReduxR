/// Reduce trait that enables reducing self via an action. It is possibly to implement this trait
/// for a different Action types. Action is usually an enum.
/// ```
/// extern crate reduxr;
///
/// use reduxr::*;
///
/// struct LampState {
///     power: bool,
/// }
///
/// enum LampOnOffAction {
///     TurnOn,
///     TurnOff,
/// }
///
/// impl Reduce<LampOnOffAction> for LampState {
///     fn reduce(self, action: &LampOnOffAction) -> Self {
///         match action {
///             LampOnOffAction::TurnOn => LampState { power: true },
///             LampOnOffAction::TurnOff => LampState { power: false },
///         }
///     }
/// }
///
/// enum LampSwitchAction {
///     Switch,
/// }
///
/// impl Reduce<LampSwitchAction> for LampState {
///     fn reduce(self, action: &LampSwitchAction) -> Self {
///         match action {
///             LampSwitchAction::Switch => LampState { power: !self.power },
///         }
///     }
/// }
///
/// fn main() {
///     let state = LampState { power: false };
///     assert_eq!(state.power, false);
///
///     let state = state.reduce(&LampOnOffAction::TurnOn);
///     assert_eq!(state.power, true);
///
///     let state = state.reduce(&LampOnOffAction::TurnOff);
///     assert_eq!(state.power, false);
///
///     let state = state.reduce(&LampSwitchAction::Switch);
///     assert_eq!(state.power, true);
///
///     let state = state.reduce(&LampSwitchAction::Switch);
///     assert_eq!(state.power, false);
/// }
/// ```
pub trait Reduce<Action> {
    fn reduce(self, action: &Action) -> Self;
}
