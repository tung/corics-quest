use crate::actor::*;
use crate::contexts::*;
use crate::modes::*;

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

type ScriptCallback = for<'a> fn(&'a mut ScriptContext) -> Pin<Box<dyn Future<Output = ()> + 'a>>;

struct LevelScripts {
    on_talk: &'static [(ActorType, ScriptCallback)],
}

static LEVEL_SCRIPTS: &[(&str, LevelScripts)] = &[(
    "Start",
    LevelScripts {
        on_talk: &[
            (ActorType::Ducille, |sctx| {
                Box::pin(async {
                    sctx.push_text_box_mode("Ducille:\nHi Coric!");
                    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                    sctx.pop_mode();
                })
            }),
            (ActorType::Jace, |sctx| {
                Box::pin(async {
                    sctx.push_text_box_mode("Jace:\nHey Coric.");
                    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                    sctx.pop_mode();
                })
            }),
            (ActorType::Julis, |sctx| {
                Box::pin(async {
                    sctx.push_text_box_mode("Julis:\nHi Coric!");
                    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                    sctx.pop_mode();
                })
            }),
            (ActorType::Matero, |sctx| {
                Box::pin(async {
                    sctx.push_text_box_mode("Matero:\nHey Coric.");
                    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                    sctx.pop_mode();
                    sctx.push_text_box_mode("Matero:\nNice cape!");
                    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                    sctx.pop_mode();
                })
            }),
        ],
    },
)];

pub async fn script_main(mut sctx: ScriptContext) {
    validate_level_scripts(&mut sctx);
    sctx.push_walk_around_mode();
    loop {
        match sctx.update_walk_around_mode().await {
            WalkAroundEvent::DebugQuit => {
                sctx.push_text_box_mode("Thanks for playing!\nSee you next time!");
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode(); // TextBox
                sctx.pop_mode(); // WalkAround
                return;
            }
            WalkAroundEvent::TalkActor(actor) => {
                let level_script = LEVEL_SCRIPTS
                    .iter()
                    .find(|(id, _)| *id == sctx.level.identifier)
                    .map(|(_, l)| l)
                    .unwrap();
                let talk_script = level_script
                    .on_talk
                    .iter()
                    .find(|(ty, _)| *ty == sctx.actors[actor].identifier)
                    .map(|(_, t)| t)
                    .unwrap();
                (talk_script)(&mut sctx).await;
            }
        }
    }
}

fn validate_level_scripts(sctx: &mut ScriptContext) {
    let mut level_identifiers: HashSet<String> = HashSet::new();
    for l in LEVEL_SCRIPTS {
        if !sctx.res.levels.contains_identifier(l.0) {
            panic!("LEVEL_SCRIPTS: unknown level identifier: {}", l.0);
        }

        if !level_identifiers.contains(l.0) {
            level_identifiers.insert(l.0.to_string());
        } else {
            panic!("LEVEL_SCRIPTS: duplicate level identifier: {}", l.0);
        }

        let mut actor_types: HashSet<ActorType> = HashSet::new();
        for t in l.1.on_talk {
            if !actor_types.contains(&t.0) {
                actor_types.insert(t.0);
            } else {
                panic!("on_talk: duplicate ActorType: {:?}", t.0);
            }
        }
    }
}
