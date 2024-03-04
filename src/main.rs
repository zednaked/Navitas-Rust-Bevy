use std::borrow::Borrow;

use bevy::{app, prelude::*, utils::warn};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::*;
use std::time::Duration;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use extol_sprite_layer::{LayerIndex, SpriteLayerPlugin, SpriteLayerOptions};


#[derive(Debug, Copy, Clone, Component, PartialEq, Eq, Hash)]
enum SpriteLayer {
    Background,
    Object,
    Enemy,
    Player,
    Ui,
}

impl LayerIndex for SpriteLayer {
    // Convert your type to an actual z-coordinate.
    fn as_z_coordinate(&self) -> f32 {
        use SpriteLayer::*;
        match *self {
            // Note that the z-coordinates must be at least 1 apart...
            Background => 0.,
            Object => 1.,
            Enemy => 2.,
            // ... but can be more than that.
            Player => 990.,
            Ui => 995.
        }
    }
}


#[derive(Component)]
pub struct Player{
    pub speed: f32,
    pub velociodade_atual: f32,
    pub direction: Vec2,

}

fn main() {
    let mut app = App::new();
         
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    //app.add_plugins(WorldInspectorPlugin::default());

    app.add_event::<Explosion>();
    app.insert_resource(ClearColor(Color::rgb(0.1, 0.05, 0.1)));
    app.add_plugins(EstrelasPlugin);
    app.add_plugins(AsteroidesPlugin);
    app.insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)));
    app.add_systems(Startup, setup);
    app.add_systems(Startup, spawna_player);
    app.add_systems(Update, move_player);
    app.add_plugins(SpriteLayerPlugin::<SpriteLayer>::default());
    app.add_plugins(CollisionPlugin);
    app.add_plugins(BulletPlugin);
    app.add_systems(Update, particle_system);
    //app.add_systems(Update, pausa);
    app.add_systems(Update, move_particles);
    app.run();
}

fn setup (mut cmd: Commands, mut windows: Query  <&mut Window>){ 

    let mut window = windows.single_mut();
    window.canvas = Some ("Bevy Canvas".into());
    
    window.resize_constraints = bevy::window::WindowResizeConstraints {
        min_width: 600.,
        min_height: 900.,
        max_width: 600.,
        max_height: 900.,
    };

    window.prevent_default_event_handling = false;
    window.resolution.set(600., 900.);
    cmd.spawn(Camera2dBundle::default());
    cmd.spawn((Text2dBundle {
        text: Text::from_section(">Navitas< - Rust+Bevy+WASM - v 0.11", TextStyle { color: Color::rgb(50.0, 100.0,155.0) , font_size: 12.0 , ..default()}),
        transform: Transform::from_translation(Vec3::new(0.,430.,0.)),
        ..default() 
    }

    ));
   
}


fn pausa (keys: Res<ButtonInput<KeyCode>>,  mut app: App)
{
    if keys.just_pressed(KeyCode::Space) {
        
        app.world.resource_mut::<Time<Virtual>>().pause();

    }
}



fn spawna_player (mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((Player{ speed: 10., velociodade_atual: 0.0, direction: Vec2::ZERO},
        SpriteBundle {
            texture: asset_server.load("Ship/2.png"),
        sprite: Sprite{
            
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        
        transform: Transform::from_translation(Vec3::new(0.,-420.,0.)),
        ..default()
    }));
}


fn move_player ( 
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut player: Query<&mut Player>,
) {
    let mut direction = Vec2::ZERO;
    let mut player = player.single_mut();
    let move_speed =player.velociodade_atual;
    let mut move_delta = Vec2::ZERO;

   if keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

   if direction == Vec2::ZERO {
        if move_speed < 0.0 {
            return;
        }
        
        if player.velociodade_atual > 0.0 {
            player.velociodade_atual -= 15.5 * time.delta_seconds() ; //desaceleração
        }
        
        move_delta = player.direction * player.velociodade_atual;
        for mut transform in &mut query {
            transform.translation += move_delta.extend(0.0);
        }
        return;
    }else{
        player.direction = direction;
        player.velociodade_atual += 45.1 * time.delta_seconds();
        if player.velociodade_atual > 8. {
            player.velociodade_atual = 8.; //Limita a velocidade
        }
        move_delta = player.direction * player.velociodade_atual;
        for mut transform in &mut query {
            transform.translation += move_delta.extend(0.0);
        }

    }
    
}


/*--------------------------------- estrela------------------------- */
#[derive(Component)]
pub struct Estrela;

#[derive(Resource)]
struct TimerEstrelas(Timer);

struct  SpawnerEstrelas;

impl Plugin for SpawnerEstrelas {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawna_estrelas)
        .insert_resource(TimerEstrelas(Timer::from_seconds(0.07, TimerMode::Repeating)));
    }
    
}

