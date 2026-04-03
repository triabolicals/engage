use unity::engine::{Color, Material, Vector2, Vector3};
use unity::prelude::*;
use crate::menu::BasicMenu;
use crate::menu::menu_item::{BasicMenuItem, MenuItem};
use crate::unityengine::{MonoBehaviorFields, Animator, GameObject, RectTransform, UnityComponent};

pub mod accessory;
pub mod common;

pub use accessory::*;

#[unity::class("App", "BasicMenuContent")]
pub struct BasicMenuContent {
    parent: MonoBehaviorFields,
    src_material: &'static Material,
    material: &'static Material,
    pub menu: &'static mut BasicMenu<BasicMenuItem>,
    animator: &'static mut Animator,
    pub obj_menu: &'static mut GameObject,
    pub front_cursor: &'static mut GameObject,
    pub back_cursor: &'static mut GameObject,
    pub viewport: &'static mut GameObject,
    pub vertical_scroll_bar: &'static mut GameObject,
    pub scroll_bar_handle: &'static mut GameObject,
    pub content: &'static mut GameObject,
    pub sub_menu_base: &'static mut GameObject,
    pub back_content: &'static mut GameObject,
    pub cursor: &'static mut BasicMenuContentCursor,
    pub scroll: &'static BasicMenuContentScroll,
    pub pos: Vector2<f32>,
    pub pos_old: Vector2<f32>,
    pub anchor_type: i32,
    pub anchored_pos_original: Vector2<f32>,
    pub anchor_min_original: Vector2<f32>,
    pub anchor_max_original: Vector2<f32>,
    pub content_base_local_pos: Vector3<f32>,
    pub back_content_base_local_pos: Vector3<f32>,
    pub color: Color,
    pub request_adjust: bool,
    // ...
}
#[unity::class("", "Cursor")]
#[nested_from_type(BasicMenuContent)]
pub struct BasicMenuContentCursor {
    pub content: &'static mut BasicMenuContent,
    unity_stuff: [u8; 0x48],
    pub pos_x: f32,
    pub pos_y: f32,
    pub from_x: f32,
    pub from_y: f32,
    pub from_w: f32,
    pub from_h: f32,
}

#[unity::class("", "Scroll")]
#[nested_from_type(BasicMenuContent)]
pub struct BasicMenuContentScroll {
    m_menu_content: &'static BasicMenuContent, // Offset 0x10, Attr: 1
    m_obj_scroll_bar: &'static GameObject, // Offset 0x18, Attr: 1
    m_scroll_bar: u64, // &'static Scrollbar, // Offset 0x20, Attr: 1
    m_scroll_now: f32, // Offset 0x28, Attr: 1
    m_scroll_old: f32, // Offset 0x2C, Attr: 1
    m_scroll_from: f32, // Offset 0x30, Attr: 1
    m_scroll_tick: f32, // Offset 0x34, Attr: 1
    m_scroll_frame: f32, // Offset 0x38, Attr: 1
}

impl BasicMenuContent {
    #[unity::class_method(75)] pub fn load_prefab_async(); // Offset: 0x2465AC0 Flags: 0
    #[unity::class_method(77)] pub fn load_prefab_async2(path: &Il2CppString); // Offset: 0x2455CD0 Flags: 0
    #[unity::class_method(76)] pub fn create() -> &'static BasicMenuContent; // Offset: 0x24622F0 Flags: 0
    #[unity::class_method(78)] pub fn unload_prefab(path: &Il2CppString); // Offset: 0x2465B90 Flags: 0
    #[unity::class_method(82)] pub fn get_canvas() -> Option<&'static GameObject>; // Offset: 0x2465C70 Flags: 0
}

