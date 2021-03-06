use super::{MessageMapper, Notification};

pub enum Effect<Ms, GMs> {
    Msg(Option<Ms>),
    GMsg(GMs),
    Notification(Notification),
    TriggeredHandler(Box<dyn FnOnce() -> Option<Ms>>),
}

impl<Ms: 'static, OtherMs: 'static, GMs> MessageMapper<Ms, OtherMs> for Effect<Ms, GMs> {
    type SelfWithOtherMs = Effect<OtherMs, GMs>;
    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Effect<OtherMs, GMs> {
        match self {
            Effect::Msg(msg) => Effect::Msg(msg.map(f)),
            Effect::GMsg(g_msg) => Effect::GMsg(g_msg),
            Effect::Notification(notification) => Effect::Notification(notification),
            Effect::TriggeredHandler(handler) => {
                Effect::TriggeredHandler(Box::new(move || handler().map(f)))
            }
        }
    }
}
