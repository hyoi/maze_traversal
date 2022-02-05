use super::*;

impl GameMap
{	//一型迷路：ランダムに掘り進み、壊すと貫通する壁は、確率で破壊する
	pub fn dig_and_dig_and_dig( &mut self )
	{	let mut grid = self.start();
		grid.y -= 1; //maze.start_xyの直上(y-1)がトンネル掘りの開始座標

		loop
		{	//ランダムに上下左右へ進む方向を決める
			let next = grid + FOUR_SIDES[ self.rng().gen_range( 0..FOUR_SIDES.len() ) ];

			//上端に達したら迷路完成
			if next.y == 0 { break }

			//掘れるなら一歩進む
			if RANGE_MAP_INNER_X.contains( &next.x )
			&& RANGE_MAP_INNER_Y.contains( &next.y ) && self.dig_or_not( next )
			{	*self.mapobj_mut( next ) = MapObj::Passage;	//道を掘る
				grid = next;
			}
		}
	}

	//さいころを振って、進むか(true)、やり直すか(false)決める
	fn dig_or_not( &mut self, grid: MapGrid ) -> bool
	{	//そもそも壁じゃないならtrue
		if ! self.is_wall( grid ) { return true }

		//上下左右のオブジェクトで壁ではないものを数える
		let mut count = 0;
		if ! self.is_wall( grid + UP    ) { count += 1 }
		if ! self.is_wall( grid + LEFT  ) { count += 1 }
		if ! self.is_wall( grid + RIGHT ) { count += 1 }
		if ! self.is_wall( grid + DOWN  ) { count += 1 }

		//２以上なら貫通させるか確率で決める
		let dice = self.rng().gen_range( 0..100 );	//百面ダイスを振って‥‥
		if count == 2 && dice < 70 { return false }	//通路になる   ⇒ 70%の確率でやめよう
		if count == 3 && dice < 90 { return false }	//Ｔ字路になる ⇒ 90%の確率でやめよう
		if count == 4 && dice < 95 { return false }	//十字路になる ⇒ 95%の確率でやめよう

		//掘ろう
		true
	}
}

//End of code.