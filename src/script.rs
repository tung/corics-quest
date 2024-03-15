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
    level_name: &'static str,
    on_enter: Option<ScriptCallback>,
    on_talk: &'static [(ActorType, ScriptCallback)],
}

static LEVEL_SCRIPTS: &[LevelScripts] = &[
    LevelScripts {
        level_name: "Start",
        on_enter: None,
        on_talk: &[
            (ActorType::Bed, |sctx| {
                Box::pin(async {
                    sctx.fade_out(60).await;
                    sctx.fade_in(60).await;
                    sctx.progress.hp = sctx.progress.max_hp;
                    sctx.progress.mp = sctx.progress.max_mp;
                    sctx.push_text_box_mode("HP and MP recovered!");
                    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                    sctx.pop_mode();
                })
            }),
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
                    if sctx.progress.maybe_upgrade_armor("Leather Armor", 1) {
                        sctx.push_text_box_mode("Julis:\nHere's your Leather Armor.");
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
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
                    if sctx.progress.maybe_upgrade_weapon("Short Sword", 1) {
                        sctx.push_text_box_mode("Matero:\nHere's your Short Sword.");
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                })
            }),
        ],
    },
    LevelScripts {
        level_name: "Earth_4",
        on_enter: Some(|sctx| {
            Box::pin(async {
                if sctx.progress.earth_defeated {
                    let earth = sctx
                        .actors
                        .iter()
                        .position(|a| a.identifier == ActorType::Earth)
                        .expect("Earth actor");
                    sctx.actors.remove(earth);
                }
            })
        }),
        on_talk: &[(ActorType::Earth, |sctx| {
            Box::pin(async {
                sctx.push_text_box_mode("Earth:\nI am the Earth Spirit.\nPrepare to die!");
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.push_battle_mode(
                    Enemy {
                        name: "Earth",
                        sprite_path: "earth.png",
                        hp: 250,
                        attack: 5,
                        defense: 5,
                        weakness: None,
                        exp: 50,
                    },
                    true,
                );

                let earth = sctx
                    .actors
                    .iter()
                    .position(|a| a.identifier == ActorType::Earth)
                    .expect("Earth actor");
                sctx.actors[earth].visible = false;

                if !handle_battle(sctx).await {
                    return;
                }

                sctx.actors[earth].visible = true;

                sctx.push_text_box_mode(
                    "Earth:\nI was... possessed... please...\nsave... the others...",
                );
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.actors.remove(earth);
                sctx.progress.earth_defeated = true;
            })
        })],
    },
    LevelScripts {
        level_name: "Water_4",
        on_enter: Some(|sctx| {
            Box::pin(async {
                if sctx.progress.water_defeated {
                    let water = sctx
                        .actors
                        .iter()
                        .position(|a| a.identifier == ActorType::Water)
                        .expect("Water actor");
                    sctx.actors.remove(water);
                }
            })
        }),
        on_talk: &[(ActorType::Water, |sctx| {
            Box::pin(async {
                sctx.push_text_box_mode("Water:\nI am the Water Spirit.\nPrepare to die!");
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.push_battle_mode(
                    Enemy {
                        name: "Water",
                        sprite_path: "water.png",
                        hp: 250,
                        attack: 5,
                        defense: 5,
                        weakness: None,
                        exp: 50,
                    },
                    true,
                );

                let water = sctx
                    .actors
                    .iter()
                    .position(|a| a.identifier == ActorType::Water)
                    .expect("Water actor");
                sctx.actors[water].visible = false;

                if !handle_battle(sctx).await {
                    return;
                }

                sctx.actors[water].visible = true;

                sctx.push_text_box_mode(
                    "Water:\nI was... possessed... please...\nsave... the others...",
                );
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.actors.remove(water);
                sctx.progress.water_defeated = true;
            })
        })],
    },
    LevelScripts {
        level_name: "Fire_4",
        on_enter: Some(|sctx| {
            Box::pin(async {
                if sctx.progress.fire_defeated {
                    let fire = sctx
                        .actors
                        .iter()
                        .position(|a| a.identifier == ActorType::Fire)
                        .expect("Fire actor");
                    sctx.actors.remove(fire);
                }
            })
        }),
        on_talk: &[(ActorType::Fire, |sctx| {
            Box::pin(async {
                sctx.push_text_box_mode("Fire:\nI am the Fire Spirit.\nPrepare to die!");
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.push_battle_mode(
                    Enemy {
                        name: "Fire",
                        sprite_path: "fire.png",
                        hp: 250,
                        attack: 5,
                        defense: 5,
                        weakness: None,
                        exp: 50,
                    },
                    true,
                );

                let fire = sctx
                    .actors
                    .iter()
                    .position(|a| a.identifier == ActorType::Fire)
                    .expect("Fire actor");
                sctx.actors[fire].visible = false;

                if !handle_battle(sctx).await {
                    return;
                }

                sctx.actors[fire].visible = true;

                sctx.push_text_box_mode("Fire:\nI was... possessed...\nThank you... Coric...");
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.actors.remove(fire);
                sctx.progress.fire_defeated = true;
            })
        })],
    },
];

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
            WalkAroundEvent::DebugLevelUp => {
                sctx.progress.level += 1;
                sctx.progress.exp = 0;
                sctx.progress.next_exp = sctx.progress.next_exp * 3 / 2;
                sctx.progress.max_hp += 30;
                sctx.progress.hp += 30;
                sctx.progress.max_mp += 2;
                sctx.progress.mp += 2;
                sctx.progress.attack += 2;
                sctx.progress.defense += 2;
                sctx.push_text_box_mode(&format!("Coric is now level {}!", sctx.progress.level));
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();
            }
            WalkAroundEvent::DebugSteps(steps) => {
                sctx.push_text_box_mode(&format!("Coric has taken {steps} steps."));
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();
            }
            WalkAroundEvent::Encounter => {
                if let Some(enemy) = sctx.level.encounters.map(EncounterGroup::random_enemy) {
                    sctx.push_battle_mode(enemy, false);
                } else {
                    sctx.push_battle_mode(
                        Enemy {
                            name: "Debug Rat",
                            sprite_path: "rat.png",
                            hp: 52,
                            attack: 5,
                            defense: 5,
                            weakness: Some(Magic::FireEdge),
                            exp: 5,
                        },
                        false,
                    );
                }
                handle_battle(&mut sctx).await;
            }
            WalkAroundEvent::MainMenu => {
                sctx.push_main_menu_mode();
                let MainMenuEvent::Done = sctx.update_main_menu_mode().await;
                sctx.pop_mode();
            }
            WalkAroundEvent::TalkActor(actor) => {
                if let Some((_, talk_script)) = LEVEL_SCRIPTS
                    .iter()
                    .find(|l| l.level_name == sctx.level.identifier)
                    .and_then(|l| {
                        l.on_talk
                            .iter()
                            .find(|(ty, _)| *ty == sctx.actors[actor].identifier)
                    })
                {
                    (talk_script)(&mut sctx).await;
                } else if sctx.actors[actor].identifier == ActorType::Chest {
                    if !sctx
                        .progress
                        .collected_chests
                        .iter()
                        .map(String::as_str)
                        .any(|s| s == sctx.level.identifier.as_str())
                    {
                        let chest_opened =
                            match sctx.actors[actor].chest_type.expect("ChestType for Chest") {
                                ChestType::FireEdge => {
                                    sctx.actors[actor].start_animation("open");
                                    learn_magic(&mut sctx, Magic::FireEdge).await;
                                    true
                                }

                                ChestType::EarthEdge => {
                                    sctx.actors[actor].start_animation("open");
                                    learn_magic(&mut sctx, Magic::EarthEdge).await;
                                    true
                                }

                                ChestType::WaterEdge => {
                                    sctx.actors[actor].start_animation("open");
                                    learn_magic(&mut sctx, Magic::WaterEdge).await;
                                    true
                                }

                                ChestType::LongSword => {
                                    chest_with_weapon(&mut sctx, actor, "Long Sword", 2).await
                                }

                                ChestType::ChainVest => {
                                    chest_with_armor(&mut sctx, actor, "Chain Vest", 2).await
                                }

                                ChestType::DuelistSword => {
                                    chest_with_weapon(&mut sctx, actor, "Duelist Sword", 3).await
                                }

                                ChestType::SteelArmor => {
                                    chest_with_armor(&mut sctx, actor, "Steel Armor", 3).await
                                }

                                ChestType::ValorBlade => {
                                    chest_with_weapon(&mut sctx, actor, "Valor Blade", 4).await
                                }

                                ChestType::MythicPlate => {
                                    chest_with_armor(&mut sctx, actor, "Mythic Plate", 4).await
                                }
                            };

                        if chest_opened {
                            sctx.progress
                                .collected_chests
                                .push(sctx.level.identifier.clone());
                        }
                    }
                } else if sctx.actors[actor].identifier == ActorType::Lever {
                    if sctx.lever_is_turned() {
                        sctx.push_text_box_mode("Coric turns the lever to the left.");
                    } else {
                        sctx.push_text_box_mode("Coric turns the lever to the right.");
                    }
                    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                    sctx.pop_mode();

                    sctx.toggle_lever();
                } else {
                    panic!(
                        "missing on_talk script for {:?} in level {}",
                        sctx.actors[actor].identifier, sctx.level.identifier,
                    );
                }
            }
            WalkAroundEvent::TouchLevelEdge(dir) => {
                if let Some((level, mut actors)) = sctx.level_by_neighbour(dir) {
                    // prepare black fade color
                    *sctx.fade = [0.0; 4];

                    // walk out of old level
                    walk_player(&mut sctx.actors[..], dir, Some((&mut sctx.fade[3], 1.0))).await;

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

                    run_level_on_enter(&mut sctx).await;

                    // walk into the new level
                    walk_player(&mut sctx.actors[..], dir, Some((&mut sctx.fade[3], 0.0))).await;
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
        if !sctx.res.levels.contains_identifier(l.level_name) {
            panic!("LEVEL_SCRIPTS: unknown level identifier: {}", l.level_name);
        }

        if !level_identifiers.contains(l.level_name) {
            level_identifiers.insert(l.level_name.to_string());
        } else {
            panic!(
                "LEVEL_SCRIPTS: duplicate level identifier: {}",
                l.level_name
            );
        }

        let mut actor_types: HashSet<ActorType> = HashSet::new();
        for t in l.on_talk {
            if !actor_types.contains(&t.0) {
                actor_types.insert(t.0);
            } else {
                panic!("on_talk: duplicate ActorType: {:?}", t.0);
            }
        }
    }
}

