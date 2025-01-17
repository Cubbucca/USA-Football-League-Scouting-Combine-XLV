use crate::{
    asset_loading, assets::GameAssets, audio::GameAudio, cleanup, game_controller, game_state,
    menus, title_screen::MenuAction, ui::text_size, AppState,
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Options)
                .with_system(setup)
                .with_system(game_controller::clear_presses),
        )
        .insert_resource(CurrentOption(0))
        .insert_resource(OptionState::initialize())
        .add_event::<OptionChangeEvent>()
        .add_system_set(
            SystemSet::on_update(AppState::Options)
                .with_system(update_menu_buttons.after("handle_input"))
                .with_system(highlight_options)
                .with_system(handle_option_changes)
                .with_system(display_current_options)
                .with_system(
                    handle_controllers
                        .label("handle_input")
                        .after(game_controller::store_controller_inputs),
                ),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Options)
                .with_system(cleanup::<CleanupMarker>)
                .with_system(game_controller::clear_presses),
        );
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Component, Clone)]
struct OptionRow {
    row: usize,
}

struct CurrentOption(usize);

enum OptionChange {
    Increase,
    Decrease,
    Select,
}

#[derive(Default)]
pub struct OptionState {
    graphics: usize,
    shadows_on: usize,
    game_version: usize,
}

impl OptionState {
    pub fn initialize() -> Self {
        OptionState {
            graphics: 0,
            shadows_on: 0,
            game_version: 0,
        }
    }
}

#[derive(Component)]
struct OptionValueMarker;

struct OptionChangeEvent {
    action: OptionChange,
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut current_option: ResMut<CurrentOption>,
    text_scaler: text_size::TextScaler,
) {
    current_option.0 = 3;

    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(CleanupMarker);

    commands
        .spawn_bundle(InputManagerBundle {
            input_map: MenuAction::default_input_map(),
            action_state: ActionState::default(),
        })
        .insert(CleanupMarker);

    // Options Title text
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(98.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    add_title(
                        parent,
                        game_assets.font.clone(),
                        text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.2),
                        "Game Settings",
                        Vec::<CleanupMarker>::new(), // just an empty vec since can't do <impl Trait>
                    );
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                        position_type: PositionType::Relative,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(OptionRow { row: 0 })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            add_label(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::DEFAULT_FONT_SIZE),
                                "Graphics   :",
                                vec![OptionRow { row: 0 }],
                            );
                        });
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            add_option(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::SCORE_FONT_SIZE),
                                vec![OptionRow { row: 0 }],
                            );
                        });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                        position_type: PositionType::Relative,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(OptionRow { row: 1 })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            add_label(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::DEFAULT_FONT_SIZE),
                                "Shadows    :",
                                vec![OptionRow { row: 1 }],
                            );
                        });

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            add_option(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::SCORE_FONT_SIZE),
                                vec![OptionRow { row: 1 }],
                            );
                        });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                        position_type: PositionType::Relative,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(OptionRow { row: 2 })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            add_label(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::DEFAULT_FONT_SIZE),
                                "Version    :",
                                vec![OptionRow { row: 2 }],
                            );
                        });

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            add_option(
                                parent,
                                game_assets.font.clone(),
                                text_scaler.scale(menus::SCORE_FONT_SIZE),
                                vec![OptionRow { row: 2 }],
                            );
                        });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(40.0), Val::Percent(20.0)),
                        position_type: PositionType::Relative,
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Percent(20.0),
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .insert(OptionRow { row: 3 })
                .with_children(|parent| {
                    add_button(
                        parent,
                        game_assets.font.clone(),
                        text_scaler.scale(menus::SCORE_FONT_SIZE),
                        "Start Game",
                        vec![OptionRow { row: 3 }],
                    );
                });
        });
}

fn add_label(
    builder: &mut ChildBuilder<'_, '_, '_>,
    font: Handle<Font>,
    font_size: f32,
    label: &str,
    mut components: Vec<impl Component>,
) {
    let mut text_bundle = builder.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect {
                left: Val::Percent(2.0),
                top: Val::Auto,
                bottom: Val::Auto,
                ..Default::default()
            },
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        text: Text::from_section(
            label,
            TextStyle {
                font,
                font_size,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::default()),
        ..Default::default()
    });
    components.drain(..).for_each(|c| {
        text_bundle.insert(c);
    });
}

