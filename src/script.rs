use crate::contexts::*;
use crate::modes::*;

pub async fn script_main(mut sctx: ScriptContext) {
    sctx.push_walk_around_mode();
    let WalkAroundEvent::DebugQuit = sctx.update_walk_around_mode().await;
    sctx.pop_mode();
}