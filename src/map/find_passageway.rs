use super::*;

impl GameMap
{	//
	pub fn identify_halls_and_passageways( &mut self )
	{	//通路をマーキングして広間と分ける。更に行き止まりをマーキングする
		for ( x, ary ) in self.map.iter().enumerate()
		{	for ( y, _obj ) in ary.iter().enumerate()
			{	if matches!( self.map[ x ][ y ], MapObj::Wall(_) ) { continue }
	
				let ( passageway, count ) = is_passageway( self, x, y );
				if passageway
				{	self.stat[ x ][ y ] |= BIT2_PASSAGEWAY;
					if count == 3 { self.stat[ x ][ y ] |= BIT3_DAED_END }
				}
			}
		}
	
		//
		for ( x, ary ) in self.map.iter().enumerate()
		{	for ( y, _obj ) in ary.iter().enumerate()
			{	if self.stat[ x ][ y ] & BIT3_DAED_END == 0 { continue }
		
				if is_alcove( self, x, y ) < 3
				{	self.stat[ x ][ y ] |= BIT4_ALCOVE
				}
			}
		}
	}

	pub fn spawn_sprite_systile
	(	&mut self,
		cmds: &mut Commands,
		color_matl: &mut ResMut<Assets<ColorMaterial>>,
	)
	{
		for x in MAP_DIGABLE_X
		{	for y in MAP_DIGABLE_Y
			{	let xy = conv_sprite_coordinates( x, y );
				if self.stat[ x as usize ][ y as usize ] & BIT4_ALCOVE != 0
				{	cmds.spawn_bundle( sprite_system_tile( xy, color_matl, Color::MAROON ) )
						.insert( SysTileSprite );
				}
				else if self.stat[ x as usize ][ y as usize ] & BIT3_DAED_END != 0
				{	cmds.spawn_bundle( sprite_system_tile( xy, color_matl, Color::DARK_GREEN ) )
						.insert( SysTileSprite );
				}
				else if self.stat[ x as usize ][ y as usize ] & BIT2_PASSAGEWAY != 0
				{	cmds.spawn_bundle( sprite_system_tile( xy, color_matl, Color::MIDNIGHT_BLUE ) )
						.insert( SysTileSprite );
				}
			}
		}
	}	
}
//

//
fn is_passageway( maze: &GameMap, x:usize, y:usize ) -> ( bool, i32 )
{	let objs = maze.enclosure( x as i32, y as i32 );

	if ! matches!( objs.upper_left  , MapObj::Wall(_) )
	&& ! matches!( objs.upper_center, MapObj::Wall(_) )
	&& ! matches!( objs.middle_left , MapObj::Wall(_) ) { return ( false, 0 ) }

	if ! matches!( objs.upper_right , MapObj::Wall(_) )
	&& ! matches!( objs.upper_center, MapObj::Wall(_) )
	&& ! matches!( objs.middle_right, MapObj::Wall(_) ) { return ( false, 0 ) }

	if ! matches!( objs.middle_left , MapObj::Wall(_) )
	&& ! matches!( objs.lower_left  , MapObj::Wall(_) )
	&& ! matches!( objs.lower_center, MapObj::Wall(_) ) { return ( false, 0 ) }

	if ! matches!( objs.middle_right, MapObj::Wall(_) )
	&& ! matches!( objs.lower_center, MapObj::Wall(_) )
	&& ! matches!( objs.lower_right , MapObj::Wall(_) ) { return ( false, 0 ) }

	let mut count = 0;
	if matches!( objs.upper_center, MapObj::Wall(_) ) { count += 1 }
	if matches!( objs.middle_left , MapObj::Wall(_) ) { count += 1 }
	if matches!( objs.middle_right, MapObj::Wall(_) ) { count += 1 }
	if matches!( objs.lower_center, MapObj::Wall(_) ) { count += 1 }

	( true, count )
}

fn is_alcove( maze: &GameMap, start_x:usize, start_y:usize ) -> usize
{	let mut map_x = start_x as i32;
	let mut map_y = start_y as i32;
	let mut pedometer = 1;
	let bak_x = -1;
	let bak_y = -1;
	let mut next_x = -1;
	let mut next_y = -1;

	loop
	{	let mut count = 0;
		for ( dx, dy ) in DIRECTION.iter()
		{	let tmp_x = map_x + dx;
			let tmp_y = map_y + dy;
			if ! MAP_INDEX_X.contains( &tmp_x )
			|| ! MAP_INDEX_Y.contains( &tmp_y )
			|| matches!( maze.map[ tmp_x as usize ][ tmp_y as usize ], MapObj::Wall(_) ) { continue }

			if ( tmp_x, tmp_y ) != ( bak_x, bak_y )
			{	count += 1;
				next_x = tmp_x;
				next_y = tmp_y;
			}
		}

		if count == 1
		{	pedometer+= 1;
			map_x = next_x;
			map_y = next_y;
		}
		else
		{
			break;
		}
	}
	pedometer
}

fn sprite_system_tile
(	( x, y ): ( f32, f32 ),
	color_matl: &mut ResMut<Assets<ColorMaterial>>,
	color: Color,
//	darkmode: bool,
) -> SpriteBundle
{	SpriteBundle
	{	material : color_matl.add( color.into() ),
		transform: Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_SYSTILE ) ),
		sprite   : Sprite::new( Vec2::new( SYSTILE_PIXEL, SYSTILE_PIXEL ) ),
//		visible  : Visible { is_visible: ! darkmode, ..Default::default() },
		..Default::default()
	}
}

//End of code.