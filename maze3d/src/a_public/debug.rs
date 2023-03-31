use super::*;

const ASSET_SPRITE_DEBUG_GRID: &str = "sprites/debug_grid.png";            //スプライトファイル
const DEPTH_SPRITE_DEBUG_GRID: f32 = 999.0;                                //スプライト重なり順
const COLOR_SPRITE_DEBUG_GRID: Color = Color::rgba( 1.0, 1.0, 1.0, 0.01 ); //スプライト色(透過)

//スプライトをマス目状に敷き詰める
pub fn spawn_grid
(   mut cmds: Commands,
    asset_svr: Res<AssetServer>,
)
{   let color = COLOR_SPRITE_DEBUG_GRID;
    let custom_size = Some ( Px2d::new( PIXELS_PER_GRID, PIXELS_PER_GRID ) );
    for x in SCREEN_GRIDS_X_RANGE
    {   for y in SCREEN_GRIDS_Y_RANGE
        {   let px2d = Grid::new( x, y ).to_screen_pixel();
            let px3d = px2d.extend( DEPTH_SPRITE_DEBUG_GRID );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( px3d ) )
            .insert( asset_svr.load( ASSET_SPRITE_DEBUG_GRID ) as Handle<Image> )
            ;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

const SIZE_OBJ3D_DEBUG_PLANE : f32 = 5.0; //3Dオブジェクト 拡大率
const SIZE_OBJ3D_DEBUG_CUBE  : f32 = 1.0; //3Dオブジェクト 拡大率
const COLOR_OBJ3D_DEBUG_PLANE: Color = Color::rgb( 0.3, 0.5, 0.3 ); //3Dオブジェクト 色
const COLOR_OBJ3D_DEBUG_CUBE : Color = Color::rgb( 0.8, 0.7, 0.6 ); //3Dオブジェクト 色

//3Dオブジェクトの表示
pub fn spawn_obj3d
(   mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   cmds.spawn( PbrBundle::default() )
    .insert( Transform::from_translation( Px3d::ZERO ) )
    .insert( meshes.add( shape::Plane::from_size( SIZE_OBJ3D_DEBUG_PLANE ).into() ) )
    .insert( materials.add( COLOR_OBJ3D_DEBUG_PLANE.into() ) )
    ;
    cmds.spawn( PbrBundle::default() )
    .insert( Transform::from_translation( Px3d::Y / 2.0 ) )
    .insert( meshes.add( shape::Cube::new( SIZE_OBJ3D_DEBUG_CUBE ).into() ) )
    .insert( materials.add( COLOR_OBJ3D_DEBUG_CUBE.into() ) )
    ;
}

//End of code.