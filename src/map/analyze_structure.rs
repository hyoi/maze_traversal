use super::*;

//Sprite
#[derive(Component)]
pub struct SysinfoObj;
const SYSTILE_PIXEL: f32 = PIXEL_PER_GRID;
const SPRITE_DEPTH_SYSINFO: f32 =  5.0;

////////////////////////////////////////////////////////////////////////////////////////////////////

impl GameMap
{	//広間と通路を識別する
	pub fn identify_halls_and_passageways( &mut self )
	{	//全面走査して壁以外のマスを調べる
		for x in MAP_INDEX_X
		{	for y in MAP_INDEX_Y
			{	//通路か？
				if self.is_wall( x, y ) { continue } //壁
				if ! self.judge_passageway( x, y ) { continue } //広間
				self.set_flag_passageway( x, y );

				//上下左右に壁がいくつあるか
				let mut count = 0;
				if self.is_wall_upper_center ( x, y ) { count += 1 }
				if self.is_wall_middle_left  ( x, y ) { count += 1 }
				if self.is_wall_middle_right ( x, y ) { count += 1 }
				if self.is_wall_lower_center ( x, y ) { count += 1 }

				//行き止まりに目印をつける
				if count == 3 { self.set_flag_dead_end( x, y ); }
			}
		}
	}

	//通路か(true)広間か(false)判断する
	fn judge_passageway( &self, x: i32, y: i32 ) -> bool
	{	//通路ではない条件
		if ! self.is_wall_upper_left   ( x, y ) // XX
		&& ! self.is_wall_upper_center ( x, y ) // XO
		&& ! self.is_wall_middle_left  ( x, y ) { return false }

		if ! self.is_wall_upper_right  ( x, y ) // XX
		&& ! self.is_wall_upper_center ( x, y ) // OX
		&& ! self.is_wall_middle_right ( x, y ) { return false }

		if ! self.is_wall_middle_left  ( x, y ) // XO
		&& ! self.is_wall_lower_left   ( x, y ) // XX
		&& ! self.is_wall_lower_center ( x, y ) { return false }

		if ! self.is_wall_middle_right ( x, y ) // OX
		&& ! self.is_wall_lower_center ( x, y ) // XX
		&& ! self.is_wall_lower_right  ( x, y ) { return false }

		true
	}

	//行き止まりの道の長さを数える
	pub fn count_deadend_passageway_length( &mut self )
	{	//全面走査して壁以外のマスを調べる
		for x in MAP_DIGABLE_X
		{	for y in MAP_DIGABLE_Y
			{	//行き止まりを見つける
				if ! self.is_dead_end( x, y ) { continue }
	
				//行き止まりから他の道との合流地点まで遡って道の長さを数える
				let mut pedometer = 0;
				let mut map_xy = ( x, y );
				let mut old_xy = ( -1, -1 );
				loop
				{	let mut next_xy = ( -1, -1 );
					let mut count = 0;
					for ( dx, dy ) in DIRECTION
					{	let tmp_xy = ( map_xy.0 + dx, map_xy.1 + dy );
						if self.is_wall( tmp_xy.0, tmp_xy.1 ) { continue }
						if tmp_xy == old_xy { continue }
						next_xy = tmp_xy;
						count += 1;
					}
					if count != 1 { break }

					//道の長さを＋１する
					pedometer += 1;
					old_xy = map_xy;
					map_xy = next_xy;
				}
				self.count[ x as usize ][ y as usize ] = pedometer;
			}
		}
	}

	//システム情報の表示用スプライト・テキストを生成する
	pub fn spawn_sysinfo_obj
	(	&mut self,
		sysinfo: bool,
		cmds: &mut Commands,
		asset_svr: &Res<AssetServer>,
	)
	{	for x in MAP_DIGABLE_X
		{	for y in MAP_DIGABLE_Y
			{	//行き止まり
				let xy = conv_sprite_coordinates( x as usize, y as usize );
				if self.is_dead_end( x, y )
				{	cmds.spawn_bundle( sprite_sysinfo( xy, Color::MIDNIGHT_BLUE, sysinfo ) )
						.insert( SysinfoObj );
					let info = self.count[ x as usize ][ y as usize ].to_string();
					cmds.spawn_bundle ( text2d_sysinfo( &info, xy, asset_svr, sysinfo ) )
						.insert( SysinfoObj );
				}
				//通路
				else if ! self.is_wall( x, y ) && ! self.is_passageway( x, y )
				{	cmds.spawn_bundle( sprite_sysinfo( xy, Color::INDIGO, sysinfo ) )
						.insert( SysinfoObj );
				}
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//システム情報用のスプライトバンドルを生成
fn sprite_sysinfo( ( x, y ): ( f32, f32 ), color: Color, is_visible: bool ) -> SpriteBundle
{	let position = Vec3::new( x, y, SPRITE_DEPTH_SYSINFO );
	let square = Vec2::new( SYSTILE_PIXEL, SYSTILE_PIXEL ) * 0.9;
	let visibility = Visibility { is_visible };

	let transform = Transform::from_translation( position );
	let sprite = Sprite { color, custom_size: Some( square ), ..Default::default() };

	SpriteBundle { transform, sprite, visibility, ..Default::default() }
}

//システム情報用のテキスト2Dバンドルを生成
fn text2d_sysinfo
(	info: &str,
	( x, y ): ( f32, f32 ),
	asset_svr: &Res<AssetServer>,
	is_visible: bool,
) -> Text2dBundle
{	let style = TextStyle
	{	font: asset_svr.load( FONT_MESSAGE_TEXT ),
		font_size: PIXEL_PER_GRID,
		color: Color::GRAY,
	};
	let align = TextAlignment
	{	vertical: VerticalAlign::Center,
		horizontal: HorizontalAlign::Center,
	};

	Text2dBundle
	{	text     : Text::with_section( info, style, align ),
		transform: Transform::from_translation( Vec3::new( x, y, 15.0 ) ),
		visibility  : Visibility { is_visible },
		..Default::default()
	}
}

//End of code.