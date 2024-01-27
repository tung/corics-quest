use crate::contexts::*;
use crate::modes::*;

pub async fn script_main(mut sctx: ScriptContext) {
    sctx.push_walk_around_mode();
    let WalkAroundEvent::DebugQuit = sctx.update_walk_around_mode().await;
    sctx.push_text_box_mode("Thanks for playing!\nSee you next time!");
    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
    sctx.pop_mode();
    sctx.pop_mode();
}
