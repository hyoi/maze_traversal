use super::*;

//定数

//MAPのレンジ
use std::ops::RangeInclusive;
pub const MAP_INDEX_X  : RangeInclusive<i32> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const MAP_INDEX_Y  : RangeInclusive<i32> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const MAP_DIGABLE_X: RangeInclusive<i32> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const MAP_DIGABLE_Y: RangeInclusive<i32> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

////////////////////////////////////////////////////////////////////////////////////////////////////

//周囲８マスをまとめて格納する型
pub struct Encloser
{	pub upper_left  : MapObj,
	pub upper_center: MapObj,
	pub upper_right : MapObj,
	pub middle_left : MapObj,
	pub middle_right: MapObj,
	pub lower_left  : MapObj,
	pub lower_center: MapObj,
	pub lower_right : MapObj,
}

//GameMap型のメソッド
impl GameMap
{	pub fn enclosure( &self, x: i32, y: i32 ) -> Encloser
	{	let get_map_obj = | x, y |
		{	if MAP_INDEX_X.contains( &x ) && MAP_INDEX_Y.contains( &y )
			{ self.map[ x as usize ][ y as usize ] }
			else
			{ MapObj::Wall( None ) }
		};
	
		Encloser
		{	upper_left  : get_map_obj( x - 1, y - 1 ),
			upper_center: get_map_obj( x    , y - 1 ),
			upper_right : get_map_obj( x + 1, y - 1 ),
			middle_left : get_map_obj( x - 1, y     ),
			middle_right: get_map_obj( x + 1, y     ),
			lower_left  : get_map_obj( x - 1, y + 1 ),
			lower_center: get_map_obj( x    , y + 1 ),
			lower_right : get_map_obj( x + 1, y + 1 ),
		}
	}

	pub fn make_enclosure_visible( &mut self, x: i32, y: i32, mut q: Query<&mut Visible> )
	{	let mut show_map_obj = | x, y, q: &mut Query<&mut Visible> |
		{	if MAP_INDEX_X.contains( &x ) && MAP_INDEX_Y.contains( &y )
			{	self.stat[ x as usize ][ y as usize ] |= BIT1_SHOW;
				match self.map[ x as usize ][ y as usize ]
				{	MapObj::Wall( Some( id ) ) => q.get_component_mut::<Visible>( id ).unwrap().is_visible = true,
					MapObj::Dot1( Some( id ) ) => q.get_component_mut::<Visible>( id ).unwrap().is_visible = true,
					_ => {}
				};
			}
		};
	
		show_map_obj( x - 1, y - 1, &mut q );
		show_map_obj( x    , y - 1, &mut q );
		show_map_obj( x + 1, y - 1, &mut q );
		show_map_obj( x - 1, y    , &mut q );
		show_map_obj( x    , y    , &mut q );
		show_map_obj( x + 1, y    , &mut q );
		show_map_obj( x - 1, y + 1, &mut q );
		show_map_obj( x    , y + 1, &mut q );
		show_map_obj( x + 1, y + 1, &mut q );
//		show_map_obj( x    , y - 2, &mut q );
//		show_map_obj( x - 2, y    , &mut q );
//		show_map_obj( x + 2, y    , &mut q );
//		show_map_obj( x    , y + 2, &mut q );
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: i32, y: i32 ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * y as f32;
	( x, y )
}

//End of code.