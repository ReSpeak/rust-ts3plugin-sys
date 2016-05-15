#![allow(dead_code)]

//! This file contains the definitions of public_definitions.h and public_rare_definitions.h

use libc::*;

/// Limited length, measured in characters
pub const MAX_SIZE_CHANNEL_NAME:                     usize = 40;
pub const MAX_SIZE_VIRTUALSERVER_NAME:               usize = 40;
pub const MAX_SIZE_CLIENT_NICKNAME:                  usize = 40;
pub const MIN_SIZE_CLIENT_NICKNAME:                  usize =  3;
pub const MAX_SIZE_REASON_MESSAGE:                   usize = 80;
pub const MAX_SIZE_CLIENT_NICKNAME_NONSDK:           usize = 30;
pub const MIN_SIZE_CLIENT_NICKNAME_NONSDK:           usize =  3;
pub const MAX_SIZE_AWAY_MESSAGE:                     usize = 80;
pub const MAX_SIZE_GROUP_NAME:                       usize = 30;
pub const MAX_SIZE_TALK_REQUEST_MESSAGE:             usize = 50;
pub const MAX_SIZE_COMPLAIN_MESSAGE:                 usize = 200;
pub const MAX_SIZE_CLIENT_DESCRIPTION:               usize = 200;
pub const MAX_SIZE_HOST_MESSAGE:                     usize = 200;
pub const MAX_SIZE_HOSTBUTTON_TOOLTIP:               usize = 50;
pub const MAX_SIZE_POKE_MESSAGE:                     usize = 100;
pub const MAX_SIZE_OFFLINE_MESSAGE:                  usize = 4096;
pub const MAX_SIZE_OFFLINE_MESSAGE_SUBJECT:          usize = 200;

/// Limited length, measured in bytes (UTF-8 encoded)
pub const MAX_SIZE_TEXTMESSAGE:                      usize = 1024;
pub const MAX_SIZE_CHANNEL_TOPIC:                    usize =  255;
pub const MAX_SIZE_CHANNEL_DESCRIPTION:              usize = 8192;
pub const MAX_SIZE_VIRTUALSERVER_WELCOMEMESSAGE:     usize = 1024;
pub const MAX_SIZE_PLUGIN_COMMAND:                   usize = 1024 * 8;
pub const MAX_SIZE_VIRTUALSERVER_HOSTBANNER_GFX_URL: usize = 2000;

/// Minimum amount of seconds before a clientID that was in use can be assigned to a new client
pub const MIN_SECONDS_CLIENTID_REUSE:                usize = 300;
pub const MAX_VARIABLES_EXPORT_COUNT:                usize = 64;

