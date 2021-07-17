use super::*;

//二次元配列の添え字から画面座標を算出する
pub fn conv_sprite_coordinates( x: i32, y: i32 ) -> ( f32, f32 )
{	let x = ( PIXEL_PER_GRID - SCREEN_WIDTH  ) / 2.0 + PIXEL_PER_GRID * x as f32;
	let y = ( SCREEN_HEIGHT - PIXEL_PER_GRID ) / 2.0 - PIXEL_PER_GRID * y as f32 - PIXEL_PER_GRID;
	( x, y )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//壁判定のメソッド: is_wall()系 -> true: 壁である、false: 壁ではない
impl GameMap
{	pub fn is_wall( &self, x: i32, y: i32 ) -> bool
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

//MAPの範囲の定数
use std::ops::RangeInclusive;
pub const MAP_INDEX_X  : RangeInclusive<i32> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const MAP_INDEX_Y  : RangeInclusive<i32> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const MAP_DIGABLE_X: RangeInclusive<i32> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const MAP_DIGABLE_Y: RangeInclusive<i32> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

//MAP座標の上下左右を表す定数
pub const UP   : ( i32, i32 ) = (  0, -1 );
pub const LEFT : ( i32, i32 ) = ( -1,  0 );
pub const RIGHT: ( i32, i32 ) = (  1,  0 );
pub const DOWN : ( i32, i32 ) = (  0,  1 );
pub const DIRECTION: [ ( i32, i32 ); 4 ] = [ UP, LEFT, RIGHT, DOWN ];

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの状態の制御に使うbit
pub const BIT_ALL_CLEAR: usize = 0;
const BIT_IS_VISIBLE   : usize = 0b0001;
const BIT_IS_PASSAGEWAY: usize = 0b0010;
const BIT_IS_DEAD_END  : usize = 0b0100;

impl GameMap
{	//指定されたマスのフラグを返す
	#[allow(dead_code)]
	pub fn is_visible    ( &self, x: i32, y: i32 ) -> bool { self.stat[ x as usize ][ y as usize ] & BIT_IS_VISIBLE    != 0 }
	pub fn is_passageway ( &self, x: i32, y: i32 ) -> bool { self.stat[ x as usize ][ y as usize ] & BIT_IS_PASSAGEWAY != 0 }
	pub fn is_dead_end   ( &self, x: i32, y: i32 ) -> bool { self.stat[ x as usize ][ y as usize ] & BIT_IS_DEAD_END   != 0 }

	//指定されたマスのフラグを立てる
	pub fn set_flag_passageway ( &mut self, x: i32, y: i32 ) { self.stat[ x as usize ][ y as usize ] |= BIT_IS_PASSAGEWAY; }
	pub fn set_flag_dead_end   ( &mut self, x: i32, y: i32 ) { self.stat[ x as usize ][ y as usize ] |= BIT_IS_DEAD_END;   }

	//指定されたマスのVISIBLEフラグを立ててスプライトを可視化する
	pub fn show( &mut self, x: i32, y: i32, q: &mut Query<&mut Visible> )
	{	if ! MAP_INDEX_X.contains( &x ) || ! MAP_INDEX_Y.contains( &y ) { return }

		self.stat[ x as usize ][ y as usize ] |= BIT_IS_VISIBLE;
		if let MapObj::Wall( Some( id ) ) = self.map[ x as usize ][ y as usize ]
		{	q.get_component_mut::<Visible>( id ).unwrap().is_visible = true;
		}
	}
}

//End of code