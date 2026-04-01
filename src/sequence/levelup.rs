use crate::unit::Unit;
use super::*;
#[unity::class("App", "LevelUpSequnece")]
pub struct LevelUpSequence {
    proc: ProcInstFields,
    res_name_level_up: &'static Il2CppString,
    res_name_class_change: &'static Il2CppString,
    pub unit: &'static mut Unit,
    pub grow: Option<&'static mut Unit>,
    pub level: i32,
    pub class_change: bool,
    pub talk_mid: Option<&'static Il2CppString>, // Offset 0x98, Attr: 1
    /*
    m_window: &LevelUpWindowController, // Offset 0xA0, Attr: 1
    m_is_show_chara_image: bool, // Offset 0xA8, Attr: 1
    m_is_talk: bool, // Offset 0xA9, Attr: 1
    m_now_capability_index: i32, // Offset 0xAC, Attr: 1
    m_capability_order: &Array<Type>, // Offset 0xB0, Attr: 1
     */
}
impl LevelUpSequence {
    #[unity::class_method(0)] pub fn ctor(&self, unit: &Unit, level: i32, is_show_chara_image: bool, is_talk: bool); // Offset: 0x1BE0F50 Flags: 0
    #[unity::class_method(1)] pub fn ctor2(&self, unit: &Unit, grow: &Unit); // Offset: 0x1BE10C0 Flags: 0
    #[unity::class_method(2)] pub fn on_create(&self); // Offset: 0x1BE1220 Flags: 0
    #[unity::class_method(3)] pub fn on_dispose(&self); // Offset: 0x1BE1230 Flags: 0
    #[unity::class_method(4)] pub fn get_res_name(&self) -> &'static Il2CppString; // Offset: 0x1BE1240 Flags: 0
    #[unity::class_method(5)] pub fn prepare(&self); // Offset: 0x1BE1260 Flags: 0
    #[unity::class_method(6)] pub fn reflect(&self); // Offset: 0x1BE13A0 Flags: 0
    #[unity::class_method(7)] pub fn reload_actor(&self); // Offset: 0x1BE19A0 Flags: 0
    #[unity::class_method(8)] pub fn wait_reload_actor(&self); // Offset: 0x1BE1AF0 Flags: 0
    #[unity::class_method(9)] pub fn effect(&self); // Offset: 0x1BE1BA0 Flags: 0
    #[unity::class_method(10)] pub fn is_loading_res(&self) -> bool; // Offset: 0x1BE1C60 Flags: 0
    #[unity::class_method(11)] pub fn open(&self); // Offset: 0x1BE1CF0 Flags: 0
    #[unity::class_method(12)] pub fn wait_anime(&self); // Offset: 0x1BE5040 Flags: 0
    #[unity::class_method(13)] pub fn check_param_change(&self); // Offset: 0x1BE5260 Flags: 0
    #[unity::class_method(14)] pub fn get_up_param_count(&self) -> i32; // Offset: 0x1BE7200 Flags: 0
    #[unity::class_method(15)] pub fn get_limit_param_count(&self) -> i32; // Offset: 0x1BE8D20 Flags: 0
    #[unity::class_method(16)] pub fn calc_talk_mid(&self); // Offset: 0x1BE8E80 Flags: 0
    #[unity::class_method(17)] pub fn talk(&self); // Offset: 0x1BE9100 Flags: 0
    #[unity::class_method(18)] pub fn key_wait(&self); // Offset: 0x1BE9150 Flags: 0
    #[unity::class_method(19)] pub fn release(&self); // Offset: 0x1BE93C0 Flags: 0
    #[unity::class_method(20)] pub fn learn_job_skill(&self); // Offset: 0x1BE94B0 Flags: 0
    #[unity::class_method(21)]
    pub fn create_bind<B>(proc: &B, unit: &Unit, level: i32, is_show_chara_image: bool, is_talk: bool) where B: Bindable; // Offset: 0x1BE9600 Flags: 0
    #[unity::class_method(22)]
    pub fn create_bind_class_change<B>(proc: &B, before_unit: &Unit, after_unit: &Unit) where B: Bindable; // Offset: 0x1BE9F40 Flags: 0
    #[unity::class_method(23)] pub fn create_bind_impl<B>(&self, proc: &B) where B:Bindable; // Offset: 0x1BE96A0 Flags: 0
}
