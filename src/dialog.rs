//! Structures and wrappers to display various Dialogs to the player. Tied to the [`ProcInst`](crate::proc::ProcInst) system.

use unity::prelude::*;
use unity::system::action::Action;
use unity::system::List;
use crate::dialog::yesno::BasicDialogItemNo;
use crate::menu::{menu_item::BasicMenuItemFields, BasicMenuFields};
use crate::mess::Mess;
use crate::sequence::Bindable;

pub mod yesno;
pub mod shopyesno;
pub mod exchangeyesno;

#[unity::class("App", "BasicDialog")]
pub struct BasicDialog2 {
    parent: BasicMenuFields<BasicDialogItem>,
    pub m_dialog_content: &'static BasicDialogContent, // Offset 0xC8, Attr: 4
    m_b_bind_bg: bool, // Offset 0xD0, Attr: 1
    m_is_not_bind_bg: bool, // Offset 0xD1, Attr: 1
}

impl BasicDialog2 {
    pub fn set_text<'a>(&self, text: impl Into<&'a Il2CppString>) {
        self.set_text_(text.into());
    }
    #[unity::class_method(6)] pub fn set_text_(&self, text: &Il2CppString); // Offset: 0x2453D90 Flags: 0
    #[unity::class_method(10)]
    pub fn create_basic_dialog_bind<B>(proc: &B, menu_item_list: &List<BasicDialogItem>) -> &'static mut BasicDialog2
    where B: Bindable;
    pub fn create_bind<B: Bindable>(proc: &B,
        text: &Il2CppString,
        yes_text: &Il2CppString,
        no_text: &Il2CppString,
        yes_handler: Option<&Action>
    ) -> &'static mut BasicDialog2
    {
        let list = List::<BasicDialogItem>::with_capacity(2).unwrap();
        let yes_item = YesMenuItem::new(yes_text, yes_handler);
        list.add(yes_item.cast_mut());
        let no = BasicDialogItemNo::new(no_text.to_string());
        list.add(no.cast_mut());
        let dialog = Self::create_basic_dialog_bind(proc, list);
        dialog.set_text_(text);
        dialog
    }
    pub fn create_bind2<B: Bindable>(
        proc: &B, t: impl AsRef<str>,
        y_text: impl AsRef<str>,
        no_text: impl AsRef<str>,
        handler: Option<&Action>
    ) -> &'static mut BasicDialog2
    {
        let list = List::<BasicDialogItem>::with_capacity(2).unwrap();
        let yes_item = YesMenuItem::new(y_text, handler);
        list.add(yes_item.cast_mut());
        let no = BasicDialogItemNo::new(no_text);
        list.add(no.cast_mut());
        let dialog = Self::create_basic_dialog_bind(proc, list);
        dialog.set_text(t);
        dialog
    }
    pub fn create_confirm_cancel_bind<B: Bindable>(proc: &B, message: impl AsRef<str>, handler: Option<&Action>) -> &'static mut BasicDialog2{
        let list = List::<BasicDialogItem>::with_capacity(2).unwrap();
        let yes_item = YesMenuItem::new_with_mess("MID_Decision", handler);
        let no_item = BasicDialogItemNo::new_from_mess("MID_MATCH_Map_UpRoad_No");
        list.add(yes_item.cast_mut());
        list.add(no_item.cast_mut());
        let dialog = Self::create_basic_dialog_bind(proc, list);
        dialog.set_text(message);
        dialog
    }
}

#[repr(C)]
pub struct BasicDialog<T: 'static> {
    parent: BasicMenuFields<T>,
    dialog_content: &'static BasicDialogContent,
    bind_bg: bool,
    not_bind_bg: bool,
}
impl <T> BasicDialog<T> {
    pub fn set_text<'a>(&self, text: impl Into<&'a Il2CppString>) {
        unsafe { dialog_set_text(self, text.into(), None) };
    }
}

#[unity::class("App", "BasicDialogContent")]
pub struct BasicDialogContent {}

#[repr(C)]
#[unity::class("App", "BasicDialogItem")]
pub struct BasicDialogItem {
    pub parent: BasicMenuItemFields,
    text: &'static Il2CppString,
}

impl BasicDialogItem {
    pub fn new(text: impl AsRef<str>) -> &'static mut BasicDialogItem {
        let item = BasicDialogItem::instantiate().unwrap();
        unsafe {
            dialog_item_ctor(item, text.into(), None);
        }
        item
    }
}

pub trait DialogMenuItem: Sized {
    fn ctor<'a>(&self, text: impl Into<&'a Il2CppString>) {
        unsafe { dialog_item_ctor2(self, text.into(), None) }
    }
    fn ctor_with_mess<'a>(&self, text: impl Into<&'a Il2CppString>) {
        unsafe { dialog_item_ctor2(self, Mess::get(text), None) }
    }
    fn cast(&self) -> &'static BasicDialogItem {
        unsafe { std::mem::transmute::<&Self, &'static BasicDialogItem>(self) }
    }
    fn cast_mut(&mut self) -> &'static mut BasicDialogItem {
        unsafe { std::mem::transmute::<&mut Self, &'static mut BasicDialogItem>(self) }
    }
}

#[unity::class("App", "YesMenuItem")]
pub struct YesMenuItem {
    pub parent: BasicMenuItemFields,
    pub text: &'static Il2CppString,
    pub yes_event_handler: &'static mut Action, // Offset 0x70, Attr: 1
}
impl DialogMenuItem for YesMenuItem {}

impl YesMenuItem {
    pub fn new<'a>(text: impl Into<&'a Il2CppString>, action: Option<&Action>) -> &'static mut YesMenuItem {
        let item = YesMenuItem::class().instantiate_as::<YesMenuItem>().unwrap();
        item.ctor_(action);
        item.ctor(text);
        item
    }
    pub fn new_with_mess<'a>(text: impl Into<&'a Il2CppString>, action: Option<&Action>) -> &'static mut YesMenuItem {
        let item = YesMenuItem::class().instantiate_as::<YesMenuItem>().unwrap();
        item.ctor_(action);
        item.text = Mess::get(text);
        item
    }
    #[unity::class_method(0)] pub fn ctor_(&self, yes_event_handler: Option<&Action>); // Offset: 0x293D8F0 Flags: 0
}

#[skyline::from_offset(0x2455f30)]
fn dialog_item_ctor(proc: &BasicDialogItem, text: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x2455f30)]
fn dialog_item_ctor2<T: DialogMenuItem>(item: &T, text: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x2453d90)]
fn dialog_set_text<T: 'static>(this: &BasicDialog<T>, text: &Il2CppString, optional_method: OptionalMethod);
