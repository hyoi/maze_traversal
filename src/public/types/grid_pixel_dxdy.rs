use super::*;

//glamの型に別名を付ける
//　※bevyがre-exportしているので use glam::{ IVec2, Vec2 }; は不要
pub type ScreenGrid  = IVec2; //画面のGrid座標
pub type ScreenPixel =  Vec2; //画面のPixel座標

//ScreenGridは外部crateの型(の別名)だから直接 impl ScreenGrid {…} できない。(オーファンルール)
//なのでトレイトを仲介してメソッドを追加する。
pub trait IntoScreenPixel
{   fn into_pixel( self ) -> ScreenPixel;
}
impl IntoScreenPixel for ScreenGrid
{   //ScreenGrid座標からScreenPixel座標を算出する
    fn into_pixel( self ) -> ScreenPixel
    {   let x = ( PIXELS_PER_GRID - SCREEN_PIXELS_WIDTH  ) / 2.0 + PIXELS_PER_GRID * self.x as f32;
        let y = ( SCREEN_PIXELS_HEIGHT - PIXELS_PER_GRID ) / 2.0 - PIXELS_PER_GRID * self.y as f32;
        ScreenPixel::new( x, y )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//四方を表す列挙型
#[derive( Debug, Copy, Clone, PartialEq, Eq )]
pub enum DxDy { Up, Down, Right, Left }

////////////////////////////////////////////////////////////////////////////////////////////////////

//ScreenGridとDxDyを加算できるよう演算子をオーバーロードする
use std::ops::*;

//ScreenGrid = ScreenGrid + DxDy
impl Add<DxDy> for ScreenGrid
{   type Output = ScreenGrid;
    fn add( mut self, dxdy: DxDy ) -> ScreenGrid
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
        self
    }
}

//ScreenGrid = ScreenGrid + &DxDy
impl Add<&DxDy> for ScreenGrid
{   type Output = ScreenGrid;
    fn add( mut self, dxdy: &DxDy ) -> ScreenGrid
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
        self
    }
}

//ScreenGrid += DxDy
impl AddAssign<DxDy> for ScreenGrid
{   fn add_assign( &mut self, dxdy: DxDy )
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
    }
}

//ScreenGrid += &DxDy
impl AddAssign<&DxDy> for ScreenGrid
{   fn add_assign( &mut self, dxdy: &DxDy )
    {   match dxdy
        {   DxDy::Up    => { self.y -= 1; }
            DxDy::Down  => { self.y += 1; }
            DxDy::Right => { self.x += 1; }
            DxDy::Left  => { self.x -= 1; }
        }
    }
}

//End of code.

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg( test )]
mod tests
{   #[test]
    fn grid_add_dxdy()
    {   use super::*;

        let grid = ScreenGrid::default();
        let mut grid_up    = grid;
        let mut grid_down  = grid;
        let mut grid_right = grid;
        let mut grid_left  = grid;
        let dxdy_up    = DxDy::Up;
        let dxdy_down  = DxDy::Down;
        let dxdy_right = DxDy::Right;
        let dxdy_left  = DxDy::Left;

        //ScreenGrid += DxDy
        grid_up    += dxdy_up;
        grid_down  += dxdy_down;
        grid_right += dxdy_right;
        grid_left  += dxdy_left;
        assert_eq!( grid_up   , ScreenGrid::new(  0, -1 ) );
        assert_eq!( grid_down , ScreenGrid::new(  0,  1 ) );
        assert_eq!( grid_right, ScreenGrid::new(  1,  0 ) );
        assert_eq!( grid_left , ScreenGrid::new( -1,  0 ) );

        //ScreenGrid = ScreenGrid + DxDy
        assert_eq!( grid_up   , grid + dxdy_up    );
        assert_eq!( grid_down , grid + dxdy_down  );
        assert_eq!( grid_right, grid + dxdy_right );
        assert_eq!( grid_left , grid + dxdy_left  );

        //ScreenGrid += &DxDy
        let ref_dxdy_up    = &dxdy_up;
        let ref_dxdy_down  = &dxdy_down;
        let ref_dxdy_right = &dxdy_right;
        let ref_dxdy_left  = &dxdy_left;
        grid_up    += ref_dxdy_down;
        grid_down  += ref_dxdy_up;
        grid_right += ref_dxdy_left;
        grid_left  += ref_dxdy_right;
        assert_eq!( grid_up   , ScreenGrid::new( 0, 0 ) );
        assert_eq!( grid_down , ScreenGrid::new( 0, 0 ) );
        assert_eq!( grid_right, ScreenGrid::new( 0, 0 ) );
        assert_eq!( grid_left , ScreenGrid::new( 0, 0 ) );

        //ScreenGrid = ScreenGrid + &DxDy
        assert_eq!( grid_up   , grid + dxdy_up    + ref_dxdy_down  );
        assert_eq!( grid_down , grid + dxdy_down  + ref_dxdy_up    );
        assert_eq!( grid_right, grid + dxdy_right + ref_dxdy_left  );
        assert_eq!( grid_left , grid + dxdy_left  + ref_dxdy_right );
    }
}

//End of test code.