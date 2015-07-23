#![allow(dead_code)]

//! This file contains the definitions of plugin_definitions.h

use libc::*;

pub const MENU_BUFSZ:   usize = 128;
pub const HOTKEY_BUFSZ: usize = 128;

/// Return values for offersConfigure
#[repr(C)]
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
pub enum MessageTarget
{
	Server = 0,
	Channel
}

#[repr(C)]
pub enum ItemType
{
	Server = 0,
	Channel,
	Client
}

#[repr(C)]
pub enum MenuType
{
	Global = 0,
	Channel,
	Client
}

#[repr(C)]
pub struct MenuItem
{
    type_name: MenuType,
    id:        c_uint,
    text:      [c_char; MENU_BUFSZ],
    icon:      [c_char; MENU_BUFSZ]
}

#[repr(C)]
pub struct Hotkey
{
    keyword:     [c_char; HOTKEY_BUFSZ],
    description: [c_char; HOTKEY_BUFSZ]
}

#[repr(C)]
pub struct BookmarkItem
{
    name:        *mut c_char,
    is_folder:   u8,
    reserved:    [u8; 3],
    /// Is either "char *uuid" or "BookmarkList *folder"
    uuid_folder: *mut c_char
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
    itemcount: c_int,
    /// Should be 0 but compiler complains (the C compiler, not the rust compiler)
    items:     [BookmarkItem; 1]
}

#[repr(C)]
pub enum GuiProfile
{
	SoundCapture = 0,
	SoundPlayback,
	Hotkey,
	Soundpack,
	Identity
}

#[repr(C)]
pub enum ConnectTab
{
	New = 0,
	Current,
	NewIfCurrentConnected
}