pub trait MenuContent: Il2CppClassData {
    #[unity::class_method(0, BasicMenuContent)] fn ctor(&self); // Offset: 0x2455E60 Flags: 0
    // [unity::class_method(3)] fn get_menu(&self) -> &'static BasicMenu; // Offset: 0x2462AF0 Flags: 0
    // #[unity::class_method(4)] fn set_menu(&self, menu: &BasicMenu); // Offset: 0x2462B00 Flags: 0
    // #[unity::class_method(8)] fn set_animator(&self, anim: &Animator); // Offset: 0x2462D20 Flags: 0
    #[unity::class_method(13, BasicMenuContent)] fn now_cursor_width(&self) -> f32; // Offset: 0x2463250 Flags: 0
    #[unity::class_method(14, BasicMenuContent)] fn now_cursor_height(&self) -> f32; // Offset: 0x24633E0 Flags: 0
    #[unity::class_method(15, BasicMenuContent)] fn get_screen_scale(&self) -> f32; // Offset: 0x2454B80 Flags: 0
    #[unity::class_method(17, BasicMenuContent)] fn delete(&self); // Offset: 0x245CB30 Flags: 0
    #[unity::class_method(18, BasicMenuContent)] fn set_cursor_move_frame(&self, frame: f32); // Offset: 0x245D630 Flags: 0
    #[unity::class_method(19, BasicMenuContent)] fn cursor_move_instant(&self); // Offset: 0x245D640 Flags: 0
    #[unity::class_method(20, BasicMenuContent)] fn set_scroll_frame(&self, frame: f32); // Offset: 0x245D660 Flags: 0
    #[unity::class_method(21, BasicMenuContent)] fn set_scroll_bar_handle_size(&self, size: f32); // Offset: 0x2463560 Flags: 0
    #[unity::class_method(22, BasicMenuContent)] fn is_scroll_busy(&self) -> bool; // Offset: 0x2460E60 Flags: 0
    #[unity::class_method(23, BasicMenuContent)] fn scroll_instant(&self); // Offset: 0x245D670 Flags: 0
    #[unity::class_method(24, BasicMenuContent)] fn reset_scroll(&self); // Offset: 0x245C320 Flags: 0
    #[unity::class_method(26, BasicMenuContent)] fn put_cursor_in_front(&self); // Offset: 0x245D940 Flags: 0
    #[unity::class_method(27, BasicMenuContent)] fn put_cursor_in_back(&self); // Offset: 0x245DA50 Flags: 0
    #[unity::class_method(28, BasicMenuContent)] fn set_cursor_to_keep_animator_state(&self, keep: bool); // Offset: 0x245D6E0 Flags: 0
    #[unity::class_method(29, BasicMenuContent)] fn move_front_cursor_from(&self, from_x: f32, from_y: f32, frame: f32); // Offset: 0x245D700 Flags: 0
    #[unity::class_method(30, BasicMenuContent)] fn move_front_cursor_from2(&self, from_menu_item: &BasicMenuItem, frame: f32); // Offset: 0x245D720 Flags: 0
    #[unity::class_method(31, BasicMenuContent)] fn set_front_cursor_visibility(&self, visibility: bool); // Offset: 0x2463CB0 Flags: 0
    #[unity::class_method(32, BasicMenuContent)] fn restart_cursor_anim(&self); // Offset: 0x245DB50 Flags: 0
    #[unity::class_method(33, BasicMenuContent)] fn get_sub_menu_base_transform(&self) -> &'static RectTransform; // Offset: 0x245DC00 Flags: 0
    #[unity::class_method(40, BasicMenuContent)] fn build_content_pos(&self); // Offset: 0x2463F70 Flags: 0
    #[unity::class_method(48, BasicMenuContent)] fn set_pivot(&self, x: f32, y: f32); // Offset: 0x245DDE0 Flags: 0
    /*
    #[unity::class_method(49)] fn get_anchor_type(&self) -> BasicMenuAnchorType; // Offset: 0x24646B0 Flags: 0
    #[unity::class_method(50)] fn set_anchor_type(&self, anchor_type: BasicMenuAnchorType, x: f32, y: f32); // Offset: 0x245DE30 Flags: 0
    #[unity::class_method(51)] fn set_anchor_type_from_screen_coord(&self, anchor_type: BasicMenuAnchorType, x: f32, y: f32); // Offset: 0x245E040 Flags: 0
    #[unity::class_method(52)] fn anchor_position_to_anchor_type(x: f32, y: f32) -> BasicMenuAnchorType; // Offset: 0x245E220 Flags: 0
    #[unity::class_method(53)] fn screen_point_to_local_point(&self, x: f32, y: f32) -> Vector2; // Offset: 0x24646C0 Flags: 0
     */
    #[unity::class_method(54, BasicMenuContent)] fn get_x(&self) -> f32; // Offset: 0x245E2B0 Flags: 0
    #[unity::class_method(55, BasicMenuContent)] fn get_y(&self) -> f32; // Offset: 0x2454930 Flags: 0
    #[unity::class_method(56, BasicMenuContent)] fn get_pos(&self) -> Vector2<f32>; // Offset: 0x2464770 Flags: 0
    #[unity::class_method(57, BasicMenuContent)] fn set_x(&self, x: f32); // Offset: 0x245E2E0 Flags: 0
    #[unity::class_method(58, BasicMenuContent)] fn set_y(&self, y: f32); // Offset: 0x245E300 Flags: 0
    #[unity::class_method(59, BasicMenuContent)] fn set_pos(&self, x: f32, y: f32); // Offset: 0x245E320 Flags: 0

    #[unity::class_method(60, BasicMenuContent)]
    fn set_transform_as_sub_menu<T>(&self, parent_menu: &BasicMenu<T>, parent_menu_item: &T)
    where T: MenuItem; // Offset: 0x245E340 Flags: 0

