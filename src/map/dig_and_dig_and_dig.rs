use super::*;

impl GameMap
{	//一型迷路：ランダムに掘り進み、壊すと合流する壁は、確率で破壊する
	pub fn dig_and_dig_and_dig( &mut self )
	{	let mut map_xy = self.start_xy;
		map_xy.1 -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

		loop
		{	//ランダムに上下左右へ進む方向を決める
			let ( dx, dy ) = DIRECTION[ self.rng.gen_range( 0..DIRECTION.len() ) ];
			let tmp_x = map_xy.0 + dx;
			let tmp_y = map_xy.1 + dy;

			//上端に達したら迷路完成
			if tmp_y == 0 { break }

			//掘れるなら一歩進む
			if MAP_DIGABLE_X.contains( &tmp_x )
			&& MAP_DIGABLE_Y.contains( &tmp_y )
			&& self.is_dig_or_not( tmp_x, tmp_y )
			{	self.map[ tmp_x as usize ][ tmp_y as usize ] = MapObj::Dot1;
				map_xy = ( tmp_x, tmp_y );
			}
		}
	}

	//さいころを振って、進むか(true)、やり直すか(false)決める
	fn is_dig_or_not( &mut self, x: i32, y: i32 ) -> bool
	{	//そもそも壁じゃないならtrue
		if ! self.is_wall( x, y ) { return true }

		//上下左右のオブジェクトで壁ではないものを数える
		let mut count = 0;
		if ! self.is_wall_upper_center( x, y ) { count += 1 }
		if ! self.is_wall_middle_left ( x, y ) { count += 1 }
		if ! self.is_wall_middle_right( x, y ) { count += 1 }
		if ! self.is_wall_lower_center( x, y ) { count += 1 }

		//２以上なら掘ると道になるので、貫通させるか確率で決める
		let dice = self.rng.gen_range( 0..100 );	//百面ダイスを振って‥‥
		if count == 2 && dice < 70 { return false }	//通路になる   ⇒ 70%の確率でfalse
		if count == 3 && dice < 90 { return false }	//Ｔ字路になる ⇒ 90%の確率でfalse
		if count == 4 && dice < 95 { return false }	//十字路になる ⇒ 95%の確率でfalse

		//壁を掘り進む
		true
	}
}

//End of code.