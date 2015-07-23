#![allow(dead_code)]

//! This file contains the definitions of clientlib_publicdefinitions.h

#[repr(C)]
pub enum Visibility
{
    Enter = 0,
    Retain,
    Leave
}

#[repr(C)]
pub enum ConnectStatus
{
    /// There is no activity to the server, this is the default value
    Disconnected = 0,
    /// We are trying to connect, we haven't got a clientID yet, we haven't been accepted by the server
    Connecting,
    /// The server has accepted us, we can talk and hear and we got a clientID, but we don't have the channels and clients yet, we can get server infos (welcome msg etc.)
    Connected,
    /// We are CONNECTED and we are visible
    ConnectionEstablishing,
    /// We are CONNECTED and we have the client and channels available
    ConnectionEstablished
}
