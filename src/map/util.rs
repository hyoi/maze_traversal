use super::*;

impl GameMap
{	//is_wall()系 -> true: 壁である、false: 壁ではない
	pub fn is_wall( &self, x: i32, y: i32 ) -> bool
	{	if ! MAP_INDEX_X.contains( &x ) || ! MAP_INDEX_Y.contains( &y ) { return true } //配列の添字外は壁
		matches!( self.map[ x as usize ][ y as usize ], MapObj::Wall(_) )
	}
	pub fn is_wall_upper_left   ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x - 1, y - 1 ) }
	pub fn is_wall_upper_center ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x    , y - 1 ) }
	pub fn is_wall_upper_right  ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x + 1, y - 1 ) }
	pub fn is_wall_middle_left  ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x - 1, y     ) }
	pub fn is_wall_middle_right ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x + 1, y     ) }
	pub fn is_wall_lower_left   ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x - 1, y + 1 ) }
	pub fn is_wall_lower_center ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x    , y + 1 ) }
	pub fn is_wall_lower_right  ( &self, x: i32, y: i32 ) -> bool { self.is_wall( x + 1, y + 1 ) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのレンジ定数
use std::ops::RangeInclusive;
pub const MAP_INDEX_X  : RangeInclusive<i32> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const MAP_INDEX_Y  : RangeInclusive<i32> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const MAP_DIGABLE_X: RangeInclusive<i32> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const MAP_DIGABLE_Y: RangeInclusive<i32> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの状態の制御に使うbit
pub const BIT_ALL_CLEAR  : usize = 0b0000;

const BIT1_IS_VISIBLE: usize = 0b0001;
const BIT1_SHOW      : usize = 0b0001;
const BIT1_HIDE      : usize = 0b0000;

const BIT2_PASSAGEWAY: usize = 0b0010;
const BIT3_DAED_END  : usize = 0b0100;
const BIT4_ALCOVE    : usize = 0b1000;

impl GameMap
{	//true: 見せる、false: 見せない
	pub fn is_visible( &self, x: i32, y: i32 ) -> bool
	{	self.stat[ x as usize ][ y as usize ] & BIT1_IS_VISIBLE != 0
	}
}

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
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: i32, y: i32 ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * y as f32 - PIXEL_PER_GRID;
	( x, y )
}

//End of code.