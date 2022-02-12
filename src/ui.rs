use super::*;

//Pluginの手続き
pub struct PluginUi;
impl Plugin for PluginUi
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.insert_resource( DbgOptResUI )					// マーカーResource
		//------------------------------------------------------------------------------------------
		.add_system( update_ui_upper_left )				// UIの表示を更新
		.add_system( update_ui_upper_right )			// UIの表示を更新
		.add_system( update_ui_lower_left )				// UIの表示を更新
		//==========================================================================================
		.add_system_set									// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )		// ＜on_exit()＞
				.with_system( spawn_text_ui_message )	// assetesプリロード後にUIを非表示で生成
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

//NA文字列
const NA_STR3: &str = "---";
const NA_STR5: &str = "-----";
const NA_STR4: &str = "--.--";

//HP GAUGE
#[derive(Component)]
struct HpGauge;
const GAUGE_RECTANGLE: ( f32, f32, f32, f32 ) = 
(	PIXEL_PER_GRID *  8.9 - SCREEN_WIDTH  / 2.0,	//X軸：画面中央からやや左より
	PIXEL_PER_GRID * -0.7 + SCREEN_HEIGHT / 2.0,	//Y軸：画面上端からやや下がった位置
	PIXEL_PER_GRID * 15.0,							//幅
	PIXEL_PER_GRID *  0.2,							//高さ
);
const SPRITE_DEPTH_GAUGE: f32 = 30.0;

////////////////////////////////////////////////////////////////////////////////////////////////////

//テキストUIを配置する
fn spawn_text_ui_message( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//中央に表示するtext
	let mut pause_text = text_ui( &MESSAGE_PAUSE, &asset_svr );
	let mut clear_text = text_ui( &MESSAGE_CLEAR, &asset_svr );
	let mut over_text  = text_ui( &MESSAGE_OVER , &asset_svr );
	let mut event_text = text_ui( &MESSAGE_EVENT, &asset_svr );
	pause_text.visibility.is_visible = false;
	clear_text.visibility.is_visible = false;
	over_text.visibility.is_visible  = false;
	event_text.visibility.is_visible = false;

	//上端・下端に表示するtext
	let mut ui_upper_left   = text_ui( &UI_UPPER_LEFT  , &asset_svr );
//	let mut ui_upper_center = text_ui( &UI_UPPER_CENTER, &asset_svr );
	let mut ui_upper_right  = text_ui( &UI_UPPER_RIGHT , &asset_svr );
	let mut ui_lower_left   = text_ui( &UI_LOWER_LEFT  , &asset_svr );
	let mut ui_lower_center = text_ui( &UI_LOWER_CENTER, &asset_svr );
	let mut ui_lower_right  = text_ui( &UI_LOWER_RIGHT , &asset_svr );

	ui_upper_left.style.align_self   = AlignSelf::FlexStart;
//	ui_upper_center.style.align_self = AlignSelf::Center;
	ui_upper_right.style.align_self  = AlignSelf::FlexEnd;
	ui_lower_left.style.align_self   = AlignSelf::FlexStart;
	ui_lower_center.style.align_self = AlignSelf::Center;
	ui_lower_right.style.align_self  = AlignSelf::FlexEnd;

	ui_upper_left.text.alignment.horizontal   = HorizontalAlign::Left;
//	ui_upper_center.text.alignment.horizontal = HorizontalAlign::Center;
	ui_upper_right.text.alignment.horizontal  = HorizontalAlign::Right;
	ui_lower_left.text.alignment.horizontal   = HorizontalAlign::Left;
	ui_lower_center.text.alignment.horizontal = HorizontalAlign::Center;
	ui_lower_right.text.alignment.horizontal  = HorizontalAlign::Right;


	//レイアウト用の隠しフレームを作る
	let per100 = Val::Percent( 100.0 );
	let center_frame = hidden_frame( Style
	{	size           : Size::new( per100, per100 ),
		position_type  : PositionType::Absolute,
		justify_content: JustifyContent::Center,
		align_items    : AlignItems::Center,
		..Default::default()
	} );
	let upper_frame = hidden_frame( Style
	{	size           : Size::new( Val::Px( SCREEN_WIDTH ), Val::Px( SCREEN_HEIGHT ) ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexEnd, //画面の上端
		..Default::default()
	} );
	let lower_frame = hidden_frame( Style
	{	size           : Size::new( Val::Px( SCREEN_WIDTH ), Val::Px( SCREEN_HEIGHT ) ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexStart, //画面の下端
		..Default::default()
	} );

	//隠しフレームの上に子要素を作成する
	cmds.spawn_bundle( center_frame ).with_children( | cmds |
	{	cmds.spawn_bundle( pause_text ).insert( MessagePause );
		cmds.spawn_bundle( clear_text ).insert( MessageClear );
		cmds.spawn_bundle( over_text  ).insert( MessageOver  );
		cmds.spawn_bundle( event_text ).insert( MessageEvent );

		cmds.spawn_bundle( upper_frame ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_upper_left   ).insert( UiUpperLeft   );
//			cmds.spawn_bundle( ui_upper_center ).insert( UiUpperCenter );
			cmds.spawn_bundle( ui_upper_right  ).insert( UiUpperRight  );
		} );

		cmds.spawn_bundle( lower_frame ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_lower_left   ).insert( UiLowerLeft   );
			cmds.spawn_bundle( ui_lower_center ).insert( UiLowerCenter );
			cmds.spawn_bundle( ui_lower_right  ).insert( UiLowerRight  );
		} );
	} );

	//おまけ
	let pixel = MapGrid { x: GRID_WIDTH - 4, y: GRID_HEIGHT - 2 }.into_pixel();
	let custom_size = Some( Vec2::new( PIXEL_PER_GRID, PIXEL_PER_GRID ) );
	cmds.spawn_bundle( SpriteBundle::default() )
		.insert( Sprite { custom_size, ..Default::default() } )
		.insert( asset_svr.load( IMAGE_SPRITE_KANI ) as Handle<Image> )
		.insert( Transform::from_translation( Vec3::new( pixel.x, pixel.y, 100.0 ) ) );
}

