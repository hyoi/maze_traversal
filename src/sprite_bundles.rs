use super::*;

//壁のスプライト
const WALL_PIXEL: f32   = PIXEL_PER_GRID;

//ドットのスプライト
const DOT_RAIDUS: f32   = PIXEL_PER_GRID / 14.;
const DOT_COLOR : Color = Color::WHITE;

//ゴールのスプライト
const GOAL_PIXEL: f32 = PIXEL_PER_GRID / 2.;
const GOAL_COLOR : Color = Color::YELLOW;

//自機のスプライト
const PLAYER_PIXEL: f32   = PIXEL_PER_GRID / 2.5;
const PLAYER_COLOR: Color = Color::YELLOW;

//スプライトのZ軸の順位
const SPRITE_DEPTH_MAZE  : f32 = 0.;
const SPRITE_DEPTH_PLAYER: f32 = 1.;

////////////////////////////////////////////////////////////////////////////////////////////////////

//壁用のスプライトバンドルを生成
pub fn sprite_wall
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
	asset_svr: &Res<AssetServer>,
) -> SpriteBundle
{	let texture_handle = asset_svr.load( SPRITE_WALL_FILE ).into();
	let locate   = Vec3::new( x, y, SPRITE_DEPTH_MAZE );
	let square   = Vec2::new( WALL_PIXEL, WALL_PIXEL );

	SpriteBundle
	{	material : color_matl.add( texture_handle ),
		transform: Transform::from_translation( locate ),
		sprite   : Sprite::new( square ),
		..Default::default()
	}
}

//ドット用のスプライトバンドルを生成
pub fn sprite_dot( ( x, y ): ( f32, f32 ) ) -> ShapeBundle
{	let locate   = Vec3::new( x, y, SPRITE_DEPTH_MAZE );

	let circle = &shapes::Circle { radius: DOT_RAIDUS, ..shapes::Circle::default() };
	GeometryBuilder::build_as
	(	circle,
		ShapeColors::new( DOT_COLOR ),
        DrawMode::Fill( FillOptions::default() ),
        Transform::from_translation( locate ),
    )
}

//ゴールのスプライトバンドルを生成
pub fn sprite_goal
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>
) -> SpriteBundle
{	let locate   = Vec3::new( x, y, SPRITE_DEPTH_MAZE );
	let square   = Vec2::new( GOAL_PIXEL, GOAL_PIXEL );

	let mut sprite = SpriteBundle
	{	material : color_matl.add( GOAL_COLOR.into() ),
		transform: Transform::from_translation( locate ),
		sprite   : Sprite::new( square ),
		..Default::default()
	};

	//45°傾けて菱形に見せる
	let quat = Quat::from_rotation_z( 45_f32.to_radians() );
	sprite.transform.rotate( quat ); //.rotate()は()を返すのでメソッドチェーンできない

	sprite
}

//自機のスプライトバンドルを生成
pub fn sprite_player( ( x, y ): ( f32, f32 ) ) -> ShapeBundle
{	let locate = Vec3::new( x, y, SPRITE_DEPTH_PLAYER );

	let triangle = &shapes::RegularPolygon
	{	sides: 3,
		feature: shapes::RegularPolygonFeature::Radius( PLAYER_PIXEL ),
		..shapes::RegularPolygon::default()
	};
	GeometryBuilder::build_as
	(	triangle,
		ShapeColors::new( PLAYER_COLOR ),
        DrawMode::Fill( FillOptions::default() ),
		Transform::from_translation( locate )
	)
}

//End of c#de.