pub struct EstrelasPlugin;

impl Plugin for EstrelasPlugin {
    fn build(&self, app: &mut App) {        
        app.add_plugins(SpawnerEstrelas);        
        app.add_systems(Update, despawna_estrelas);
        app.add_systems(Update, move_estrelas);
    }
}

fn move_estrelas(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Estrela>>,
) {
    for mut transform in &mut query {
        transform.translation.y -= 485. * time.delta_seconds();
    }
}

 

fn spawna_estrelas(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>, mut timer: ResMut<TimerEstrelas> ){
    if timer.0.tick(time.delta()).just_finished() {
    
        let mut rng = thread_rng();
        let x = rng.gen_range(-300.0..300.0);
        let y = 450.0;

        let tamanho: f32 = rng.gen_range(1.0..4.0);

        commands.spawn((
            Estrela,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(rng.gen(), 1., 1.),
                    custom_size: Some(Vec2::new(tamanho, tamanho)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..default()
                },
                ..default()
            },
         
        ));
    }
}

pub fn despawna_estrelas(
    mut commands: Commands,
    query: Query<(Entity, &Estrela, &Transform )>,
    windows: Query<&Window>
) {
    let window = windows.single();
    let window_height = window.height();
    
    for (entity, _, transform) in query.iter() {
        if transform.translation.y < -450.  {
            commands.entity(entity).despawn();
        }
    }
}

/*---------------------------------fim estrela --------------------------------------- */



/*--------------------------------- asteroides ------------------------- */
#[derive(Component)]
pub struct Asteroide;

#[derive(Resource)]
struct TimerAsteroide(Timer);

struct SpawnerAsteroide;

impl Plugin for SpawnerAsteroide {
    fn build(&self, app: &mut App) {
        let mut rgn = thread_rng();
        app.add_systems(Update, spawna_asteroides)
            .insert_resource(TimerAsteroide(Timer::from_seconds(0.500, TimerMode::Repeating)));
    }
}

pub struct AsteroidesPlugin;

impl Plugin for AsteroidesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpawnerAsteroide);
        app.add_systems(Update, despawna_asteroides);
        app.add_systems(Update, move_asteroides);
    }
}

fn move_asteroides(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Asteroide>>,
) {
    

    for mut transform in &mut query {
        let mut rng = thread_rng();
        transform.translation.y -= 400. * time.delta_seconds();
        transform.translation.x -= 1.;  // rng.gen_range(-1.0..1.0);
        transform.rotation *= Quat::from_rotation_z(-0.1);
    }
}

