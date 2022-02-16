use super::*;

//Pluginの手続き
pub struct PluginUiHeaderLeft;
impl Plugin for PluginUiHeaderLeft
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system( update_ui_header_left  )			// UIの表示を更新
		//==========================================================================================
		.add_system_set									// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )		// ＜on_exit()＞
				.with_system( spawn_sprite_hp_gauge )	// HPゲージを作成
		)
		//==========================================================================================
		.add_system_set									// ＜GameState::Over＞
		(	SystemSet::on_exit( GameState::Over )		// ＜on_exit()＞
				.with_system( init_record )				// GameOver後の初期化
		)
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct UiHeaderLeft;
pub const UI_HEADER_LEFT: [ MessageSect; 2 ] =
[	( " HP ", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.8, Color::ORANGE ),
	( ""    , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];

//HP GAUGE
#[derive(Component)]
struct HpGauge;
const GAUGE_RECTANGLE: ( f32, f32, f32, f32 ) = 
(	PIXEL_PER_GRID *  7.4 - SCREEN_WIDTH  / 2.0,	//X軸：画面中央からやや左より
	PIXEL_PER_GRID * -0.7 + SCREEN_HEIGHT / 2.0,	//Y軸：画面上端からやや下がった位置
	PIXEL_PER_GRID * 11.8,							//幅
	PIXEL_PER_GRID *  0.2,							//高さ
);
const SPRITE_DEPTH_GAUGE: f32 = 30.0;

////////////////////////////////////////////////////////////////////////////////////////////////////

//HP GAUGEのスプライトを作成する
fn spawn_sprite_hp_gauge( mut cmds: Commands )
{	let ( x, y, w, h ) = GAUGE_RECTANGLE;

	let sprite = Sprite
	{	color: Color::GREEN,
		custom_size: Some( Vec2::new( w, h ) ),
		..Default::default()
	};
	let transform = Transform::from_translation( Vec3::new( x, y, SPRITE_DEPTH_GAUGE ) );

	let bundle = SpriteBundle { sprite, transform, ..Default::default() };
	cmds.spawn_bundle( bundle ).insert( HpGauge );
}

//HP GAGEを更新する(上端左)
fn update_ui_header_left
(	mut q_ui: Query< &mut Text, With<UiHeaderLeft>>,
	mut q_gauge: Query<( &mut Transform, &mut Sprite ), With<HpGauge>>,
	o_record: Option<Res<Record>>,
)
{	if let Ok( mut ui ) = q_ui.get_single_mut()
	{	let hp = match o_record
		{	Some( record ) =>
			{	let hp = record.hp.max( 0.0 );
				let ( mut transform, mut sprite ) = q_gauge.get_single_mut().unwrap();

				//スプライトの幅のスケールを縮小する。
				//すると両端が縮むので、スプライトを左に移動して右端が縮んだように見せる
				let scale_width = &mut transform.scale[ 0 ];
				if 	*scale_width > 0.0
				{	*scale_width = ( hp / MAX_HP ).max( 0.0 );
					let ( x, _, w, _ ) = GAUGE_RECTANGLE;
					let translation = &mut transform.translation;
					translation.x = x - ( MAX_HP - hp ) * w / 200.0;	
				}

				//色を変える(緑色⇒黄色⇒赤色)
				let temp = hp / MAX_HP;
				sprite.color = Color::rgb
				(	1.0 - ( temp - 0.6 ).max( 0.0 ) * 2.0,
					( temp.min( 0.7 ) * 2.0 - 0.4 ).max( 0.0 ),
					0.0
				);
				hp.to_string()
			},
			None => NA_STR3.to_string()
		};
		ui.sections[ 1 ].value = hp;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//GameOverのon_exit()でゲームを初期化する
fn init_record
(	mut q: Query<( &mut Transform, &mut Sprite ), With<HpGauge>>,
	mut record: ResMut<Record>,
)
{	//スコア等の初期化
	*record = Record::default();

	//HP GAGEのスプライトを初期化
	if let Ok ( ( mut transform, mut sprite ) ) = q.get_single_mut()
	{	let ( x, y, w, h ) = GAUGE_RECTANGLE;
		transform.scale[ 0 ] = 100.0;
		transform.translation.x = x;
		transform.translation.y = y;
		sprite.custom_size = Some( Vec2::new( w, h ) );
		sprite.color = Color::GREEN;
	}
}

//End of code.