async fn run_level_on_enter(sctx: &mut ScriptContext) {
    if let Some(on_enter) = LEVEL_SCRIPTS
        .iter()
        .find(|l| l.level_name == sctx.level.identifier)
        .and_then(|l| l.on_enter)
    {
        (on_enter)(sctx).await;
    }
}

async fn handle_battle(sctx: &mut ScriptContext) -> bool {
    sctx.actors[0].visible = false;
    let event = sctx.update_battle_mode().await;
    sctx.pop_mode();
    sctx.actors[0].visible = true;
    match event {
        BattleEvent::Victory => true,
        BattleEvent::RanAway => false,
        BattleEvent::Defeat => {
            sctx.fade_out(90).await;

            // warp player back to town
            let (level, mut actors) = sctx.level_by_identifier("Start");
            sctx.actors.truncate(1);
            let mut player = sctx.actors.pop().expect("player actor");
            player.grid_x = 6;
            player.grid_y = 3;
            player.start_animation("face_s");
            actors.insert(0, player);
            *sctx.level = level;
            *sctx.actors = actors;

            run_level_on_enter(sctx).await;

            sctx.progress.hp = sctx.progress.max_hp;
            sctx.fade_in(90).await;

            sctx.push_text_box_mode("Coric:\nOuch!");
            let TextBoxEvent::Done = sctx.update_text_box_mode().await;
            sctx.pop_mode();

            false
        }
    }
}