fn spawna_asteroides(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<TimerAsteroide>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        let x = rng.gen_range(-300.0..300.0);
        let y = 450.0;

        let tamanho: f32 = rng.gen_range(14.0..75.0);

        commands.spawn((
            Asteroide,
            SpriteBundle {
                texture: asset_server.load("Background/Meteor1.png"),
                sprite: Sprite {
                    color: Color::rgb(rng.gen(), 1., 1.),
                    custom_size: Some(Vec2::new(tamanho, tamanho)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    rotation: Quat::from_rotation_z(rng.gen_range(0.0..std::f32::consts::PI)),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

pub fn despawna_asteroides(
    mut commands: Commands,
    query: Query<(Entity, &Asteroide, &Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_height = window.height();

    for (entity, _, transform) in query.iter() {
        if transform.translation.y < -450. {
            commands.entity(entity).despawn();
        }
    }
}

/*---------------------------------fim asteroides ----------------------- */

/*----------------------------------Colisao ------------------------------ */

#[derive(Component)]
struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collision);
    }
}


fn check_collision(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Sprite), With<Asteroide>>,
    player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
    bullet_query: Query<(Entity, &Transform, &Sprite), With<Bullet>>,
    mut eventos: EventWriter<Explosion>,
) {
    for (asteroid_entity, asteroid_transform, asteroid_sprite) in query.iter() {
        for (player_entity, player_transform, player_sprite) in player_query.iter() {
            if is_collision(asteroid_transform, asteroid_sprite, player_transform, player_sprite) {
                // Handle collision between asteroid and player
                // Destroy asteroid and player entities, or apply damage, etc.
                eventos.send(Explosion((asteroid_transform.translation.x, asteroid_transform.translation.y).into()));
                commands.entity(asteroid_entity).despawn();
               // commands.entity(player_entity).despawn();
            }
        }

        for (bullet_entity, bullet_transform, bullet_sprite) in bullet_query.iter() {
            if is_collision(asteroid_transform, asteroid_sprite, bullet_transform, bullet_sprite) {
                // Handle collision between asteroid and bullet
                // Destroy asteroid and bullet entities, or apply damage, etc.
                eventos.send(Explosion((asteroid_transform.translation.x, asteroid_transform.translation.y).into()));
                commands.entity(asteroid_entity).despawn();
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

fn is_collision(
    entity1_transform: &Transform,
    entity1_sprite: &Sprite,
    entity2_transform: &Transform,
    entity2_sprite: &Sprite,
) -> bool {
    // Calculate the distance between the entities
    let distance = entity1_transform.translation.distance(entity2_transform.translation);

    // Calculate the sum of the sizes of the entities
    let size1 = 50.;
    let size2 = 50.;
    let sum_of_sizes = size1 + size1 + size2 + size2;

    // Check if the distance is less than the sum of the sizes
    if distance <= 50.0 {
        return true;
    }

    return false;
}

/*----------------------------------Fim Colisao ------------------------------ */

/*----------------------------------Bullet ------------------------------ */


#[derive(Component)]
pub struct Bullet; 

#[derive(Resource)]
struct TimerBullet(Timer);

struct SpawnerBullet;

impl Plugin for SpawnerBullet {
    fn build(&self, app: &mut App) {
        let mut rgn = thread_rng();
        app.add_systems(Update, spawna_bullets)
            .insert_resource(TimerBullet(Timer::from_seconds(0.600, TimerMode::Repeating)));
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpawnerBullet);
        app.add_systems(Update, despawna_bullets);
        app.add_systems(Update, move_bullets);
    }
}

fn move_bullets(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Bullet>>,
) {
    for mut transform in &mut query {
        transform.translation.y += 485. * time.delta_seconds();
    }
}

fn spawna_bullets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<TimerBullet>,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Transform, &Sprite), With<Player>>,
) {
    for (player_entity, player_transform, player_sprite) in player_query.iter() {
            if timer.0.tick(time.delta()).just_finished() {
                commands.spawn((
                    Bullet,
                    SpriteBundle {
                        texture: asset_server.load("Shoot/1.png"),
                        sprite: Sprite {
                            color: Color::rgb(1., 1., 1.),
                            custom_size: Some(Vec2::new(20.0, 20.0)),
                            ..default()
                        },
                        transform: Transform {
                            translation: player_transform.translation,
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
    }
}

fn despawna_bullets(
    mut commands: Commands,
    query: Query<(Entity, &Bullet, &Transform)>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_height = window.height();

    for (entity, _, transform) in query.iter() {
        if transform.translation.y > 150. {
            commands.entity(entity).despawn();
        }
    }
}

/*----------------------------------Fim Bullet ------------------------------ */

#[derive(Event)]
struct Explosion(Vec2);


#[derive(Component)]
pub struct Particle {
    position: Vec2,
    velocity: f32,
    size: f32,
    color: Color,
    direction: Vec2,
    time: f32,
    lapsed: f32,
}

fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut events: EventReader<Explosion>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    
) {
    let mut rng = thread_rng();
    for event in events.read() {
        let position = event.0;
        for _ in (0..18) {
    
        commands.spawn((
            Particle {
                position,
                velocity: rng.gen_range(50.0..150.0),
                size: 5.0,
                color: Color::rgb(1.0, 0.0, 0.0),
                direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                time: time.elapsed_seconds(), 
                lapsed: 0.0, 
            },
            SpriteLayer::Background,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: rng.gen_range(25.5..35.0) })),
                material: materials.add(Color::hsl(rng.gen_range(71.5..98.0), 0.0, 0.0)),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT to +X_EXTENT.
                    position.x + rng.gen_range(-10.0..10.0),
                    position.y + rng.gen_range(-10.0..10.0),
                    -1.0,
                ),
                ..default()
            },        
        ));
        commands.spawn((
            Particle {
                position,
                velocity: rng.gen_range(50.0..150.0),
                size: 5.0,
                color: Color::rgb(1.0, 0.0, 0.0),
                direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                time: time.elapsed_seconds(), 
                lapsed: 0.0, 
            },
            SpriteLayer::Object,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: rng.gen_range(15.5..35.0) })),
                material: materials.add(Color::hsl(rng.gen_range(71.5..98.0), 0.1, 1.0)),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT to +X_EXTENT.
                    position.x + rng.gen_range(-10.0..10.0),
                    position.y + rng.gen_range(-10.0..10.0),
                    1.0,
                ),
                ..default()
            },        
        ));


    }
        


    }
}

fn move_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform)>,
) {

    for (entity, mut particle, mut transform) in &mut query {
        let direcao = particle.direction.clone();
        let velocidade = particle.velocity.clone();

        particle.lapsed += time.delta_seconds();
        
        particle.position += direcao * velocidade * time.delta_seconds();
        transform.translation = particle.position.extend(0.0);

        if particle.lapsed > 0.5 {
            commands.entity(entity).despawn();
        }

    }
    

}
