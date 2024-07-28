use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn make_dirt_effect() -> EffectAsset {
  // Create a color gradient for the particles
  let mut gradient = Gradient::new();
  gradient.add_key(0.0, Vec4::new(0.5, 0.11, 0.18, 1.0));
  gradient.add_key(1.0, Vec4::new(0.5, 0.11, 0.18, 0.0));

  let writer = ExprWriter::new();

  let age = writer.lit(0.).expr();
  let init_age = SetAttributeModifier::new(Attribute::AGE, age);

  let lifetime = writer.lit(0.5).expr();
  let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

  let init_pos = SetPositionCircleModifier {
    center: writer.lit(Vec3::ZERO).expr(),
    axis: writer.lit(Vec3::Z).expr(),
    radius: writer.lit(5.0).expr(),
    dimension: ShapeDimension::Surface,
  };

  let init_vel = SetVelocityCircleModifier {
    center: writer.lit(Vec3::ZERO).expr(),
    axis: writer.lit(Vec3::Z).expr(),
    speed: writer.lit(0.1).expr(),
  };

  let mut module = writer.finish();

  let round = RoundModifier::constant(&mut module, 2.0 / 3.0);

  // Create a new effect asset spawning 30 particles per second from a circle
  // and slowly fading from blue-ish to transparent over their lifetime.
  // By default the asset spawns the particles at Z=0.
  let spawner = Spawner::burst(5.0.into(), 0.25.into());

  EffectAsset::new(vec![4096], spawner, module)
    .with_name("2d")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .render(SizeOverLifetimeModifier {
      gradient: Gradient::constant(Vec2::splat(20.0)),
      screen_space_size: false,
    })
    .render(ColorOverLifetimeModifier { gradient })
    .render(round)
}