async fn chest_with_armor(
    sctx: &mut ScriptContext,
    chest: usize,
    name: &'static str,
    defense: i32,
) -> bool {
    sctx.actors[chest].start_animation("open");
    if sctx.progress.maybe_upgrade_armor(name, defense) {
        sctx.push_text_box_mode(&format!("Coric found the {name}!"));
        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
        sctx.pop_mode();
        true
    } else {
        sctx.push_text_box_mode(&format!("Coric found the {name}, but\ndoesn't need it."));
        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
        sctx.pop_mode();
        sctx.actors[chest].start_animation("closed");
        false
    }
}

async fn chest_with_weapon(
    sctx: &mut ScriptContext,
    chest: usize,
    name: &'static str,
    attack: i32,
) -> bool {
    sctx.actors[chest].start_animation("open");
    if sctx.progress.maybe_upgrade_weapon(name, attack) {
        sctx.push_text_box_mode(&format!("Coric found the {name}!"));
        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
        sctx.pop_mode();
        true
    } else {
        sctx.push_text_box_mode(&format!("Coric found the {name}, but\ndoesn't need it."));
        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
        sctx.pop_mode();
        sctx.actors[chest].start_animation("closed");
        false
    }
}

async fn learn_magic(sctx: &mut ScriptContext, magic: Magic) {
    let magic_slot = sctx
        .progress
        .magic
        .iter_mut()
        .find(|m| m.magic == magic)
        .expect("magic slot");
    magic_slot.known = true;

    sctx.push_text_box_mode(&format!("Coric learned {}!", magic.name()));
    let TextBoxEvent::Done = sctx.update_text_box_mode().await;
    sctx.pop_mode();
}
