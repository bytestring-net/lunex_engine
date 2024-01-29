use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin::<NoData, NoData, MyWidget>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, MyWidget>::new())

        //.add_plugins(Shape2dPlugin::default())
        //.add_systems(Update, render_update)
        .add_systems(PreStartup, prestartup)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, assets: Res<AssetCache>, mut _materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn((
        MyWidget,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera::default(),
            ..default()
        }
    ));

    commands.spawn((
        UiTreeBundle::<NoData, NoData, MyWidget> {
            tree: UiTree::new("MyWidget"),
            dimension: Dimension::new((1000.0, 1000.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MovableByCamera,
    )).with_children(|parent| {

        let root = UiLink::path("Root");
        parent.spawn((
            MyWidget,
            root.clone(),
            UiLayout::Window::FULL.pack(),
            UiStack::new().direction(StackDirection::Vertical)//.gap(Abs::SM),
        ));










        parent.spawn((
            MyWidget,
            root.new(),
            UiLayout::Div::new().pad(Abs::MD).margin(Abs::ZERO).align_x(Align::START).pack(),
        ));
        parent.spawn((
            MyWidget,
            root.new(),
            UiLayout::Div::new().pad(Abs::MD).margin(Abs::ZERO).align_x(Align::CENTER).pack(),
        ));
        parent.spawn((
            MyWidget,
            root.new(),
            UiLayout::Div::new().pad(Abs::MD).margin(Abs::ZERO).align_x(Align::END).pack(),
        ));

        parent.spawn((
            MyWidget,
            root.new(),
            UiLayout::Div::new().pad(Abs::MD).pad_x(Prc(10.0)).margin(Abs::SM).br().pack(),
        ));



        for _ in 0..2 {
            parent.spawn((
                MyWidget,
                root.new(),
                UiLayout::Div::new().pad(Abs::MD).margin(Abs::SM).pack(),
            ));
        }

        parent.spawn((
            MyWidget,
            root.new(),
            UiLayout::Div::new().pad(Abs::MD).margin(Abs::SM).br().pack(),
            /*UiText2dBundle {
                text: Text::from_section("hello world!",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::RED,
                    }),
                ..default()
            }*/
            //UiImage2dBundle::from(assets.main_background.clone())
        ));

        for _ in 0..3 {
            parent.spawn((
                MyWidget,
                root.new(),
                UiLayout::Div::new().pad(Abs::MD).margin(Abs::SM).pack(),
            ));
        }


        parent.spawn((
            MyWidget,
            root.add(".||#:2").new(),
            UiLayout::Div::new().pad(Abs::MD).br().pack(),
        ));




        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            UiLayout::Solid::new().align_x(Align::CENTER).pack(),
            UiImage2dBundle::from(assets.main_background.clone())
        ));

    });

}

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;



#[derive(Resource)]
pub struct AssetCache {
    pub font: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub button: Handle<Image>,

    pub switch_base: Handle<Image>,
    pub switch_head: Handle<Image>,

    pub main_background: Handle<Image>,
    pub main_board: Handle<Image>,
    pub main_logo: Handle<Image>,
    pub settings_background: Handle<Image>,
}
fn prestartup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetCache {
        font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
        font_bold: asset_server.load("fonts/rajdhani/Rajdhani-Bold.ttf"),
        button: asset_server.load("images/main_menu/button.png"),

        switch_base: asset_server.load("images/settings/switch_base.png"),
        switch_head: asset_server.load("images/settings/switch_head.png"),

        main_background: asset_server.load("images/main_menu/background.png"),
        main_board: asset_server.load("images/main_menu/board.png"),
        main_logo: asset_server.load("images/main_menu/bevypunk.png"),
        settings_background: asset_server.load("images/settings/background.png"),
    });
}