fn add_option(
    builder: &mut ChildBuilder<'_, '_, '_>,
    font: Handle<Font>,
    font_size: f32,
    mut components: Vec<impl Component + Clone>,
) {
    let mut text_bundle = builder.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..Default::default()
            },
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        text: Text::from_section(
            "<".to_string(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::default()),
        ..Default::default()
    });

    components.clone().drain(..).for_each(|c| {
        text_bundle.insert(c);
    });

    let mut text_bundle = builder.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..Default::default()
            },
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        text: Text::from_section(
            "".to_string(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::default()),
        ..Default::default()
    });

    components.clone().drain(..).for_each(|c| {
        text_bundle.insert(c);
    });
    text_bundle.insert(OptionValueMarker);

    let mut text_bundle = builder.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..Default::default()
            },
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        text: Text::from_section(
            ">".to_string(),
            TextStyle {
                font,
                font_size,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::default()),
        ..Default::default()
    });

    components.drain(..).for_each(|c| {
        text_bundle.insert(c);
    });
}

fn add_button(
    builder: &mut ChildBuilder<'_, '_, '_>,
    font: Handle<Font>,
    font_size: f32,
    label: &str,
    mut components: Vec<impl Component>,
) {
    let mut text_bundle = builder.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect::all(Val::Auto),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font,
                font_size,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        }),
        ..Default::default()
    });

    components.drain(..).for_each(|c| {
        text_bundle.insert(c);
    });
}

pub fn add_title(
    builder: &mut ChildBuilder<'_, '_, '_>,
    font: Handle<Font>,
    font_size: f32,
    title: &str,
    mut components: Vec<impl Component>,
) {
    let mut text_bundle = builder.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                ..Default::default()
            },
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        text: Text::from_section(
            title.to_string(),
            TextStyle {
                font,
                font_size,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment {
            horizontal: HorizontalAlign::Center,
            ..Default::default()
        }),
        ..Default::default()
    });

    components.drain(..).for_each(|c| {
        text_bundle.insert(c);
    });
}

fn highlight_options(
    current_option: Res<CurrentOption>,
    mut options: Query<(&OptionRow, Option<&mut UiColor>, Option<&mut Text>)>,
) {
    for (option, maybe_ui_color, maybe_text) in options.iter_mut() {
        if option.row == current_option.0 {
            if let Some(mut ui_color) = maybe_ui_color {
                *ui_color = UiColor(menus::HOVERED_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for mut text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::BLACK;
                }
            }
        } else {
            if let Some(mut ui_color) = maybe_ui_color {
                *ui_color = UiColor(menus::NORMAL_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::WHITE;
                }
            }
        }
    }
}

fn update_menu_buttons(
    mut current_option: ResMut<CurrentOption>,
    action_state: Query<&ActionState<MenuAction>>,
    game_assets: Res<GameAssets>,
    mut audio: GameAudio,
    mut option_change_event_writer: EventWriter<OptionChangeEvent>,
) {
    let action_state = action_state.single();
    let max_options = 3;

    if action_state.just_pressed(MenuAction::Up) {
        audio.play_sfx(&game_assets.blip);
        current_option.0 = current_option.0.checked_sub(1).unwrap_or(max_options);
    }
    if action_state.just_pressed(MenuAction::Down) {
        audio.play_sfx(&game_assets.blip);
        current_option.0 = if current_option.0 == max_options {
            0
        } else {
            current_option.0 + 1
        };
    }
    if action_state.just_pressed(MenuAction::Right) {
        option_change_event_writer.send(OptionChangeEvent {
            action: OptionChange::Increase,
        });
    }
    if action_state.just_pressed(MenuAction::Left) {
        option_change_event_writer.send(OptionChangeEvent {
            action: OptionChange::Decrease,
        });
    }
    if action_state.just_pressed(MenuAction::Select) {
        option_change_event_writer.send(OptionChangeEvent {
            action: OptionChange::Select,
        });
    }
}

