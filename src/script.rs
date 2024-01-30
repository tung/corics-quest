use crate::actor::*;
use crate::contexts::*;
use crate::enemy::*;
use crate::levels::TILE_SIZE;
use crate::modes::*;
use crate::progress::*;

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
            WalkAroundEvent::Encounter => {
                sctx.actors[0].visible = false;
                if let Some(enemy) = sctx.level.encounters.map(EncounterGroup::random_enemy) {
                    sctx.push_battle_mode(enemy);
                } else {
                    sctx.push_battle_mode(Enemy {
                        name: "Debug Rat",
                        sprite_path: "rat.png",
                        hp: 52,
                        attack: 5,
                        defense: 5,
                        weakness: Some(Magic::FireEdge),
                    });
                }
                match sctx.update_battle_mode().await {
                    BattleEvent::RanAway | BattleEvent::Victory => {
                        sctx.pop_mode();
                        sctx.actors[0].visible = true;
                    }
                    BattleEvent::Defeat => {
                        sctx.pop_mode();
                        sctx.actors[0].visible = true;
                        sctx.push_text_box_mode("Coric:\nOuch!");
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                        sctx.progress.hp = sctx.progress.max_hp;
                    }
                }
            }
            WalkAroundEvent::MainMenu => {
                sctx.push_main_menu_mode();
                let MainMenuEvent::Done = sctx.update_main_menu_mode().await;
                sctx.pop_mode();
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
            WalkAroundEvent::TouchLevelEdge(dir) => {
                if let Some((level, mut actors)) = sctx.level_by_neighbour(dir) {
                    // walk out of old level
                    walk_player(&mut sctx.actors[..], dir).await;

                    // move player to the new level
                    sctx.actors.truncate(1);
                    let mut player = sctx.actors.pop().expect("player actor");
                    player.grid_x +=
                        (sctx.level.px_world_x - level.px_world_x) / TILE_SIZE - dir.dx();
                    player.grid_y +=
                        (sctx.level.px_world_y - level.px_world_y) / TILE_SIZE - dir.dy();
                    actors.insert(0, player);
                    *sctx.actors = actors;
                    *sctx.level = level;

                    // walk into the new level
                    walk_player(&mut sctx.actors[..], dir).await;
                } else {
                    sctx.actors[0].stop_walk_animation();
                }
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
