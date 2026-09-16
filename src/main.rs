use bevy::{prelude::*, render::mesh::RectangleMeshBuilder, camera::*, window::WindowPlugin};

const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 270.0;
const WINDOW_TITLE: &str = "Pong";
const BACKGROUND_COLOR: Color = Color::BLACK;
const PLAYER_SPEED: f32 = 250.0;
const BALL_SPEED: f32 = 150.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_players, move_ball, collide_and_score).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: WINDOW_WIDTH,
                min_height: WINDOW_HEIGHT,
            }, 
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Mesh2d(meshes.add(RectangleMeshBuilder::new(10.0, 75.0).build())),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(-235.0, 0.0, 0.0),
        Score {val: 0},
        Player1,
    ));

    commands.spawn((
        Mesh2d(meshes.add(RectangleMeshBuilder::new(10.0, 75.0).build())),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(235.0, 0.0, 0.0),
        Score {val: 0},
        Player2,
    ));

    commands.spawn((
        Mesh2d(meshes.add(RectangleMeshBuilder::new(7.5, 7.5).build())),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0,0.0,0.0),
        BallDir {x: 1.0, y: 1.0},
    ));

    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Transform::from_xyz(-220.0, 120.0, 0.0),
        Player1ScoreText,
    ));

    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Transform::from_xyz(220.0, 120.0, 0.0),
        Player2ScoreText,
    ));
}

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player1ScoreText;


#[derive(Component)]
pub struct Player2;

#[derive(Component)]
pub struct Player2ScoreText;

#[derive(Component)]
pub struct Score {
    val: i32
}

#[derive(Component)]
pub struct BallDir {
    x: f32,
    y: f32,
}

fn move_players(mut transform_1: Single<&mut Transform, (With<Player1>, Without<Player2>)>, mut transform_2: Single<&mut Transform, (With<Player2>, Without<Player1>)>, time: Res<Time>, keys: Res<ButtonInput<KeyCode>>){
    if keys.pressed(KeyCode::KeyW) {
        transform_1.translation.y += PLAYER_SPEED * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyS) {
        transform_1.translation.y -= PLAYER_SPEED * time.delta_secs();
    }

    transform_1.translation.y = transform_1.translation.y.clamp(-105.0, 105.0);

    if keys.pressed(KeyCode::ArrowUp) {
        transform_2.translation.y += PLAYER_SPEED * time.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowDown) {
        transform_2.translation.y -= PLAYER_SPEED * time.delta_secs();
    }

    transform_2.translation.y = transform_2.translation.y.clamp(-105.0, 105.0);
}

fn move_ball(mut ball: Single<(&BallDir, &mut Transform)>, time: Res<Time>) {
    let ball_dir = ball.0;
    let transform_b = &mut ball.1;

    transform_b.translation.x += ball_dir.x * time.delta_secs() * BALL_SPEED;
    transform_b.translation.y += ball_dir.y * time.delta_secs() * BALL_SPEED;
}

fn collide_and_score(mut score_text_1: Single<&mut Text2d, (With<Player1ScoreText>, Without<Player2ScoreText>)>, mut score_text_2: Single<&mut Text2d, (With<Player2ScoreText>, Without<Player1ScoreText>)>, mut score_1: Single<&mut Score, (With<Player1>, Without<Player2>)>, mut score_2: Single<&mut Score, (With<Player2>, Without<Player1>)>, mut transform_b: Single<&mut Transform, (With<BallDir>, Without<Player1>, Without<Player2>)>, mut ball_dir: Single<&mut BallDir>, transform_1: Single<&Transform, (With<Player1>, Without<Player2>, Without<BallDir>)>, transform_2: Single<&Transform, (With<Player2>, Without<Player1>, Without<BallDir>)>, mut exit: MessageWriter<AppExit>) {
    if transform_b.translation.y > 135.0 {
        ball_dir.y *= -1.0;
        transform_b.translation.y = 135.0;
    }

    if transform_b.translation.y < -135.0 {
        ball_dir.y *= -1.0;
        transform_b.translation.y = -135.0;
    }

    if transform_b.translation.x < -240.0 || transform_b.translation.x > 240.0 {
        exit.write(AppExit::Success);
    }

    if aabb((transform_1.translation.x, transform_1.translation.y, 10.0, 75.0), (transform_b.translation.x, transform_b.translation.y, 7.5, 7.5)) {
        ball_dir.x *= -1.0;
        transform_b.translation.x = -220.0;
        score_1.val += 1;
        score_text_1.0 = score_1.val.to_string();
    }

    if aabb((transform_2.translation.x, transform_2.translation.y, 10.0, 75.0), (transform_b.translation.x, transform_b.translation.y, 7.5, 7.5)) {
        ball_dir.x *= -1.0;
        transform_b.translation.x = 220.0;
        score_2.val += 1;
        score_text_2.0 = score_2.val.to_string();
    }
}

fn aabb(obj_1: (f32, f32, f32, f32), obj_2: (f32, f32, f32, f32)) -> bool {
    return obj_1.0 - obj_1.2/2.0 < obj_2.0 + obj_2.2/2.0 && obj_1.1 - obj_1.3/2.0 < obj_2.1 + obj_2.3/2.0 && obj_1.0 + obj_1.2/2.0 > obj_2.0 - obj_2.2/2.0 && obj_1.1 + obj_1.3/2.0 > obj_2.1 - obj_2.3/2.0
}