/// Speaker locations used by some sound callbacks
pub const SPEAKER_FRONT_LEFT:            usize = 0x1;
pub const SPEAKER_FRONT_RIGHT:           usize = 0x2;
pub const SPEAKER_FRONT_CENTER:          usize = 0x4;
pub const SPEAKER_LOW_FREQUENCY:         usize = 0x8;
pub const SPEAKER_BACK_LEFT:             usize = 0x10;
pub const SPEAKER_BACK_RIGHT:            usize = 0x20;
pub const SPEAKER_FRONT_LEFT_OF_CENTER:  usize = 0x40;
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: usize = 0x80;
pub const SPEAKER_BACK_CENTER:           usize = 0x100;
pub const SPEAKER_SIDE_LEFT:             usize = 0x200;
pub const SPEAKER_SIDE_RIGHT:            usize = 0x400;
pub const SPEAKER_TOP_CENTER:            usize = 0x800;
pub const SPEAKER_TOP_FRONT_LEFT:        usize = 0x1000;
pub const SPEAKER_TOP_FRONT_CENTER:      usize = 0x2000;
pub const SPEAKER_TOP_FRONT_RIGHT:       usize = 0x4000;
pub const SPEAKER_TOP_BACK_LEFT:         usize = 0x8000;
pub const SPEAKER_TOP_BACK_CENTER:       usize = 0x10000;
pub const SPEAKER_TOP_BACK_RIGHT:        usize = 0x20000;
pub const SPEAKER_HEADPHONES_LEFT:       usize = 0x40000;
pub const SPEAKER_HEADPHONES_RIGHT:      usize = 0x40000;
pub const SPEAKER_MONO:                  usize = 0x40000;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TalkStatus
{
    NotTalking           = 0,
    Talking              = 1,
    TalkingWhileDisabled = 2,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodecType
{
    /// Mono,   16bit,  8kHz, bitrate dependent on the quality setting
    SpeexNarrowband = 0,
    /// Mono,   16bit, 16kHz, bitrate dependent on the quality setting
    SpeexWideband,
    /// Mono,   16bit, 32kHz, bitrate dependent on the quality setting
    SpeexUltrawideband,
    /// Mono,   16bit, 48kHz, bitrate dependent on the quality setting
    CeltMono,
    /// Mono,   16bit, 48kHz, bitrate dependent on the quality setting, optimized for voice
    OpusVoice,
    /// Stereo, 16bit, 48kHz, bitrate dependent on the quality setting, optimized for music
    OpusMusic
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodecEncryptionMode
{
    PerChannel = 0,
    ForcedOff,
    ForcedOn
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextMessageTargetMode
{
    Client = 1,
    Channel,
    Server,
    Max
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MuteInputStatus
{
    None = 0,
    Muted
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MuteOutputStatus
{
    None = 0,
    Muted
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HardwareInputStatus
{
    Disabled = 0,
    Enabled
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HardwareOutputStatus
{
    Disabled = 0,
    Enabled
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputDeactivationStatus
{
    Active      = 0,
    Deactivated = 1
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReasonIdentifier
{
    /// No reason data
    None                              = 0,
    /// {SectionInvoker}
    Moved                             = 1,
    /// No reason data
    Subscription                      = 2,
    /// reasonmsg = reason
    LostConnection                    = 3,
    /// {SectionInvoker} reasonmsg=reason; {SectionInvoker} is only added server->client
    KickChannel                       = 4,
    /// {SectionInvoker} reasonmsg=reason; {SectionInvoker} is only added server->client
    KickServer                        = 5,
    /// {SectionInvoker} reasonmsg=reason bantime=time; {SectionInvoker} is only added server->client
    KickServerBan                     = 6,
    /// reasonmsg = reason
    Serverstop                        = 7,
    /// reasonmsg = reason
    Clientdisconnect                  = 8,
    /// No reason data
    Channelupdate                     = 9,
    /// {SectionInvoker}
    Channeledit                       = 10,
    /// reasonmsg = reason
    ClientdisconnectServerShutdown    = 11
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChannelProperties
{
    /// Available for all channels that are "in view", always up-to-date
    Name = 0,
    /// Available for all channels that are "in view", always up-to-date
    Topic,
    /// Must be requested (=> requestChannelDescription)
    Description,
    /// Not available client side
    Password,
    /// Available for all channels that are "in view", always up-to-date
    Codec,
    /// Available for all channels that are "in view", always up-to-date
    CodecQuality,
    /// Available for all channels that are "in view", always up-to-date
    MaxClients,
    /// Available for all channels that are "in view", always up-to-date
    MaxFamilyClients,
    /// Available for all channels that are "in view", always up-to-date
    Order,
    /// Available for all channels that are "in view", always up-to-date
    FlagPermanent,
    /// Available for all channels that are "in view", always up-to-date
    FlagSemiPermanent,
    /// Available for all channels that are "in view", always up-to-date
    FlagDefault,
    /// Available for all channels that are "in view", always up-to-date
    FlagPassword,
    /// Available for all channels that are "in view", always up-to-date
    CodecLatencyFactor,
    /// Available for all channels that are "in view", always up-to-date
    CodecIsUnencrypted,
    /// Not available client side, not used in teamspeak, only SDK. Sets the options+salt for security hash.
    SecuritySalt,
    /// How many seconds to wait before deleting this channel
    DeleteDelay,

    /// Rare properties
    Dummy2,
    Dummy3,
    Dummy4,
    Dummy5,
    Dummy6,
    Dummy7,
    /// Available for all channels that are "in view", always up-to-date
    FlagMaxClientsUnlimited,
    /// Available for all channels that are "in view", always up-to-date
    FlagMaxFamilyClientsUnlimited,
    /// Available for all channels that are "in view", always up-to-date
    FlagMaxFamilyClientsInherited,
    /// Only available client side, stores whether we are subscribed to this channel
    FlagAreSubscribed,
    /// Not available client side, the folder used for file-transfers for this channel
    Filepath,
    /// Available for all channels that are "in view", always up-to-date
    NeededTalkPower,
    /// Available for all channels that are "in view", always up-to-date
    ForcedSilence,
    /// Available for all channels that are "in view", always up-to-date
    NamePhonetic,
    /// Available for all channels that are "in view", always up-to-date
    IconId,
    /// Available for all channels that are "in view", always up-to-date
    FlagPrivate,
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientProperties
{
    /// Automatically up-to-date for any client "in view", can be used to identify this particular client installation
    UniqueIdentifier = 0,
    /// Automatically up-to-date for any client "in view"
    Nickname,
    /// For other clients than ourself, this needs to be requested (=> requestClientVariables)
    Version,
    /// For other clients than ourself, this needs to be requested (=> requestClientVariables)
    Platform,
    /// Automatically up-to-date for any client that can be heard (in room / whisper)
    FlagTalking,
    /// Automatically up-to-date for any client "in view", this clients microphone mute status
    InputMuted,
    /// Automatically up-to-date for any client "in view", this clients headphones/speakers/mic combined mute status
    OutputMuted,
    /// Automatically up-to-date for any client "in view", this clients headphones/speakers only mute status
    OutputOnlyMuted,
    /// Automatically up-to-date for any client "in view", this clients microphone hardware status (is the capture device opened?)
    InputHardware,
    /// Automatically up-to-date for any client "in view", this clients headphone/speakers hardware status (is the playback device opened?)
    OutputHardware,
    /// Only usable for ourself, not propagated to the network
    InputDeactivated,
    /// Internal use
    IdleTime,
    /// Only usable for ourself, the default channel we used to connect on our last connection attempt
    DefaultChannel,
    /// Internal use
    DefaultChannelPassword,
    /// Internal use
    ServerPassword,
    /// Automatically up-to-date for any client "in view", not used by TeamSpeak, free storage for sdk users
    MetaData,
    /// Only make sense on the client side locally, "1" if this client is currently muted by us, "0" if he is not
    IsMuted,
    /// Automatically up-to-date for any client "in view"
    IsRecording,
    /// Internal use
    VolumeModificator,
    /// Sign
    VersionSign,
    /// SDK use, not used by teamspeak. Hash is provided by an outside source. A channel will use the security salt + other client data to calculate a hash, which must be the same as the one provided here.
    SecurityHash,

    /// Rare properties
    Dummy3,
    Dummy4,
    Dummy5,
    Dummy6,
    Dummy7,
    Dummy8,
    Dummy9,
    /// Internal use
    KeyOffset,
    /// Internal use
    LastVarRequest,
    /// Used for serverquery clients, makes no sense on normal clients currently
    LoginName,
    /// Used for serverquery clients, makes no sense on normal clients currently
    LoginPassword,
    /// Automatically up-to-date for any client "in view", only valid with PERMISSION feature, holds database client id
    DatabaseId,
    /// Automatically up-to-date for any client "in view", only valid with PERMISSION feature, holds database client id
    ChannelGroupId,
    /// Automatically up-to-date for any client "in view", only valid with PERMISSION feature, holds all servergroups client belongs too
    Servergroups,
    /// This needs to be requested (=> requestClientVariables), first time this client connected to this server
    Created,
    /// This needs to be requested (=> requestClientVariables), last time this client connected to this server
    Lastconnected,
    /// This needs to be requested (=> requestClientVariables), how many times this client connected to this server
    Totalconnections,
    /// Automatically up-to-date for any client "in view", this clients away status
    Away,
    /// Automatically up-to-date for any client "in view", this clients away status
    AwayMessage,
    /// Automatically up-to-date for any client "in view", determines if this is a real client or a server-query connection
    Type,
    /// Automatically up-to-date for any client "in view", this client got an avatar
    FlagAvatar,
    /// Automatically up-to-date for any client "in view", only valid with PERMISSION feature, holds database client id
    TalkPower,
    /// Automatically up-to-date for any client "in view", only valid with PERMISSION feature, holds timestamp where client requested to talk
    TalkRequest,
    /// Automatically up-to-date for any client "in view", only valid with PERMISSION feature, holds matter for the request
    TalkRequestMsg,
    /// Automatically up-to-date for any client "in view"
    Description,
    /// Automatically up-to-date for any client "in view"
    IsTalker,
    /// This needs to be requested (=> requestClientVariables)
    MonthBytesUploaded,
    /// This needs to be requested (=> requestClientVariables)
    MonthBytesDownloaded,
    /// This needs to be requested (=> requestClientVariables)
    TotalBytesUploaded,
    /// This needs to be requested (=> requestClientVariables)
    TotalBytesDownloaded,
    /// Automatically up-to-date for any client "in view"
    IsPrioritySpeaker,
    /// Automatically up-to-date for any client "in view"
    UnreadMessages,
    /// Automatically up-to-date for any client "in view"
    NicknamePhonetic,
    /// Automatically up-to-date for any client "in view"
    NeededServerqueryViewPower,
    /// Only usable for ourself, the default token we used to connect on our last connection attempt
    DefaultToken,
    /// Automatically up-to-date for any client "in view"
    IconId,
    /// Automatically up-to-date for any client "in view"
    IsChannelCommander,
    /// Automatically up-to-date for any client "in view"
    Country,
    /// Automatically up-to-date for any client "in view", only valid with PERMISSION feature, contains channel_id where the channel_group_id is set from
    ChannelGroupInheritedChannelId,
    /// Automatically up-to-date for any client "in view", stores icons for partner badges
    Badges, 
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VirtualServerProperties
{
    /// Available when connected, can be used to identify this particular server installation
    UniqueIdentifier = 0,
    /// Available and always up-to-date when connected
    Name,
    /// Available when connected, (=> requestServerVariables)
    Welcomemessage,
    /// Available when connected
    Platform,
    /// Available when connected
    Version,
    /// Only available on request (=> requestServerVariables), stores the maximum number of clients that may currently join the server
    MaxClients,
    /// Not available to clients, the server password
    Password,
    /// Only available on request (=> requestServerVariables)
    ClientsOnline,
    /// Only available on request (=> requestServerVariables)
    ChannelsOnline,
    /// Available when connected, stores the time when the server was created
    Created,
    /// Only available on request (=> requestServerVariables), the time since the server was started
    Uptime,
    /// Available and always up-to-date when connected
    CodecEncryptionMode,

    /// Rare properties
    Dummy0,
    Dummy1,
    Dummy2,
    Dummy3,
    Dummy4,
    Dummy5,
    Dummy6,
    Dummy7,
    Dummy8,
    /// Internal use
    Keypair,
    /// Available when connected, not updated while connected
    Hostmessage,
    /// Available when connected, not updated while connected
    HostmessageMode,
    /// Not available to clients, stores the folder used for file transfers
    Filebase,
    /// The client permissions server group that a new client gets assigned
    DefaultServerGroup,
    /// The channel permissions group that a new client gets assigned when joining a channel
    DefaultChannelGroup,
    /// Only available on request (=> requestServerVariables)
    FlagPassword,
    /// The channel permissions group that a client gets assigned when creating a channel
    DefaultChannelAdminGroup,
    /// Only available on request (=> requestServerVariables)
    MaxDownloadTotalBandwidth,
    /// Only available on request (=> requestServerVariables)
    MaxUploadTotalBandwidth,
    /// Available when connected, always up-to-date 
    HostbannerUrl,
    /// Available when connected, always up-to-date 
    HostbannerGfxUrl,
    /// Available when connected, always up-to-date
    HostbannerGfxInterval,
    /// Only available on request (=> requestServerVariables)
    ComplainAutobanCount,
    /// Only available on request (=> requestServerVariables)
    ComplainAutobanTime,
    /// Only available on request (=> requestServerVariables)
    ComplainRemoveTime,
    /// Only available on request (=> requestServerVariables)
    MinClientsInChannelBeforeForcedSilence,
    /// Available when connected, always up-to-date
    PrioritySpeakerDimmModificator,
    /// Available when connected
    Id,
    /// Only available on request (=> requestServerVariables)
    AntifloodPointsTickReduce,
    /// Only available on request (=> requestServerVariables)
    AntifloodPointsNeededCommandBlock,
    /// Only available on request (=> requestServerVariables)
    AntifloodPointsNeededIpBlock,
    /// Only available on request (=> requestServerVariables)
    ClientConnections,
    /// Only available on request (=> requestServerVariables)
    QueryClientConnections,
    /// Available when connected, always up-to-date
    HostbuttonTooltip,
    /// Available when connected, always up-to-date
    HostbuttonUrl,
    /// Available when connected, always up-to-date
    HostbuttonGfxUrl,
    /// Only available on request (=> requestServerVariables)
    QueryclientsOnline,
    /// Only available on request (=> requestServerVariables)
    DownloadQuota,
    /// Only available on request (=> requestServerVariables)
    UploadQuota,
    /// Only available on request (=> requestServerVariables)
    MonthBytesDownloaded,
    /// Only available on request (=> requestServerVariables)
    MonthBytesUploaded,
    /// Only available on request (=> requestServerVariables)
    TotalBytesDownloaded,
    /// Only available on request (=> requestServerVariables)
    TotalBytesUploaded,
    /// Only available on request (=> requestServerVariables)
    Port,
    /// Only available on request (=> requestServerVariables)
    Autostart,
    /// Only available on request (=> requestServerVariables)
    MachineId,
    /// Only available on request (=> requestServerVariables)
    NeededIdentitySecurityLevel,
    /// Only available on request (=> requestServerVariables)
    LogClient,
    /// Only available on request (=> requestServerVariables)
    LogQuery,
    /// Only available on request (=> requestServerVariables)
    LogChannel,
    /// Only available on request (=> requestServerVariables)
    LogPermissions,
    /// Only available on request (=> requestServerVariables)
    LogServer,
    /// Only available on request (=> requestServerVariables)
    LogFiletransfer,
    /// Only available on request (=> requestServerVariables)
    MinClientVersion,
    /// Available when connected, always up-to-date
    NamePhonetic,
    /// Available when connected, always up-to-date
    IconId,
    /// Available when connected, always up-to-date
    ReservedSlots,
    /// Only available on request (=> requestServerVariables)
    TotalPacketlossSpeech,
    /// Only available on request (=> requestServerVariables)
    TotalPacketlossKeepalive,
    /// Only available on request (=> requestServerVariables)
    TotalPacketlossControl,
    /// Only available on request (=> requestServerVariables)
    TotalPacketlossTotal,
    /// Only available on request (=> requestServerVariables)
    TotalPing,
    /// Internal use | contains only ONE binded ip
    Ip,
    /// Only available on request (=> requestServerVariables)
    WeblistEnabled,
    /// Internal use
    AutogeneratedPrivilegekey,
    /// Available when connected
    AskForPrivilegekey,
    /// Available when connected, always up-to-date
    HostbannerMode,
    /// Available when connected, always up-to-date
    ChannelTempDeleteDelayDefault,
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionProperties
{
    /// Average latency for a round trip through and back this connection
    Ping = 0,
    /// Standard deviation of the above average latency
    PingDeviation,
    /// How long the connection exists already
    ConnectedTime,
    /// How long since the last action of this client
    IdleTime,
    /// IP of this client (as seen from the server side)
    ClientIp,
    /// Port of this client (as seen from the server side)
    ClientPort,
    /// IP of the server (seen from the client side) - only available on yourself, not for remote clients, not available server side
    ServerIp,
    /// Port of the server (seen from the client side) - only available on yourself, not for remote clients, not available server side
    ServerPort,
    /// How many Speech packets were sent through this connection
    PacketsSentSpeech,
    PacketsSentKeepalive,
    PacketsSentControl,
    /// How many packets were sent totally (this is PACKETS_SENT_SPEECH + PACKETS_SENT_KEEPALIVE + PACKETS_SENT_CONTROL)
    PacketsSentTotal,
    BytesSentSpeech,
    BytesSentKeepalive,
    BytesSentControl,
    BytesSentTotal,
    PacketsReceivedSpeech,
    PacketsReceivedKeepalive,
    PacketsReceivedControl,
    PacketsReceivedTotal,
    BytesReceivedSpeech,
    BytesReceivedKeepalive,
    BytesReceivedControl,
    BytesReceivedTotal,
    PacketlossSpeech,
    PacketlossKeepalive,
    PacketlossControl,
    /// The probability with which a packet round trip failed because a packet was lost
    PacketlossTotal,
    /// The probability with which a speech packet failed from the server to the client
    Server2ClientPacketlossSpeech,
    Server2ClientPacketlossKeepalive,
    Server2ClientPacketlossControl,
    Server2ClientPacketlossTotal,
    Client2ServerPacketlossSpeech,
    Client2ServerPacketlossKeepalive,
    Client2ServerPacketlossControl,
    Client2ServerPacketlossTotal,
    /// Howmany bytes of speech packets we sent during the last second
    BandwidthSentLastSecondSpeech,
    BandwidthSentLastSecondKeepalive,
    BandwidthSentLastSecondControl,
    BandwidthSentLastSecondTotal,
    /// Howmany bytes/s of speech packets we sent in average during the last minute
    BandwidthSentLastMinuteSpeech,
    BandwidthSentLastMinuteKeepalive,
    BandwidthSentLastMinuteControl,
    BandwidthSentLastMinuteTotal,
    BandwidthReceivedLastSecondSpeech,
    BandwidthReceivedLastSecondKeepalive,
    BandwidthReceivedLastSecondControl,
    BandwidthReceivedLastSecondTotal,
    BandwidthReceivedLastMinuteSpeech,
    BandwidthReceivedLastMinuteKeepalive,
    BandwidthReceivedLastMinuteControl,
    BandwidthReceivedLastMinuteTotal,

    /// Rare properties
    Dummy0,
    Dummy1,
    Dummy2,
    Dummy3,
    Dummy4,
    Dummy5,
    Dummy6,
    Dummy7,
    Dummy8,
    Dummy9,
    /// How many bytes per second are currently being sent by file transfers
    FileTransferBandwidthSent,
    /// Bow many bytes per second are currently being received by file transfers
    FiletransferBandwidthReceived,
    /// How many bytes we received in total through file transfers
    FiletransferBytesReceivedTotal,
    /// How many bytes we sent in total through file transfers
    FiletransferBytesSentTotal,
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogTypes
{
    None         =  0,
    File         =  1,
    Console      =  2,
    Userlogging  =  4,
    NoNetlogging =  8,
    Database     = 16
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum LogLevel
{
    /// These messages stop the program
    Critical = 0,
    /// Everything that is really bad, but not so bad we need to shut down
    Error,
    /// Everything that *might* be bad
    Warning,
    /// Output that might help find a problem
    Debug,
    /// Informational output, like "starting database version x.y.z"
    Info,
    /// Developer only output (will not be displayed in release mode)
    Devel
}

#[repr(C)]
pub struct Ts3Vector
{
    /// X co-ordinate in 3D space
    pub x: c_float,
    /// Y co-ordinate in 3D space
    pub y: c_float,
    /// Z co-ordinate in 3D space
    pub z: c_float
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupWhisperType
{
    Servergroup      = 0,
    Channelgroup     = 1,
    Channelcommander = 2,
    Allclients       = 3,
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupWhisperTargetMode
{
    All                   = 0,
    Currentchannel        = 1,
    Parentchannel         = 2,
    Allparentchannel      = 3,
    Channelfamily         = 4,
    Ancestorchannelfamily = 5,
    Subchannels           = 6,
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MonoSoundDestination
{
    /// Send mono sound to all available speakers
    All               = 0,
    /// Send mono sound to front center speaker if available
    FrontCenter       = 1,
    /// Send mono sound to front left/right speakers if available
    FrontLeftAndRight = 2
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SecuritySaltOptions
{
    /// Put nickname into security hash
    CheckNickname = 1,
    /// Put (game)meta data into security hash
    CheckMetaData = 2
}

/// This enum is used to disable client commands on the server
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientCommand
{
    RequestConnectionInfo       = 0,
    RequestClientMove           = 1,
    RequestXXMuteClients        = 2,
    RequestClientKickFromXXX    = 3,
    FlushChannelCreation        = 4,
    FlushChannelUpdate          = 5,
    RequestChannelMove          = 6,
    RequestChannelDelete        = 7,
    RequestChannelDescription   = 8,
    RequestChannelXXSubscripeXX = 9,
    RequestServerConnectionInfo = 10,
    RequestSendXXXTextMsg       = 11,
    Endmarker
}

/// Access Control List
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ACLType
{
    None      = 0,
    WhiteList = 1,
    BlackList = 2
}

/// Some structs to handle variables in callbacks
#[repr(C)]
pub struct VariablesExportItem
{
    /// This item has valid values. ignore this item if 0
    pub item_is_valid:   u8,
    /// The value in proposed is set. if 0 ignore proposed
    pub proposed_is_set: u8,
    /// Current value (stored in memory)
    pub current:         *const c_char,
    /// New value to change to (const, so no updates please)
    pub proposed:        *const c_char
}

#[repr(C)]
pub struct VariablesExport
{
    pub items: [VariablesExportItem; MAX_VARIABLES_EXPORT_COUNT]
}

#[repr(C)]
pub struct ClientMiniExport
{
    pub id:       u16,
    pub channel:  u64,
    pub ident:    *const c_char,
    pub nickname: *const c_char
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupShowNameTreeMode
{
    /// Dont group show name
    None = 0,
    /// Show group name before client name
    Before,
    /// Show group name behind client name
    Behind
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PluginTargetMode
{
    /// Send plugincmd to all clients in current channel
    CurrentChannel = 0,
    /// Send plugincmd to all clients on server
    Server,
    /// Send plugincmd to all given client ids
    Client,
    /// Send plugincmd to all subscribed clients in current channel
    CurrentChannelSubscribedClients,
    Max
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerBinding
{
    Virtualserver = 0,
    Serverquery   = 1,
    Filetransfer  = 2
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HostmessageMode
{
    /// Dont display anything
    None = 0,
    /// Display message inside log
    Log,
    /// Display message inside a modal dialog
    Modal,
    /// Display message inside a modal dialog and quit/close server/connection
    Modalquit
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HostbannerMode
{
    /// Do not adjust
    NoAdjust = 0,
    /// Do not adjust
    AdjustIgnoreAspect,
    /// Do not adjust
    AdjustKeepAspect
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileTransferState
{
    Initialising = 0,
    Active,
    Finished
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileListType
{
    Directory = 0,
    File
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientType
{
    Normal = 0,
    Serverquery
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AwayStatus
{
    None = 0,
    Zzz
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandLinePropertiesRare
{
    Nothing = 0,
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerInstancePropertiesRare
{
    DatabaseVersion = 0,
    FiletransferPort,
    ServerEntropy,
    MonthlyTimestamp,
    MaxDownloadTotalBandwidth,
    MaxUploadTotalBandwidth,
    GuestServerqueryGroup,
    /// How many commands we can issue while in the SERVERINSTANCE_SERVERQUERY_FLOOD_TIME window
    ServerqueryFloodCommands,
    /// Time window in seconds for max command execution check
    ServerqueryFloodTime,
    /// How many seconds someone get banned if he floods
    ServerqueryBanTime,
    TemplateServeradminGroup,
    TemplateServerdefaultGroup,
    TemplateChanneladminGroup,
    TemplateChannelDefaultGroup,
    PermissionsVersion,
    PendingConnectionsPerIp,
    Endmarker
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LicenseViolationType
{
    No = 0,
    Slot,
    SlotSuspicion
}

bitflags! {
flags BBCodeTags: u32 {
    const B           = 0x00000001,
    const I           = 0x00000002,
    const U           = 0x00000004,
    const S           = 0x00000008,
    const SUP         = 0x00000010,
    const SUB         = 0x00000020,
    const COLOR       = 0x00000040,
    const SIZE        = 0x00000080,
    const GROUP_TEXT  = 0x000000FF,

    const LEFT        = 0x00001000,
    const RIGHT       = 0x00002000,
    const CENTER      = 0x00004000,
    const GROUP_ALIGN = 0x00007000,

    const URL         = 0x00010000,
    const IMAGE       = 0x00020000,
    const HR          = 0x00040000,

    const LIST        = 0x00100000,
    const LISTITEM    = 0x00200000,
    const GROUP_LIST  = 0x00300000,

    const TABLE       = 0x00400000,
    const TR          = 0x00800000,
    const TH          = 0x01000000,
    const TD          = 0x02000000,
    const GROUP_TABLE = 0x03C00000,

    const DEF_SIMPLE     = B.bits | I.bits | U.bits | S.bits | SUP.bits |
        SUB.bits | COLOR.bits | URL.bits,
    const DEF_SIMPLE_IMG = DEF_SIMPLE.bits | IMAGE.bits,
    const DEF_EXTENDED   = GROUP_TEXT.bits | GROUP_ALIGN.bits | URL.bits |
        IMAGE.bits | HR.bits | GROUP_LIST.bits | GROUP_TABLE.bits
}}

// As they are only typedefs and I didn't found any usage, I'll just leave them here for now
//typedef int(*ExtraBBCodeValidator)(void* userparam, const char* tag, const char* paramValue, int paramValueSize, const char* childValue, int childValueSize);
//typedef const char* (*ExtraBBCodeParamTransform)(void* userparam, const char* tag, const char* paramValue);