//TextBundleを作る
fn text_ui( message: &[ MessageSect ], asset_svr: &Res<AssetServer> ) -> TextBundle
{	let mut sections = Vec::new();
	message.iter().for_each
	(	| ( line, file, size, color ) |
		{	let value = line.to_string();
			let style = TextStyle
			{	font     : asset_svr.load( *file ),
				font_size: *size,
				color    : *color
			};
			sections.push( TextSection { value, style } );
		}
	);

	let alignment = TextAlignment
	{	vertical  : VerticalAlign::Center,
		horizontal: HorizontalAlign::Center,
	};
	let position_type = PositionType::Absolute;

	let text  = Text { sections, alignment };
	let style = Style { position_type, ..Default::default() };
	TextBundle { text, style, ..Default::default() }
}

//レイアウト用に隠しフレームを作る
fn hidden_frame( style: Style ) -> NodeBundle
{	let visibility = Visibility { is_visible: false };
	NodeBundle { style, visibility, ..Default::default() }
}

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
fn update_ui_upper_left
(	mut q_ui: Query< &mut Text, With<UiUpperLeft>>,
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

//スコアとステージの表示を更新する
fn update_ui_upper_right
(	mut q: Query< &mut Text, With<UiUpperRight>>,
	o_record: Option<Res<Record>>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	let ( score, stage ) = o_record.map_or
		(	( NA_STR5.to_string(), NA_STR5.to_string() ),
			| x | ( x.score.to_string(), x.stage.to_string() )
		);
		ui.sections[ 0 ].value = score;
		ui.sections[ 2 ].value = stage;
	}
}

//下端の情報表示を更新する(左)
fn update_ui_lower_left
(	mut q: Query< &mut Text, With<UiLowerLeft>>,
	diag: Res<Diagnostics>,
)
{	if let Ok( mut ui ) = q.get_single_mut()
	{	let fps_avr = diag.get( FrameTimeDiagnosticsPlugin::FPS ).map_or
		(	NA_STR4.to_string(),
			| fps | match fps.average()
			{	Some( avg ) => format!( "{:2.2}", avg ),
				None        => NA_STR4.to_string()
			}
		);
		ui.sections[ 1 ].value = fps_avr;
	}
}

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