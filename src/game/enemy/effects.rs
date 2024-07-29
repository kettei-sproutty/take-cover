use bevy::prelude::*;
use bevy_particle_systems::*;

use super::ENEMY_SPRITE_SIZE;

pub fn make_dirt_effect(dirt_sprite: Handle<Image>) -> ParticleSystemBundle {
  ParticleSystemBundle {
    particle_system: ParticleSystem {
      max_particles: 50,
      color: ColorOverTime::Constant(Color::srgba(1.0, 1.0, 1.0, 1.0)),
      texture: ParticleTexture::Sprite(dirt_sprite),
      spawn_rate_per_second: 5.0.into(),
      initial_speed: JitteredValue::jittered(2.0, -1.0..1.0),
      lifetime: JitteredValue::jittered(2.0, -0.5..0.5),
      looping: true,
      system_duration_seconds: 10.0,
      scale: ValueOverTime::Curve(Curve::new(vec![
        CurvePoint::new(1.8, 0.0),
        CurvePoint::new(0.0, 1.0),
      ])),
      ..ParticleSystem::default()
    },
    transform: Transform::from_xyz(0.0, -ENEMY_SPRITE_SIZE * 0.5, 1.0),
    ..default()
  }
}

pub fn make_attack_effect(dirt_sprite: Handle<Image>) -> ParticleSystemBundle {
  ParticleSystemBundle {
    particle_system: ParticleSystem {
      color: ColorOverTime::Constant(Color::srgba(1.0, 1.0, 1.0, 1.0)),
      texture: ParticleTexture::Sprite(dirt_sprite),
      bursts: vec![ParticleBurst {
        count: 30,
        time: 0.0,
      }],
      spawn_rate_per_second: 30.0.into(),
      initial_speed: JitteredValue::jittered(70.0, -20.0..10.0),
      rotate_to_movement_direction: true,
      lifetime: JitteredValue::jittered(2.0, -0.5..0.5),
      looping: true,
      system_duration_seconds: 1.0,
      scale: ValueOverTime::Curve(Curve::new(vec![
        CurvePoint::new(1.8, 0.0),
        CurvePoint::new(0.0, 0.5),
      ])),
      ..ParticleSystem::default()
    },
    transform: Transform::from_xyz(0.0, ENEMY_SPRITE_SIZE * 0.25, 3.0),
    ..default()
  }
}

pub fn make_dead_enemy_effect(dead_sprite: Handle<Image>) -> ParticleSystemBundle {
  ParticleSystemBundle {
    particle_system: ParticleSystem {
      color: ColorOverTime::Constant(Color::srgba(1.0, 1.0, 1.0, 1.0)),
      texture: ParticleTexture::Sprite(dead_sprite),
      bursts: vec![ParticleBurst {
        count: 5,
        time: 0.0,
      }],
      spawn_rate_per_second: 5.0.into(),
      initial_speed: JitteredValue::jittered(70.0, -20.0..10.0),
      rotate_to_movement_direction: true,
      lifetime: JitteredValue::jittered(2.0, -0.5..0.5),
      looping: false,
      system_duration_seconds: 1.0,
      scale: ValueOverTime::Curve(Curve::new(vec![
        CurvePoint::new(1.8, 0.0),
        CurvePoint::new(0.0, 0.5),
      ])),
      ..ParticleSystem::default()
    },
    transform: Transform::from_xyz(0.0, ENEMY_SPRITE_SIZE * 0.25, 1.0),
    ..default()
  }
}
