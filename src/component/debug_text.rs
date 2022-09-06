use crate::*;

#[derive(Component)]
pub struct DebugText;

impl DebugText {
  pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraMono-Medium.ttf");
    commands
      .spawn_bundle(
        TextBundle::from_section(
          "Debug",
          TextStyle {
            font: font.clone(),
            font_size: 25.,
            color: Color::WHITE,
          },
        )
        .with_style(Style {
          align_self: AlignSelf::FlexEnd,
          position_type: PositionType::Absolute,
          position: UiRect {
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
          },
          ..default()
        }),
      )
      .insert(DebugText);
  }

  pub fn update(
    mut query: Query<&mut Text, With<DebugText>>,
    zones: Res<Zones>,
    player_query: Query<&Transform, With<Player>>,
  ) {
    let mut text = query.single_mut();
    let pt = player_query.single();
    if let Some(zone) = zones.zone(&pt.translation) {
      for building in &zone.buildings {
        if let Some(cell) = building.pos_global_to_cell(pt.translation) {
          text.sections[0].value = format!("Coord: {},{}", cell.coord.z, cell.coord.x);
          return;
        }
      }
    }

    text.sections[0].value = format!("Coord: None");
  }
}
