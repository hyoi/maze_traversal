use super::*;

//Pluginの手続き
pub struct PluginUi;
impl Plugin for PluginUi
{	fn build( &self, app: &mut AppBuilder )
	{	app
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )						// ＜on_exit()＞
				.with_system( spawn_text_ui_message.system() )			// assetesプリロード後にUIを非表示で生成
		)
		//------------------------------------------------------------------------------------------
		.add_system_set													// ＜GameState::Clear＞
		(	SystemSet::on_enter( GameState::Clear )						// ＜on_enter()＞
				.with_system( show_clear_message.system() )				// CLEARメッセージを表示する
		)
		.add_system_set													// ＜GameState::Clear＞
		(	SystemSet::on_update( GameState::Clear )					// ＜on_update()＞
				.with_system( change_state_after_countdown.system() )	// CD完了⇒GameState::Startへ
		)
		.add_system_set													// ＜GameState::Clear＞
		(	SystemSet::on_exit( GameState::Clear )						// ＜on_exit()＞
				.with_system( hide_clear_message.system() )				// CLEARメッセージを隠す
		)
		//------------------------------------------------------------------------------------------
		.add_startup_system( spawn_hp_gauge_sprite.system() )			//
		.add_system( update_ui_upper_left.system() )					// 情報を更新
		.add_system( update_ui_lower_left.system() )					// 情報を更新
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//定義と定数

//Font file
pub const FONT_MESSAGE_TEXT: &str = "fonts/Orbitron-Black.ttf";
pub const FONT_TITLE_TEXT  : &str = "fonts/ReggaeOne-Regular.ttf";

//Text UI
type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

pub struct MessagePause;
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 5.0, Color::SILVER ),
];

struct MessageClear;
const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "C L E A R !!\n"   , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "Next stage...\n\n", FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( ""                 , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 4.0, Color::WHITE ),
];

pub struct MessageEvent;
const MESSAGE_EVENT: [ MessageSect; 3 ] =
[	( "E V E N T !!\n", FONT_TITLE_TEXT, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "戦闘中...\n\n"  , FONT_TITLE_TEXT, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( "Hit SPACE Kry!", FONT_TITLE_TEXT, PIXEL_PER_GRID * 2.5, Color::GOLD ),
];

const NA_STR3: &str = "---";

struct UiUpperRight;
const UI_UPPER_RIGHT: [ MessageSect; 2 ] =
[	( APP_TITLE, FONT_TITLE_TEXT, PIXEL_PER_GRID * 1.3, Color::ORANGE ),
	( "迷路踏破", FONT_TITLE_TEXT, PIXEL_PER_GRID * 1.6, Color::WHITE  ),
];

struct UiUpperLeft;
const UI_UPPER_LEFT: [ MessageSect; 2 ] =
[	( "HP "  , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.9, Color::ORANGE ),
	( NA_STR3, FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];

struct UiLowerLeft;
const UI_LOWER_LEFT: [ MessageSect; 2 ] =
[	( "FPS " , FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 0.9, Color::ORANGE ),
	( NA_STR3, FONT_MESSAGE_TEXT, PIXEL_PER_GRID * 1.0, Color::WHITE  ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//テキストUIを配置する
fn spawn_text_ui_message( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//中央に表示するtext
	let mut pause_text = text_messsage( &MESSAGE_PAUSE, &asset_svr );
	let mut clear_text = text_messsage( &MESSAGE_CLEAR, &asset_svr );
	let mut event_text = text_messsage( &MESSAGE_EVENT, &asset_svr );
	pause_text.visible.is_visible = false;	//初期は非表示
	clear_text.visible.is_visible = false;	//初期は非表示
	event_text.visible.is_visible = false;	//初期は非表示

	//上端に表示するtext
	let mut ui_upper_right = text_messsage( &UI_UPPER_RIGHT, &asset_svr );
	ui_upper_right.style.align_self = AlignSelf::FlexEnd;
	ui_upper_right.text.alignment.horizontal = HorizontalAlign::Right;
	let mut ui_upper_left = text_messsage( &UI_UPPER_LEFT, &asset_svr );
	ui_upper_left.style.align_self = AlignSelf::FlexStart;
	ui_upper_left.text.alignment.horizontal = HorizontalAlign::Left;

	//下端に表示するtext
	let mut ui_lower_left = text_messsage( &UI_LOWER_LEFT, &asset_svr );
	ui_lower_left.style.align_self = AlignSelf::FlexStart;
	ui_lower_left.text.alignment.horizontal = HorizontalAlign::Left;

	//隠しフレームの上に子要素を作成する
	cmds.spawn_bundle( hidden_frame_for_centering() ).with_children( | cmds |
	{	cmds.spawn_bundle( pause_text ).insert( MessagePause );
		cmds.spawn_bundle( clear_text ).insert( MessageClear );
		cmds.spawn_bundle( event_text ).insert( MessageEvent );

		cmds.spawn_bundle( hidden_upper_frame() ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_upper_right ).insert( UiUpperRight );
			cmds.spawn_bundle( ui_upper_left  ).insert( UiUpperLeft  );
		} );

		cmds.spawn_bundle( hidden_lower_frame() ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_lower_left ).insert( UiLowerLeft );
		} );

	} );
}