fn handle_controllers(
    controllers: Res<game_controller::GameController>,
    mut players: Query<(Entity, &mut ActionState<MenuAction>)>,
) {
    for (_, mut action_state) in players.iter_mut() {
        for (_, just_pressed) in controllers.just_pressed.iter() {
            action_state.release(MenuAction::Up);
            action_state.release(MenuAction::Down);
            action_state.release(MenuAction::Left);
            action_state.release(MenuAction::Right);

            action_state.release(MenuAction::Select);

            if just_pressed.contains(&game_controller::GameButton::Up) {
                action_state.press(MenuAction::Up);
            }
            if just_pressed.contains(&game_controller::GameButton::Down) {
                action_state.press(MenuAction::Down);
            }
            if just_pressed.contains(&game_controller::GameButton::Left) {
                action_state.press(MenuAction::Left);
            }
            if just_pressed.contains(&game_controller::GameButton::Right) {
                action_state.press(MenuAction::Right);
            }
            if just_pressed.contains(&game_controller::GameButton::ActionDown)
                || just_pressed.contains(&game_controller::GameButton::Start)
            {
                action_state.press(MenuAction::Select);
            }
        }
    }
}

fn handle_option_changes(
    current_option: Res<CurrentOption>,
    mut option_change_event_reader: EventReader<OptionChangeEvent>,
    mut options: ResMut<OptionState>,
    mut game_assets: ResMut<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut audio: GameAudio,
) {
    for option_change in option_change_event_reader.iter() {
        match current_option.0 {
            0 => {
                let min = 0;
                let max = 2;
                match option_change.action {
                    OptionChange::Increase => {
                        options.graphics = if options.graphics == max {
                            min
                        } else {
                            options.graphics + 1
                        };
                        audio.play_sfx(&game_assets.blip);
                    }
                    OptionChange::Decrease => {
                        options.graphics = if options.graphics == min {
                            max
                        } else {
                            options.graphics - 1
                        };
                        audio.play_sfx(&game_assets.blip);
                    }
                    _ => (),
                };
            }
            1 => {
                let min = 0;
                let max = 1;
                match option_change.action {
                    OptionChange::Increase => {
                        options.shadows_on = if options.shadows_on == max { min } else { max };
                        audio.play_sfx(&game_assets.blip);
                    }
                    OptionChange::Decrease => {
                        options.shadows_on = if options.shadows_on == max { min } else { max };
                        audio.play_sfx(&game_assets.blip);
                    }
                    _ => (),
                };
            }
            2 => {
                let min = 0;
                let max = 1;
                match option_change.action {
                    OptionChange::Increase => {
                        options.game_version = if options.game_version == max {
                            min
                        } else {
                            max
                        };
                        audio.play_sfx(&game_assets.blip);
                    }
                    OptionChange::Decrease => {
                        options.game_version = if options.game_version == max {
                            min
                        } else {
                            max
                        };
                        audio.play_sfx(&game_assets.blip);
                    }
                    _ => (),
                };
            }
            3 => {
                if let OptionChange::Select = option_change.action {
                    *game_state = game_state::GameState::initialize(
                        options.graphics == 0,
                        options.shadows_on == 0,
                        options.game_version == 0,
                    );

                    audio.play_sfx(&game_assets.blip);
                    assets_handler.load(AppState::InGame, &mut game_assets, &mut game_state);
                }
            }
            _ => (),
        }
    }
}

fn display_current_options(
    option_state: ResMut<OptionState>,
    mut options: Query<(&mut Text, &OptionRow), With<OptionValueMarker>>,
) {
    for (mut option_text, option_row) in options.iter_mut() {
        if option_row.row == 0 {
            option_text.sections[0].value = match option_state.graphics {
                0 => " High ".to_string(),
                _ => " Low  ".to_string(),
            };
        }

        if option_row.row == 1 {
            option_text.sections[0].value = match option_state.shadows_on {
                0 => "  On  ".to_string(),
                _ => "  Off ".to_string(),
            };
        }

        if option_row.row == 2 {
            option_text.sections[0].value = match option_state.game_version {
                0 => "Latest".to_string(),
                _ => " Jam  ".to_string(),
            };
        }
    }
}
