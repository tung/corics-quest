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
                    let (salves, xsalves, tonics) = if !sctx.progress.earth_defeated {
                        (1, 0, 1)
                    } else if !sctx.progress.water_defeated {
                        (2, 0, 2)
                    } else {
                        (1, 1, 2)
                    };
                    let salves_given = sctx.progress.maybe_give_items(Item::Salve, salves);
                    let xsalves_given = sctx.progress.maybe_give_items(Item::XSalve, xsalves);
                    let tonics_given = sctx.progress.maybe_give_items(Item::Tonic, tonics);
                    if salves_given + xsalves_given + tonics_given > 0 {
                        sctx.push_text_box_mode(
                            "Ducille:\n\
                             You need items, Coric?\n\
                             Let's see what I can find...",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                    if salves_given > 0 {
                        if salves_given == 1 {
                            sctx.push_text_box_mode("Coric got a Salve!");
                        } else {
                            sctx.push_text_box_mode(&format!("Coric got {salves_given} Salves!"));
                        }
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                    if xsalves_given > 0 {
                        if xsalves_given == 1 {
                            sctx.push_text_box_mode("Coric got an XSalve!");
                        } else {
                            sctx.push_text_box_mode(&format!("Coric got {xsalves_given} XSalves!"));
                        }
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                    if tonics_given > 0 {
                        if tonics_given == 1 {
                            sctx.push_text_box_mode("Coric got a Tonic!");
                        } else {
                            sctx.push_text_box_mode(&format!("Coric got {tonics_given} Tonics!"));
                        }
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }

                    if !sctx.progress.earth_defeated {
                        sctx.push_text_box_mode(
                            "Ducille:\n\
                             You can rest in your bed\n\
                             to recover and save.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.water_defeated {
                        sctx.push_text_box_mode(
                            "Ducille:\n\
                             If you fall in battle,\n\
                             we'll bring you back home.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.fire_defeated {
                        sctx.push_text_box_mode(
                            "Ducille:\n\
                             The spirits were possessed, you say?\n\
                             I wonder how that came to be...",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                })
            }),
            (ActorType::Jace, |sctx| {
                Box::pin(async {
                    if !sctx.progress.earth_defeated {
                        sctx.push_text_box_mode(
                            "Jace:\n\
                             The spirits reside in three castles.\n\
                             The Earth Castle lies to the east.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.water_defeated {
                        sctx.push_text_box_mode(
                            "Jace:\n\
                             Head to the Water Castle, across\n\
                             the lakes and forests to the west.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.fire_defeated {
                        sctx.push_text_box_mode(
                            "Jace:\n\
                             Across the chasms and cliffs\n\
                             to the north lies the Fire Castle.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                })
            }),
            (ActorType::Julis, |sctx| {
                Box::pin(async {
                    if !sctx.progress.earth_defeated {
                        sctx.push_text_box_mode(
                            "Julis:\n\
                             Talk to us when you make progress;\n\
                             we'll have more to tell you.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.water_defeated {
                        sctx.push_text_box_mode(
                            "Julis:\n\
                             Ducille tends to the apocathery;\n\
                             Jace knows the lay of the land.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.fire_defeated {
                        sctx.push_text_box_mode(
                            "Julis:\n\
                             We have records of vampires\n\
                             bested by water.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                })
            }),
            (ActorType::Matero, |sctx| {
                Box::pin(async {
                    let weapon_given = sctx.progress.maybe_upgrade_weapon("Short Sword", 2);
                    let armor_given = sctx.progress.maybe_upgrade_armor("Leather Armor", 2);
                    if weapon_given || armor_given {
                        sctx.push_text_box_mode(
                            "Matero:\n\
                             Going on a quest?\n\
                             I have some gear you can use.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                    if weapon_given {
                        sctx.push_text_box_mode("Coric got a Short Sword!");
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }
                    if armor_given {
                        sctx.push_text_box_mode("Coric got a Leather Armor!");
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    }

                    if !sctx.progress.earth_defeated {
                        sctx.push_text_box_mode(
                            "Matero:\n\
                             If you use magic on a foe, you can\n\
                             follow up next turn for more damage.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.water_defeated {
                        sctx.push_text_box_mode(
                            "Matero:\n\
                             Rumor has it that each castle has\n\
                             a weapon and an armor to find.",
                        );
                        let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                        sctx.pop_mode();
                    } else if !sctx.progress.fire_defeated {
                        sctx.push_text_box_mode(
                            "Matero:\n\
                             If spikes block your path, you can\n\
                             pull a lever to retract them.",
                        );
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
                sctx.push_text_box_mode(
                    "Earth:\nI will return you to the dust\nwhence you came, mortal!",
                );
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.push_battle_mode(
                    Enemy {
                        name: "Earth",
                        sprite_path: "earth.png",
                        hp: 1700,
                        attack: 27,
                        defense: 24,
                        weakness: Some(Magic::FireEdge),
                        exp: 500,
                        actions: &[
                            EnemyAction {
                                chance: 10,
                                msg: "hurls a massive boulder!",
                                damage_factor: Some(2.0),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "hurls a massive boulder!\nCoric deftly leaps aside!",
                                damage_factor: None,
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "slams its fist the ground!\nCoric is pummeled by debris!",
                                damage_factor: Some(1.5),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "throws debris at Coric!\nCoric deflects some of it!",
                                damage_factor: Some(0.5),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "roars with a bitter rage!",
                                damage_factor: None,
                            },
                        ],
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
                sctx.push_text_box_mode("Water:\nThe Water of Life claims all souls!");
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.push_battle_mode(
                    Enemy {
                        name: "Water",
                        sprite_path: "water.png",
                        hp: 5000,
                        attack: 49,
                        defense: 46,
                        weakness: Some(Magic::EarthEdge),
                        exp: 2000,
                        actions: &[
                            EnemyAction {
                                chance: 10,
                                msg: "throws columns of ice!\nOne of them hits Coric!",
                                damage_factor: Some(1.6),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "throws columns of ice!\nCoric narrowly dodges them!",
                                damage_factor: None,
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "summons a huge wave!\nCoric is slammed!",
                                damage_factor: Some(1.3),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "fires a torrent of water!",
                                damage_factor: Some(1.1),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "emits a hollow wail!",
                                damage_factor: None,
                            },
                        ],
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
                sctx.push_text_box_mode("Fire:\nYour flame of life will be\nextinguished here!");
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();

                sctx.push_battle_mode(
                    Enemy {
                        name: "Fire",
                        sprite_path: "fire.png",
                        hp: 10000,
                        attack: 77,
                        defense: 74,
                        weakness: Some(Magic::WaterEdge),
                        exp: 9000,
                        actions: &[
                            EnemyAction {
                                chance: 10,
                                msg: "summons roaring flames!\nCoric is roasted!",
                                damage_factor: Some(1.8),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "shoots infernal bolts!\nOne of them hits Coric!",
                                damage_factor: Some(1.5),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "shoots infernal bolts!\nCoric weaves between them!",
                                damage_factor: None,
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "whips up glowing embers!\nCoric is burned!",
                                damage_factor: Some(0.7),
                            },
                            EnemyAction {
                                chance: 10,
                                msg: "lets out a piercing cry!",
                                damage_factor: None,
                            },
                        ],
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
                sctx.progress.exp = 0;
                sctx.progress.gain_level();
                sctx.push_text_box_mode(&format!("Coric is now level {}!", sctx.progress.level));
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();
            }
            WalkAroundEvent::DebugSteps(steps) => {
                sctx.push_text_box_mode(&format!("Coric has taken {steps} steps."));
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();
            }
            WalkAroundEvent::DebugEquip(equipment_set) => {
                let weapon = match equipment_set {
                    1 => Weapon {
                        name: String::from("Short Sword"),
                        attack: 2,
                    },
                    2 => Weapon {
                        name: String::from("Long Sword"),
                        attack: 7,
                    },
                    3 => Weapon {
                        name: String::from("Duelist Sword"),
                        attack: 13,
                    },
                    4 => Weapon {
                        name: String::from("Valor Blade"),
                        attack: 25,
                    },
                    _ => unreachable!(),
                };
                let armor = match equipment_set {
                    1 => Armor {
                        name: String::from("Leather Armor"),
                        defense: 2,
                    },
                    2 => Armor {
                        name: String::from("Chain Vest"),
                        defense: 7,
                    },
                    3 => Armor {
                        name: String::from("Steel Armor"),
                        defense: 13,
                    },
                    4 => Armor {
                        name: String::from("Mythic Plate"),
                        defense: 25,
                    },
                    _ => unreachable!(),
                };
                sctx.progress.attack +=
                    weapon.attack - sctx.progress.weapon.as_ref().map(|w| w.attack).unwrap_or(0);
                sctx.progress.defense +=
                    armor.defense - sctx.progress.armor.as_ref().map(|a| a.defense).unwrap_or(0);
                sctx.progress.weapon = Some(weapon);
                sctx.progress.armor = Some(armor);

                sctx.push_text_box_mode(&format!("Coric uses equipment set {equipment_set}."));
                let TextBoxEvent::Done = sctx.update_text_box_mode().await;
                sctx.pop_mode();
            }
            WalkAroundEvent::DebugBattle(encounter_group) => {
                let group = match encounter_group {
                    1 => EncounterGroup::Wilderness1,
                    2 => EncounterGroup::EarthCastle,
                    3 => EncounterGroup::Wilderness2,
                    4 => EncounterGroup::WaterCastle,
                    5 => EncounterGroup::Wilderness3,
                    6 => EncounterGroup::FireCastle,
                    _ => unreachable!(),
                };
                let enemy = group.random_enemy(&mut sctx.rng);
                sctx.push_battle_mode(enemy, false);
                handle_battle(&mut sctx).await;
            }
            WalkAroundEvent::Encounter => {
                if let Some(enemy) = sctx.level.encounters.map(|g| g.random_enemy(&mut sctx.rng)) {
                    sctx.push_battle_mode(enemy, false);
                } else {
                    sctx.push_battle_mode(
                        Enemy {
                            name: "Debug Foe",
                            sprite_path: "fire.png",
                            hp: 10000,
                            attack: 77,
                            defense: 74,
                            weakness: Some(Magic::WaterEdge),
                            exp: 5,
                            actions: &[],
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
                                    chest_with_weapon(&mut sctx, actor, "Long Sword", 7).await
                                }

                                ChestType::ChainVest => {
                                    chest_with_armor(&mut sctx, actor, "Chain Vest", 7).await
                                }

                                ChestType::DuelistSword => {
                                    chest_with_weapon(&mut sctx, actor, "Duelist Sword", 13).await
                                }

                                ChestType::SteelArmor => {
                                    chest_with_armor(&mut sctx, actor, "Steel Armor", 13).await
                                }

                                ChestType::ValorBlade => {
                                    chest_with_weapon(&mut sctx, actor, "Valor Blade", 25).await
                                }

                                ChestType::MythicPlate => {
                                    chest_with_armor(&mut sctx, actor, "Mythic Plate", 25).await
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
            sctx.progress.mp = sctx.progress.max_mp;
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
