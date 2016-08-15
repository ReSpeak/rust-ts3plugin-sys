#![allow(dead_code)]

//! This file contains the definitions of plugin_definitions.h

use libc::*;

pub const MENU_BUFSZ:   usize = 128;
pub const HOTKEY_BUFSZ: usize = 128;

/// Return values for offersConfigure
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConfigureOffer
{
    /// Plugin does not implement ts3plugin_configure
    No = 0,
    /// Plugin does implement ts3plugin_configure and requests to run this function in an own thread
    NewThread,
    /// Plugin does implement ts3plugin_configure and requests to run this function in the Qt GUI thread
    QtThread
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageTarget
{
    Server = 0,
    Channel
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemType
{
    Server = 0,
    Channel,
    Client
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuType
{
    Global = 0,
    Channel,
    Client
}

#[repr(C)]
pub struct MenuItem
{
    pub type_name: MenuType,
    pub id:        c_uint,
    pub text:      [c_char; MENU_BUFSZ],
    pub icon:      [c_char; MENU_BUFSZ]
}

#[repr(C)]
pub struct Hotkey
{
    pub keyword:     [c_char; HOTKEY_BUFSZ],
    pub description: [c_char; HOTKEY_BUFSZ]
}

#[repr(C)]
pub struct BookmarkItem
{
    pub name:        *mut c_char,
    pub is_folder:   u8,
    pub reserved:    [u8; 3],
    /// Is either "char *uuid" or "BookmarkList *folder"
    pub uuid_folder: *mut c_char
}

impl BookmarkItem
{
    fn get_uuid(&self) -> *mut c_char
    {
        self.uuid_folder
    }

    fn get_folder(&self) -> *mut BookmarkList
    {
        self.uuid_folder as *mut BookmarkList
    }

    fn set_uuid(&mut self, val: *mut c_char)
    {
        self.uuid_folder = val;
    }

    fn set_folder(&mut self, val: *mut BookmarkList)
    {
        self.uuid_folder = val as *mut c_char;
    }
}

#[repr(C)]
pub struct BookmarkList
{
    pub itemcount: c_int,
    /// Should be 0 but compiler complains (the C compiler, not the rust compiler)
    pub items:     [BookmarkItem; 1]
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GuiProfile
{
    SoundCapture = 0,
    SoundPlayback,
    Hotkey,
    Soundpack,
    Identity
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectTab
{
    New = 0,
    Current,
    NewIfCurrentConnected
}
