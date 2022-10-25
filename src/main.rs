use bevy::{
    prelude::{shape::Quad, *},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut normal_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle((
            Transform::from_xyz(0., -200., 0.).with_scale(Vec3::ONE * 0.5),
            GlobalTransform::default(),
            Visibility::default(),
            ComputedVisibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32., 32.)),
                    color: Color::BLUE,
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Vec2::new(300., 75.).into(),
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    });

    let mesh = Mesh::from(Quad::new(Vec2::splat(100.)));
    let mesh_handle = meshes.add(mesh);
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(mesh_handle),
        material: normal_materials.add(ColorMaterial::from(Color::BLACK)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}