    #[unity::class_method(61, BasicMenuContent)] fn update_x(&self); // Offset: 0x245C130 Flags: 0
    #[unity::class_method(62, BasicMenuContent)] fn update_y(&self); // Offset: 0x245C230 Flags: 0
    #[unity::class_method(63, BasicMenuContent)] fn update_pos(&self); // Offset: 0x2464780 Flags: 0
    #[unity::class_method(64, BasicMenuContent)] fn get_w(&self) -> f32; // Offset: 0x2454810 Flags: 0
    #[unity::class_method(65, BasicMenuContent)] fn get_h(&self) -> f32; // Offset: 0x245E660 Flags: 0
    #[unity::class_method(66, BasicMenuContent)] fn update_after_scroll(&self, is_force_update: bool); // Offset: 0x2463570 Flags: 0
    #[unity::class_method(67, BasicMenuContent)] fn force_rebuild_layout(&self); // Offset: 0x245C360 Flags: 0
    // #[unity::class_method(71)] fn calc_name(menu_name: &Il2CppString, asset_name: &Il2CppString) -> &'static Il2CppString; // Offset: 0x24653B0 Flags: 0 );
    // #[unity::class_method(80, generic)] fn create2<T>(path: &Il2CppString, parent: &GameObject) -> T; // Offset: -1 Flags: 1
    // #[unity::class_method(81, generic)] fn create3<T>(src_obj_menu: &GameObject, parent: &GameObject) -> T; // Offset: -1 Flags: 1

    #[unity::class_method(4, vtable)] fn get_menu_item_content_max(&self) -> i32; // Offset: 0x2462940 Flags: 0
    #[unity::class_method(5, vtable)] fn get_rect_transform(&self) -> &'static RectTransform; // Offset: 0x24629E0 Flags: 0
    #[unity::class_method(6, vtable)] fn is_opening(&self) -> bool; // Offset: 0x2462B10 Flags: 0
    #[unity::class_method(7, vtable)] fn is_closing(&self) -> bool; // Offset: 0x2462BC0 Flags: 0
    #[unity::class_method(8, vtable)] fn is_closed(&self) -> bool; // Offset: 0x2462C70 Flags: 0
    #[unity::class_method(9, vtable)] fn calc_cursor_moved_pos_x(&self, menu_item_index: i32) -> f32; // Offset: 0x2462D30 Flags: 0
    #[unity::class_method(10, vtable)] fn calc_cursor_moved_pos_y(&self, menu_item_index: i32) -> f32; // Offset: 0x2462EB0 Flags: 0
    #[unity::class_method(11, vtable)] fn calc_cursor_width(&self, menu_item_index: i32) -> f32; // Offset: 0x2463240 Flags: 0
    #[unity::class_method(12, vtable)] fn calc_cursor_height(&self, menu_item_index: i32) -> f32; // Offset: 0x24633D0 Flags: 0
    #[unity::class_method(13, vtable)] fn init_obj_reference(&self); // Offset: 0x2454E60 Flags: 0
    #[unity::class_method(14, vtable)] fn set_cursor_color(&self, c: Color); // Offset: 0x2463970 Flags: 0
    #[unity::class_method(15, vtable)] fn suspend(&self); // Offset: 0x2463CC0 Flags: 0
    #[unity::class_method(16, vtable)] fn un_suspend(&self); // Offset: 0x2463D00 Flags: 0
    #[unity::class_method(17, vtable)] fn set_color(&self, color: Color); // Offset: 0x2463D40 Flags: 0
    #[unity::class_method(18, vtable)] fn build_menu_item_content(&self); // Offset: 0x2455600 Flags: 0
    #[unity::class_method(19, vtable)] fn build_material(&self); // Offset: 0x2463F60 Flags: 0
    #[unity::class_method(20, vtable)] fn build_wh(&self); // Offset: 0x2454680 Flags: 0
    #[unity::class_method(21, vtable)] fn calc_w(&self) -> f32; // Offset: 0x24641F0 Flags: 0
    #[unity::class_method(22, vtable)] fn calc_h(&self) -> f32; // Offset: 0x2464490 Flags: 0
    #[unity::class_method(23, vtable)] fn get_cursor_offset_x(&self) -> f32; // Offset: 0x2464530 Flags: 0
    #[unity::class_method(24, vtable)] fn get_cursor_offset_y(&self) -> f32; // Offset: 0x2464540 Flags: 0
    #[unity::class_method(25, vtable)] fn open_anime(&self); // Offset: 0x2464550 Flags: 0
    #[unity::class_method(26, vtable)] fn close_anime(&self); // Offset: 0x24645C0 Flags: 0
    #[unity::class_method(27, vtable)] fn after_build(&self); // Offset: 0x24646A0 Flags: 0
    #[unity::class_method(28, vtable)] fn cycle_menu_item_content(&self, is_forward: bool, cycle_count: i32); // Offset: 0x24647B0 Flags: 0
    #[unity::class_method(29, vtable)] fn get_line_height_for_scroll(&self) -> f32; // Offset: 0x2465040 Flags: 0
    #[unity::class_method(30, vtable)] fn awake(&self); // Offset: 0x2465240 Flags: 0
    #[unity::class_method(31, vtable)] fn start(&self); // Offset: 0x2465490 Flags: 0
    #[unity::class_method(32, vtable)] fn update(&self); // Offset: 0x24656C0 Flags: 0
    #[unity::class_method(33, vtable)] fn on_destroy(&self); // Offset: 0x2465A40 Flags:
}
impl MenuContent for BasicMenuContent {}
impl UnityComponent for BasicMenuContent {}