use super::*;

//internal modules
mod ui_header_left;
mod ui_header_right;
mod ui_footer_left;

use ui_header_left::*;
use ui_header_right::*;
use ui_footer_left::*;

//Pluginの手続き
pub struct PluginUi;
impl Plugin for PluginUi
{	fn build( &self, app: &mut App )
	{	app
		//------------------------------------------------------------------------------------------
		.insert_resource( DbgOptResUI )							// マーカーResource
		//==========================================================================================
		.add_system_set											// ＜GameState::Init＞
		(	SystemSet::on_exit( GameState::Init )				// ＜on_exit()＞
				.with_system( spawn_text_ui_message )			// assetesプリロード後にUIを非表示で生成
		)
		//==========================================================================================
		.add_system( handle_esc_key_for_pause::<MessagePause> )	// [Esc]でpause処理
		//------------------------------------------------------------------------------------------
		.add_plugin( PluginUiHeaderLeft )
		.add_plugin( PluginUiHeaderRight )
		.add_plugin( PluginUiFooterLeft )
		//------------------------------------------------------------------------------------------
		;
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//NA文字列
const NA_STR3: &str = "---";
const NA_STR5: &str = "-----";
const NA_STR4: &str = "--.--";

//TEXT UIのメッセージセクションの型
type MessageSect<'a> = ( &'a str, &'a str, f32, Color );

#[derive(Component)]
struct MessagePause;
const MESSAGE_PAUSE: [ MessageSect; 1 ] =
[	( "P A U S E", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 5.0, Color::SILVER ),
];

const MESSAGE_CLEAR: [ MessageSect; 3 ] =
[	( "C L E A R !!\n"   , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 5.0, Color::GOLD  ),
	( "Next floor...\n\n", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 2.0, Color::WHITE ),
	( ""                 , FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 4.0, Color::WHITE ),
];

const MESSAGE_OVER: [ MessageSect; 3 ] =
[	( "GAME OVER\n", FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 5.0, Color::RED ),
	( ""           , FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 4.0, Color::RED ),
	( ""           , FONT_REGGAEONE_REGULAR, PIXEL_PER_GRID * 4.0, Color::RED ),
];

#[derive(Component)]
struct UiHeaderCenter;
const UI_HEADER_CENTER: [ MessageSect; 1 ] =
[	( APP_TITLE, FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 1.0, Color::WHITE ),
];

#[derive(Component)]
struct UiFooterCenter;
const UI_FOOTER_CENTER: [ MessageSect; 1 ] =
[	( "2021 - 2022 hyoi", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.7, Color::WHITE ),
];

#[derive(Component)]
struct UiFooterRight;
const UI_FOOTER_RIGHT: [ MessageSect; 1 ] =
[	( "powered by Rust & Bevy ", FONT_ORBITRON_BLACK, PIXEL_PER_GRID * 0.7, Color::WHITE ),
];

////////////////////////////////////////////////////////////////////////////////////////////////////

//テキストUIを配置する
fn spawn_text_ui_message( mut cmds: Commands, asset_svr: Res<AssetServer> )
{	//中央に表示するtext
	let mut pause_text = text_ui( &MESSAGE_PAUSE, &asset_svr );
	let mut clear_text = text_ui( &MESSAGE_CLEAR, &asset_svr );
	let mut over_text  = text_ui( &MESSAGE_OVER , &asset_svr );
	pause_text.visibility.is_visible = false;
	clear_text.visibility.is_visible = false;
	over_text.visibility.is_visible  = false;

	//上端・下端に表示するtext
	let mut ui_upper_left   = text_ui( &UI_HEADER_LEFT  , &asset_svr );
	let mut ui_upper_center = text_ui( &UI_HEADER_CENTER, &asset_svr );
	let mut ui_upper_right  = text_ui( &UI_HEADER_RIGHT , &asset_svr );
	let mut ui_lower_left   = text_ui( &UI_FOOTER_LEFT  , &asset_svr );
	let mut ui_lower_center = text_ui( &UI_FOOTER_CENTER, &asset_svr );
	let mut ui_lower_right  = text_ui( &UI_FOOTER_RIGHT , &asset_svr );

	ui_upper_left.style.align_self   = AlignSelf::FlexStart;
	ui_upper_center.style.align_self = AlignSelf::Center;
	ui_upper_right.style.align_self  = AlignSelf::FlexEnd;
	ui_lower_left.style.align_self   = AlignSelf::FlexStart;
	ui_lower_center.style.align_self = AlignSelf::Center;
	ui_lower_right.style.align_self  = AlignSelf::FlexEnd;

	ui_upper_left.text.alignment.horizontal   = HorizontalAlign::Left;
	ui_upper_center.text.alignment.horizontal = HorizontalAlign::Center;
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

		cmds.spawn_bundle( upper_frame ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_upper_left   ).insert( UiHeaderLeft   );
			cmds.spawn_bundle( ui_upper_center ).insert( UiHeaderCenter );
			cmds.spawn_bundle( ui_upper_right  ).insert( UiHeaderRight  );
		} );

		cmds.spawn_bundle( lower_frame ).with_children( | cmds |
		{	cmds.spawn_bundle( ui_lower_left   ).insert( UiFooterLeft   );
			cmds.spawn_bundle( ui_lower_center ).insert( UiFooterCenter );
			cmds.spawn_bundle( ui_lower_right  ).insert( UiFooterRight  );
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

//End of code.