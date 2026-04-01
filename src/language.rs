//! Utility to force the game to reload the language MSBTs.

use unity::prelude::*;

/// Holder for the language and voice chosen by the player.
#[unity::class("App", "Language")]
pub struct Language { }

impl Language {
    
    #[unity::class_method(2)] pub fn initialize(); // Offset: 0x1BDB490 Flags: 0
    #[unity::class_method(4)] pub fn get_lang() -> LanguageLangs; // Offset: 0x1BDBC80 Flags: 0
    #[unity::class_method(8)] pub fn set_lang(lang: LanguageLangs); // Offset: 0x1BDBBF0 Flags: 0

    /// Force the game to unload and reload every language MSBT file.
    ///
    /// Avoid calling this while the player is already in the game, as it will most likely call a crash.
    #[unity::class_method(11)] pub fn reflect_setting(); // Offset: 0x1BDC390 Flags: 0
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LanguageLangs {
    JPJapanese = 0, // Attr: 17
    USEnglish = 1, // Attr: 17
    USFrench = 2, // Attr: 17
    USSpanish = 3, // Attr: 17
    EUEnglish = 4, // Attr: 17
    EUFrench = 5, // Attr: 17
    EUSpanish = 6, // Attr: 17
    EUGerman = 7, // Attr: 17
    EUItalian = 8, // Attr: 17
    CNTraditional = 9, // Attr: 17
    CNSimplified = 10, // Attr: 17
    KRKorean = 11, // Attr: 17
}