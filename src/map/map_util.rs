use super::*;

//MAPの範囲の定数
use std::ops::RangeInclusive;
pub const RANGE_MAP_X      : RangeInclusive<usize> = 0..= MAP_WIDTH  - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_Y      : RangeInclusive<usize> = 0..= MAP_HEIGHT - 1;	//MAP配列の添え字のレンジ
pub const RANGE_MAP_INNER_X: RangeInclusive<usize> = 1..= MAP_WIDTH  - 2;	//掘削可能なレンジ（最外壁は掘れない）
pub const RANGE_MAP_INNER_Y: RangeInclusive<usize> = 1..= MAP_HEIGHT - 2;	//掘削可能なレンジ（最外壁は掘れない）

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

impl GameMap
{	//指定されたマスのフラグを返す
	pub fn is_passageway ( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_PASSAGEWAY != 0 }
	pub fn is_dead_end   ( &self, grid: MapGrid ) -> bool { self.bits( grid ) & BIT_DEAD_END   != 0 }
	pub fn is_hall       ( &self, grid: MapGrid ) -> bool { ! self.is_wall( grid ) && ! self.is_passageway( grid ) }
}

//End of code