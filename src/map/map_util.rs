use super::*;

//MAPの範囲の定数
use std::ops::RangeInclusive;
pub const RANGE_MAP_X      : RangeInclusive<usize> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_Y      : RangeInclusive<usize> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_INNER_X: RangeInclusive<usize> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const RANGE_MAP_INNER_Y: RangeInclusive<usize> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

////////////////////////////////////////////////////////////////////////////////////////////////////

impl GameMap
{	//配列を初期化する
	pub fn clear_map( &mut self )
	{	self.map .iter_mut().for_each( | x | x.fill( MapObj::Wall ) );
		self.bits.iter_mut().for_each( | x | x.fill( 0            ) );
		self.coin.iter_mut().for_each( | x | x.fill( 0            ) );
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//壁判定のメソッド: is_wall()系 -> true: 壁である、false: 壁ではない
impl GameMap
{	pub fn is_wall( &self, x: usize, y: usize ) -> bool
	{	if ! RANGE_MAP_X.contains( &x ) || ! RANGE_MAP_Y.contains( &y ) { return true } //配列の添字外は壁
		matches!( self.map[ x ][ y ], MapObj::Wall )
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

//MAPのマスの状態の制御に使うbit
const BIT_IS_PASSAGEWAY: usize = 0b0010;
const BIT_IS_DEAD_END  : usize = 0b0100;

impl GameMap
{	//指定されたマスのフラグを返す
	pub fn is_passageway ( &self, x: usize, y: usize ) -> bool { self.bits[ x ][ y ] & BIT_IS_PASSAGEWAY != 0 }
	pub fn is_dead_end   ( &self, x: usize, y: usize ) -> bool { self.bits[ x ][ y ] & BIT_IS_DEAD_END   != 0 }
	pub fn is_hall       ( &self, x: usize, y: usize ) -> bool { ! self.is_wall( x, y ) && ! self.is_passageway( x, y ) }

	//指定されたマスのフラグを立てる
	pub fn set_flag_passageway ( &mut self, x: usize, y: usize ) { self.bits[ x ][ y ] |= BIT_IS_PASSAGEWAY; }
	pub fn set_flag_dead_end   ( &mut self, x: usize, y: usize ) { self.bits[ x ][ y ] |= BIT_IS_DEAD_END;   }
}

//End of code