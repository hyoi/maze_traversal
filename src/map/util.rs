use super::*;

//壁判定のメソッド: is_wall()系 -> true: 壁である、false: 壁ではない
impl GameMap
{	pub fn is_wall( &self, x: usize, y: usize ) -> bool
	{	if ! MAP_INDEX_X.contains( &x ) || ! MAP_INDEX_Y.contains( &y ) { return true } //配列の添字外は壁
		matches!( self.map[ x as usize ][ y as usize ], MapObj::Wall(_) )
	}
	pub fn is_wall_upper_left   ( &self, x: usize, y: usize ) -> bool { self.is_wall( x - 1, y - 1 ) }
	pub fn is_wall_upper_center ( &self, x: usize, y: usize ) -> bool { self.is_wall( x    , y - 1 ) }
	pub fn is_wall_upper_right  ( &self, x: usize, y: usize ) -> bool { self.is_wall( x + 1, y - 1 ) }
	pub fn is_wall_middle_left  ( &self, x: usize, y: usize ) -> bool { self.is_wall( x - 1, y     ) }
	pub fn is_wall_middle_right ( &self, x: usize, y: usize ) -> bool { self.is_wall( x + 1, y     ) }
	pub fn is_wall_lower_left   ( &self, x: usize, y: usize ) -> bool { self.is_wall( x - 1, y + 1 ) }
	pub fn is_wall_lower_center ( &self, x: usize, y: usize ) -> bool { self.is_wall( x    , y + 1 ) }
	pub fn is_wall_lower_right  ( &self, x: usize, y: usize ) -> bool { self.is_wall( x + 1, y + 1 ) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPの範囲の定数
use std::ops::RangeInclusive;
pub const MAP_INDEX_X  : RangeInclusive<usize> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const MAP_INDEX_Y  : RangeInclusive<usize> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const MAP_DIGABLE_X: RangeInclusive<usize> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const MAP_DIGABLE_Y: RangeInclusive<usize> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

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
const BIT_IS_EVENT_DONE: usize = 0b1000;

impl GameMap
{	//指定されたマスのフラグを返す
	#[allow(dead_code)]
	pub fn is_visible    ( &self, x: usize, y: usize ) -> bool { self.stat[ x ][ y ] & BIT_IS_VISIBLE    != 0 }
	pub fn is_passageway ( &self, x: usize, y: usize ) -> bool { self.stat[ x ][ y ] & BIT_IS_PASSAGEWAY != 0 }
	pub fn is_dead_end   ( &self, x: usize, y: usize ) -> bool { self.stat[ x ][ y ] & BIT_IS_DEAD_END   != 0 }
	pub fn is_event_done ( &self, x: usize, y: usize ) -> bool { self.stat[ x ][ y ] & BIT_IS_EVENT_DONE != 0 }

	//指定されたマスのフラグを立てる
	pub fn set_flag_passageway ( &mut self, x: usize, y: usize ) { self.stat[ x ][ y ] |= BIT_IS_PASSAGEWAY; }
	pub fn set_flag_dead_end   ( &mut self, x: usize, y: usize ) { self.stat[ x ][ y ] |= BIT_IS_DEAD_END;   }
	pub fn set_flag_event_done ( &mut self, x: usize, y: usize ) { self.stat[ x ][ y ] |= BIT_IS_EVENT_DONE; }

	//指定されたマスのVISIBLEフラグを立ててスプライトを可視化する
	pub fn show( &mut self, x: usize, y: usize, q: &mut Query<&mut Visibility> )
	{	if ! MAP_INDEX_X.contains( &x ) || ! MAP_INDEX_Y.contains( &y ) { return }

		self.stat[ x ][ y ] |= BIT_IS_VISIBLE;
		if let MapObj::Wall( Some( id ) ) = self.map[ x ][ y ]
		{	q.get_component_mut::<Visibility>( id ).unwrap().is_visible = true;
		}
	}
}

//End of code