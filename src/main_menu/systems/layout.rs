use bevy::prelude::*;

use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::{
        get_button_text_style, BUTTON_STYLE, IMAGE_STYLE, MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR,
        TITLE_STYLE,
    },
};

pub fn spawn_main_menu(mut cmd: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut cmd, &asset_server);
}

pub fn despawn_main_menu(mut cmd: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        cmd.entity(main_menu_entity).despawn_recursive(); //Despawn the parent and all of it's children
    }
}

pub fn build_main_menu(cmd: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = cmd
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                //background_color: Color::RED.into(),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            //TITLE
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    //IMG 1
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_blue_large.png").into(),
                        ..default()
                    });

                    //TEXT
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bevy Ball Game",
                                get_button_text_style(asset_server, 64.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    //IMG 2
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/ball_red_large.png").into(),
                        ..default()
                    });
                });

            //PLAY
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(asset_server, 32.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            //QUIT
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(asset_server, 32.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