//TextBundleを作る
fn text_messsage( message: &[ MessageSect ], asset_svr: &Res<AssetServer> ) -> TextBundle
{	let mut sections = Vec::new();
	for ( value, file, size, color ) in message.iter()
	{	let value = value.to_string();
		let style = TextStyle
		{	font     : asset_svr.load( *file ),
			font_size: *size,
			color    : *color
		};
		sections.push( TextSection { value, style } );
	}
	let alignment = TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center };
	let text = Text { sections, alignment };
	let style = Style { position_type: PositionType::Absolute, ..Default::default() };
	TextBundle { style, text, ..Default::default() }
}

//中央寄せ用の隠しフレーム
fn hidden_frame_for_centering() -> NodeBundle
{	let per100 = Val::Percent( 100.0 );
	let style = Style
	{	size: Size::new( per100, per100 ),
		position_type  : PositionType::Absolute,
		justify_content: JustifyContent::Center,
		align_items    : AlignItems::Center,
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

//上端幅合せ用の隠しフレーム
fn hidden_upper_frame() -> NodeBundle
{	let width  = Val::Px( SCREEN_WIDTH  );
	let height = Val::Px( SCREEN_HEIGHT );
	let style = Style
	{	size: Size::new( width, height ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexEnd, //画面の上端
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

//下端幅合せ用の隠しフレーム
fn hidden_lower_frame() -> NodeBundle
{	let width  = Val::Px( SCREEN_WIDTH  );
	let height = Val::Px( SCREEN_HEIGHT );
	let style = Style
	{	size: Size::new( width, height ),
		position_type  : PositionType::Absolute,
		flex_direction : FlexDirection::Column,
		justify_content: JustifyContent::FlexStart, //画面の下端
		..Default::default()
	};
	let visible = Visible { is_visible: false, ..Default::default() };
	NodeBundle { style, visible, ..Default::default() }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//LIFE GAUGE
struct HpGauge;
const GAUGE_RECTANGLE: ( f32, f32, f32, f32 ) = 
(	PIXEL_PER_GRID *  8.9 - SCREEN_WIDTH  / 2.0,	//X軸：画面中央からやや左より
	PIXEL_PER_GRID * -0.7 + SCREEN_HEIGHT / 2.0,	//Y軸：画面上端からやや下がった位置
	PIXEL_PER_GRID * 15.0,							//幅
	PIXEL_PER_GRID *  0.2,							//高さ
);
const GAUGE_DEPTH: f32 = 30.0;

//
fn spawn_hp_gauge_sprite
(	mut cmds: Commands,
	mut color_matl: ResMut<Assets<ColorMaterial>>,
)
{	//LIFE GAUGEのスプライト
	let ( x, y, w, h ) = GAUGE_RECTANGLE;
	let sprite = SpriteBundle
	{	material : color_matl.add( Color::GREEN.into() ),
		transform: Transform::from_translation( Vec3::new( x, y, GAUGE_DEPTH ) ),
		sprite   : Sprite::new( Vec2::new( w, h ) ),
		..Default::default()
	};
	cmds.spawn_bundle( sprite ).insert( HpGauge );
}

//上端の情報表示を更新する(左)
fn update_ui_upper_left
(	mut q_gauge: Query<( &mut Transform, &Handle<ColorMaterial> ), With<HpGauge>>,
	mut q_ui: Query<&mut Text, With<UiUpperLeft>>,
	o_player: Option<Res<PlayerParameters>>,
	mut assets_color_matl: ResMut<Assets<ColorMaterial>>,
)
{	if let Ok( mut ui ) = q_ui.single_mut()
	{	let hp_gauge = match o_player
		{	Some( player ) =>
			{	let hp_now = player.hp_now.max( 0.0 );
				let ( mut transform, handle ) = q_gauge.single_mut().unwrap();

				//スプライトの幅のスケールを縮小する。
				//すると両端が縮むので、スプライトを左に移動して右端が縮んだように見せる
				let scale_width = &mut transform.scale[ 0 ];
				if 	*scale_width > 0.0
				{	*scale_width = ( hp_now / player.hp_max ).max( 0.0 );
					let ( x, _, w, _ ) = GAUGE_RECTANGLE;
					let translation = &mut transform.translation;
					translation.x = x - ( player.hp_max - hp_now ) * w / 200.0;	
				}

				//色を変える(緑色⇒黄色⇒赤色)
				let color_matl = assets_color_matl.get_mut( handle ).unwrap();
				let temp = hp_now / player.hp_max;
				color_matl.color = Color::rgb
				(	1.0 - ( temp - 0.6 ).max( 0.0 ) * 2.0,
					( temp.min( 0.7 ) * 2.0 - 0.4 ).max( 0.0 ),
					0.0
				);
				format!( "{}", hp_now )
			},
			None => NA_STR3.to_string()
		};
		ui.sections[ 1 ].value = hp_gauge;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//下端の情報表示を更新する(左)
fn update_ui_lower_left
(	mut q: Query<&mut Text, With<UiLowerLeft>>,
	diag: Res<Diagnostics>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	let fps_avr = if let Some( fps ) = diag.get( FrameTimeDiagnosticsPlugin::FPS )
		{	match fps.average()
			{	Some( avg ) => format!( "{:.2}", avg ),
				None        => NA_STR3.to_string()
			}
		} else { NA_STR3.to_string() };
		ui.sections[ 1 ].value = fps_avr;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//CLEARメッセージ表示
fn show_clear_message( mut q: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = true }
}

//CLEARメッセージ非表示
fn hide_clear_message( mut q: Query<&mut Visible, With<MessageClear>> )
{	if let Ok( mut ui ) = q.single_mut() { ui.is_visible = false }
}

//カウントダウンの後、Startへ遷移
fn change_state_after_countdown
(	mut q: Query<&mut Text, With<MessageClear>>,
	mut state: ResMut<State<GameState>>,
	( mut count, mut timer ): ( Local<i32>, Local<Timer> ),
	time: Res<Time>,
)
{	if let Ok( mut ui ) = q.single_mut()
	{	if *count <= 0									//カウンターが未初期化か？
		{	*timer = Timer::from_seconds( 1.0, false );	//1秒タイマーセット
			*count = 6;									//カウント数の初期化
		}
		else if timer.tick( time.delta() ).finished()	//1秒経過したら
		{	timer.reset();								//タイマー再セット
			*count -= 1;								//カウントダウン

			//カウントダウンが終わったら、Startへ遷移する
			if *count <= 0 { let _ = state.overwrite_set( GameState::Start ); }
		}
		ui.sections[ 2 ].value = format!( "{}", ( *count - 1 ).max( 0 ) );
	}
}

//End of code.