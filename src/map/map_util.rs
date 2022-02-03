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

	//配列の値を返す
	pub fn map ( &self, grid: MapGrid ) -> MapObj { self.map [ grid.x ][ grid.y ] }
	pub fn bits( &self, grid: MapGrid ) -> usize  { self.bits[ grid.x ][ grid.y ] }
	pub fn coin( &self, grid: MapGrid ) -> usize  { self.coin[ grid.x ][ grid.y ] }

	//配列の値をセットする
	pub fn set_mapobj( &mut self, grid: MapGrid, obj : MapObj ) { self.map [ grid.x ][ grid.y ] = obj  }
	pub fn set_coin  ( &mut self, grid: MapGrid, coin: usize  ) { self.coin[ grid.x ][ grid.y ] = coin }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//壁判定のメソッド: is_wall()系 -> true: 壁である、false: 壁ではない
impl GameMap
{	pub fn is_wall( &self, grid: MapGrid ) -> bool
	{	if ! RANGE_MAP_X.contains( &grid.x )
		|| ! RANGE_MAP_Y.contains( &grid.y ) { return true } //配列の添字外は壁
		matches!( self.map( grid ), MapObj::Wall )
	}

	pub fn is_wall_upper_left   ( &self, grid: MapGrid ) -> bool { self.is_wall( grid + UP   + LEFT  ) }
	pub fn is_wall_upper_center ( &self, grid: MapGrid ) -> bool { self.is_wall( grid + UP           ) }
	pub fn is_wall_upper_right  ( &self, grid: MapGrid ) -> bool { self.is_wall( grid + UP   + RIGHT ) }
	pub fn is_wall_middle_left  ( &self, grid: MapGrid ) -> bool { self.is_wall( grid        + LEFT  ) }
	pub fn is_wall_middle_right ( &self, grid: MapGrid ) -> bool { self.is_wall( grid        + RIGHT ) }
	pub fn is_wall_lower_left   ( &self, grid: MapGrid ) -> bool { self.is_wall( grid + DOWN + LEFT  ) }
	pub fn is_wall_lower_center ( &self, grid: MapGrid ) -> bool { self.is_wall( grid + DOWN         ) }
	pub fn is_wall_lower_right  ( &self, grid: MapGrid ) -> bool { self.is_wall( grid + DOWN + RIGHT ) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//MAPのマスの状態の制御に使うbit
const BIT_IS_PASSAGEWAY: usize = 0b0010;
const BIT_IS_DEAD_END  : usize = 0b0100;

impl GameMap
{	//指定されたマスのフラグを返す
	pub fn is_passageway ( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_IS_PASSAGEWAY != 0 }
	pub fn is_dead_end   ( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_IS_DEAD_END   != 0 }
	pub fn is_hall       ( &self, grid: MapGrid ) -> bool { ! self.is_wall( grid ) && ! self.is_passageway( grid ) }

	//指定されたマスのフラグを立てる
	pub fn set_flag_passageway ( &mut self, grid: MapGrid ) { self.bits[ grid.x ][ grid.y ] |= BIT_IS_PASSAGEWAY; }
	pub fn set_flag_dead_end   ( &mut self, grid: MapGrid ) { self.bits[ grid.x ][ grid.y ] |= BIT_IS_DEAD_END;   }
}

//End of code