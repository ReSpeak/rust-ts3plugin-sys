#![allow(dead_code)]
// Uses lots of alignment which rustfmt destroys
#![cfg_attr(rustfmt, rustfmt::skip)]

//! This file contains the definitions of public_definitions.h and public_rare_definitions.h

use std;
use std::os::raw::*;

/// channel name maximum length in characters
pub const MAX_SIZE_CHANNEL_NAME:                     usize = 40;
/// virtual server name maximum length in characters
pub const MAX_SIZE_VIRTUALSERVER_NAME:               usize = 40;
/// client display name length limit in characters
pub const MAX_SIZE_CLIENT_NICKNAME:                  usize = 40;
/// client display name minimum length in characters
pub const MIN_SIZE_CLIENT_NICKNAME:                  usize =  3;
/// length limit in characters for kick, move, etc reasons
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
pub const MAX_SIZE_USER_TAG:                         usize = 100;

/// text message length limit, measured in bytes (utf8 encoded)
pub const MAX_SIZE_TEXTMESSAGE:                      usize = 8192;
/// // channel topic lengt limith, measured in bytes (utf8 encoded)
pub const MAX_SIZE_CHANNEL_TOPIC:                    usize =  255;
/// channel description length limit, measured in bytes (utf8 encoded)
pub const MAX_SIZE_CHANNEL_DESCRIPTION:              usize = 8192;
/// server welcome message length limit measured in bytes (utf8 encoded)
pub const MAX_SIZE_VIRTUALSERVER_WELCOMEMESSAGE:     usize = 1024;
pub const MAX_SIZE_VIRTUALSERVER_HOSTBANNER_GFX_URL: usize = 2000;
pub const SIZE_MYTSID:                               usize = 44;

/// Minimum amount of seconds before a clientID that was in use can be assigned to a new client
pub const MIN_SECONDS_CLIENTID_REUSE:                usize = 300;
pub const MAX_VARIABLES_EXPORT_COUNT:                usize = 64;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Visibility {
	/// Client joined from an unsubscribed channel, or joined the server.
	Enter = 0,
	/// Client switched from one subscribed channel to a different subscribed channel.
	Retain,
	/// Client switches to an unsubscribed channel, or disconnected from server.
	Leave,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectStatus {
	/// There is no activity to the server, this is the default value
	Disconnected = 0,
	/// We are trying to connect, we haven't got a clientID yet, we haven't been accepted by the server
	Connecting,
	/// The server has accepted us, we can talk and hear and we got a clientID, but we don't have the channels and clients yet, we can get server infos (welcome msg etc.)
	Connected,
	/// We are connected and we are visible
	ConnectionEstablishing,
	/// We are connected and we have the client and channels available
	ConnectionEstablished,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LocalTestMode {
	Off = 0,
	VoiceLocalOnly,
	VoiceLocalAndRemote,
	TalkStatusChangesOnly,
}

bitflags! {
/// Speaker locations used by some sound callbacks
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct Speaker: c_uint {
	const SPEAKER_FRONT_LEFT            = 0x00001;
	const SPEAKER_FRONT_RIGHT           = 0x00002;
	const SPEAKER_FRONT_CENTER          = 0x00004;
	const SPEAKER_LOW_FREQUENCY         = 0x00008;
	const SPEAKER_BACK_LEFT             = 0x00010;
	const SPEAKER_BACK_RIGHT            = 0x00020;
	const SPEAKER_FRONT_LEFT_OF_CENTER  = 0x00040;
	const SPEAKER_FRONT_RIGHT_OF_CENTER = 0x00080;
	const SPEAKER_BACK_CENTER           = 0x00100;
	const SPEAKER_SIDE_LEFT             = 0x00200;
	const SPEAKER_SIDE_RIGHT            = 0x00400;
	const SPEAKER_TOP_CENTER            = 0x00800;
	const SPEAKER_TOP_FRONT_LEFT        = 0x01000;
	const SPEAKER_TOP_FRONT_CENTER      = 0x02000;
	const SPEAKER_TOP_FRONT_RIGHT       = 0x04000;
	const SPEAKER_TOP_BACK_LEFT         = 0x08000;
	const SPEAKER_TOP_BACK_CENTER       = 0x10000;
	const SPEAKER_TOP_BACK_RIGHT        = 0x20000;
	const SPEAKER_HEADPHONES_LEFT       = 0x10000000;
	const SPEAKER_HEADPHONES_RIGHT      = 0x20000000;
	const SPEAKER_MONO                  = 0x40000000;
}}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TalkStatus {
	/// client is not talking
	NotTalking           = 0,
	/// client is talking
	Talking              = 1,
	/// client is talking while the microphone is muted (only valid for own client)
	TalkingWhileDisabled = 2,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodecType {
	/// (deprecated) Mono,   16bit,  8kHz, bitrate dependent on the quality setting
	SpeexNarrowband = 0,
	/// (deprecated) Mono,   16bit, 16kHz, bitrate dependent on the quality setting
	SpeexWideband,
	/// (deprecated) Mono,   16bit, 32kHz, bitrate dependent on the quality setting
	SpeexUltrawideband,
	/// (deprecated) Mono,   16bit, 48kHz, bitrate dependent on the quality setting
	CeltMono,
	/// Mono,   16bit, 48kHz, bitrate dependent on the quality setting, optimized for voice
	OpusVoice,
	/// Stereo, 16bit, 48kHz, bitrate dependent on the quality setting, optimized for music
	OpusMusic,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodecEncryptionMode {
	/// voice data encryption decided per channel
	PerChannel = 0,
	/// voice data encryption disabled
	ForcedOff,
	/// voice data encryption enabled
	ForcedOn,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextMessageTargetMode {
	/// Message is a private message to another client
	Client = 1,
	/// Message is sent to a channel, received by all clients in that channel at the time
	Channel,
	/// Message is sent to every client on the server
	Server,
	Max,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MuteInputStatus {
	/// Microphone is not muted, audio is sent to the server
	None = 0,
	/// Microphone is muted, no audio is transmitted to the server
	Muted,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MuteOutputStatus {
	/// Speaker is active, server is sending us audio
	None = 0,
	/// Speaker is muted, server is not sending audio to us
	Muted,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HardwareInputStatus {
	/// no capture device opened
	Disabled = 0,
	/// capture device open
	Enabled,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HardwareOutputStatus {
	/// no playback device opened
	Disabled = 0,
	/// playback device open
	Enabled,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputDeactivationStatus {
	/// Audio is captured from the capture device.
	Active      = 0,
	/// No audio is captured from the capture device.
	Deactivated = 1,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReasonIdentifier {
	/// No reason data
	None                              = 0,
	/// client was moved
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
	ClientdisconnectServerShutdown    = 11,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtocolEncryptionCipher {
	Aes128 = 0,
	Aes256,
	EndMarker,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChannelProperties {
	/// String.  Read/Write. Name of the channel. Always available.
	Name = 0,
	/// String.  Read/Write. Short single line text describing what the channel is about. Always available.
	Topic,
	/// String.  Read/Write. Arbitrary text (up to 8k bytes) with information about the channel.
	/// Must be requested (`ts3client_requestChannelDescription`)
	Description,
	/// String.  Read/Write. Password of the channel. Read access is limited to the server. Clients
	/// will only ever see the last password they attempted to use when joining the channel. Always available.
	Password,
	/// Integer. Read/Write. The codec this channel is using. One of the values from the `CodecType`
	/// enum. Always available.
	Codec,
	/// Integer. Read/Write. The quality setting of the channel. Valid values are 0 to 10 inclusive.
	/// Higher value means better voice quality but also more bandwidth usage. Always available.
	CodecQuality,
	/// Integer. Read/Write. The number of clients that can be in the channel simultaneously.
	/// Always available.
	MaxClients,
	/// Integer. Read/Write. The total number of clients that can be in this channel and all
	/// sub channels of this channel. Always available.
	MaxFamilyClients,
	/// UInt64.  Read/Write. The ID of the channel below which this channel should be displayed. If 0
	/// the channel is sorted at the top of the current level. Always available.
	Order,
	/// Integer. Read/Write. Boolean (1/0) indicating whether the channel remains when empty.
	/// Permanent channels are stored to the database and available after server restart. SDK
	/// users will need to take care of restoring channel at server start on their own.
	/// Mutually exclusive with `CHANNEL_FLAG_SEMI_PERMANENT`. Always available.
	FlagPermanent,
	/// Integer. Read/Write. Boolean (1/0) indicating whether the channel remains when
	/// empty. Semi permanent channels are not stored to disk and gone after server
	/// restart but remain while empty. Mutually exclusive with 
	/// `CHANNEL_FLAG_PERMANENT.`` Always available.
	FlagSemiPermanent,
	/// Integer. Read/Write. Boolean (1/0). The default channel is the channel that all clients
	/// are located in when they join the server, unless the client explicitly specified a
	/// different channel when connecting and is allowed to join their preferred channel. Only
	/// one channel on the server can have this flag set. The default channel must have 
	/// `CHANNEL_FLAG_PERMANENT` set. Always available.
	FlagDefault,
	/// Integer. Read/Write. Boolean (1/0) indicating whether this channel is password protected.
	/// When removing or setting `CHANNEL_PASSWORD` you also need to adjust this flag.
	FlagPassword,
	/// (deprecated) Integer. Read/Write. Allows to increase packet size, reducing
	/// bandwith at the cost of higher latency of voice transmission. Valid values are
	/// 1-10 inclusive. 1 is the default and offers the lowest latency. Always available.
	CodecLatencyFactor,
	/// Integer. Read/Write. Boolean (1/0). If 0 voice data is encrypted, if 1 the voice
	/// data is not encrypted. Only used if the server 
	/// `VIRTUALSERVER_CODEC_ENCRYPTION_MODE` is set to `CODEC_ENCRYPTION_PER_CHANNEL`.
	/// Always available.
	CodecIsUnencrypted,
	/// String.  Read/Write. SDK Only, not used by TeamSpeak. This channels security hash. When
	/// a client joins their `CLIENT_SECURITY_HASH` is compared to this value, to allow or
	/// deny the client access to the channel. Used to enforce clients joining the server with
	/// specific identity and `CLIENT_META_DATA`. See SDK Documentation about this feature
	/// for further details. Always available.
	SecuritySalt,
	/// UInt64.  Read/Write. Number of seconds deletion of temporary channels is delayed after
	/// the last client leaves the channel. Channel is only deleted if empty when the delete
	/// delay expired. Always available.
	DeleteDelay,
	/// String.  Read only.  An identifier that uniquely identifies a channel. Available in
	/// Server >= 3.10.0
	UniqueIdentifier,

	/// Rare properties
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
	BannerGfxUrl,
	/// Available for all channels that are "in view", always up-to-date
	BannerMode,
	PermissionHints,
	/// Storage space that is allowed to be used by this channels files (in MiB)
	StorageQuota,
	Endmarker,

	/// (for clientlibv2) expected delete time in monotonic clock seconds or 0 if nothing is expected
	DeleteDelayDeadline = 127,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientProperties {
	/// String.  Read only.  Public Identity, can be used to identify a client
	/// installation. Remains identical as long as the client keeps using the same
	/// identity. Available for visible clients.
	UniqueIdentifier = 0,
	/// String.  Read/Write. Display name of the client. Available for visible clients.
	Nickname,
	/// String.  Read only.  Version String of the client used. For clients other than ourself this
	/// needs to be requested (`ts3client_requestClientVariables`).
	Version,
	/// String.  Read only.  Operating system used by the client. For other clients other than ourself
	/// this needs to be requested (`ts3client_requestClientVariables`).
	Platform,
	/// Integer. Read only.  Whether the client is talking. Available on clients that are either
	/// whispering to us, or in our channel.
	FlagTalking,
	/// Integer. Read/Write. Microphone mute status. Available for visible clients. One of the
	/// values from the `MuteInputStatus` enum.
	InputMuted,
	/// Integer. Read only.  Speaker mute status. Speaker mute implies microphone mute. Available
	/// for visible clients. One of the values from the `MuteOutputStatus` enum.
	OutputMuted,
	/// Integer. Read only.  Speaker mute status. Microphone may be active. Available for
	/// visible clients. One of the values from the `MuteOutputStatus` enum.
	OutputOnlyMuted,
	/// Integer. Read only.  Indicates whether a capture device is open. Available for visible
	/// clients. One of the values from the `HardwareInputStatus` enum.
	InputHardware,
	/// Integer. Read only.  Indicates whether a playback device is open. Available for visible
	/// clients. One of the values from the `HardwareOutputStatus` enum.
	OutputHardware,
	/// Integer. Read/Write. Not available server side. Local microphone mute status.
	/// Available only for own client. Used to implement Push To Talk. One of the values from
	/// the `InputDeactivationStatus` enum.
	InputDeactivated,
	/// UInt64.  Read only.  Seconds since last activity. Available only for own client.
	IdleTime,
	/// String.  Read only.  User specified channel they joined when connecting to the server.
	/// Available only for own client.
	DefaultChannel,
	/// String.  Read only.  User specified channel password for the channel they
	/// attempted to join when connecting to the server. Available only for own
	/// client.
	DefaultChannelPassword,
	/// String.  Read only.  User specified server password. Available only for own client.
	ServerPassword,
	/// String.  Read/Write. Can be used to store up to 4096 bytes of information on clients. Not
	/// used by TeamSpeak. Available for visible clients.
	MetaData,
	/// Integer. Read only.  Not available server side. Indicates whether we have muted the client
	/// using `ts3client_requestMuteClients`. Available for visible clients other than ourselves.
	IsMuted,
	/// Integer. Read only.  Indicates whether the client is recording incoming audio. Available
	/// for visible clients.
	IsRecording,
	/// Integer. Read only.  Volume adjustment for this client as set by
	/// `ts3client_setClientVolumeModifier`. Available for visible clients.
	VolumeModificator,
	/// String.  Read only.  TeamSpeak internal signature.
	VersionSign,
	/// String.  Read/Write. This clients security hash. Not used by TeamSpeak, SDK only. Hash is
	/// provided by an outside source. A channel will use the security salt + other client data
	/// to calculate a hash, which must be the same as the one provided here. See SDK
	/// documentation about Client / Channel Security Hashes for more details.
	SecurityHash,
	/// String.  Read only.  SDK only. List of available ciphers this client can use.
	EncryptionCiphers,

	/// Rare properties
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
	/// Automatically up-to-date for any client "in view"
	MyteamspeakId,
	/// Automatically up-to-date for any client "in view"
	Integrations,
	/// Stores info from the myts server and contains the subscription info
	ActiveIntegrationsInfo,
	MytsAvatar,
	SignedBadges,
	PermissionHints,
	/// automatically up-to-date for any client "in view", stores public chat user tag
	UserTag,
	Endmarker,

	/// (for clientlibv2) unique hardware id
	HwId = 127,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VirtualServerProperties {
	/// String.  Read only.  Unique identifier for a virtual server, does not
	/// change on server restart. Available if `ts3client_getConnectionStatus`
	/// is >= `STATUS_CONNECTED`.
	UniqueIdentifier = 0,
	/// String.  Read/Write. The virtual server display name. Available if
	/// `ts3client_getConnectionStatus` is >= `STATUS_CONNECTED`.
	Name,
	/// String.  Read/Write. The welcome message displayed to clients on connect.
	/// Available if `ts3client_getConnectionStatus` is >= `STATUS_CONNECTED`. Not
	/// updated automatically when changed, updates need to be requested (
	/// `ts3client_requestServerVariables`).
	Welcomemessage,
	/// String.  Read only.  The operating system the server is running on. Available if
	/// `ts3client_getConnectionStatus` is >= `STATUS_CONNECTED`.
	Platform,
	/// String.  Read only.  The server software version string. Available if
	/// `ts3client_getConnectionStatus` is >= `STATUS_CONNECTED`.
	Version,
	/// UInt64.  Read/Write. The maximum number of clients that can be connected
	/// simultaneously. Only available on request (`ts3client_requestServerVariables`).
	MaxClients,
	/// String.  Read/Write. The server password. Read access is limited to the server. Clients
	/// will only get the password they supplied when connecting. Available if
	/// `ts3client_getConnectionStatus` is >= `STATUS_CONNECTED`.
	Password,
	/// UInt64.  Read only.  The current number of clients connected to the server,
	/// including query connections. Only available on request (\ref
	/// ts3client_requestServerVariables).
	ClientsOnline,
	/// UInt64.  Read only.  The current number of channels on the server. Only
	/// available on request (`ts3client_requestServerVariables`).
	ChannelsOnline,
	/// Integer. Read only.  The time this virtual server was created as unix timestamp.
	/// Available if `ts3client_getConnectionStatus` is >= `STATUS_CONNECTED`.
	Created,
	/// UInt64.  Read only.  Number of seconds that have passed since the virtual server was
	/// started. Only available on request (`ts3client_requestServerVariables`).
	Uptime,
	/// Integer. Read/Write. Boolean (1/0) that specifies if voice data is encrypted
	/// during transfer. One of the values from the `CodecEncryptionMode` enum.
	/// Available if `ts3client_getConnectionStatus` is >= `STATUS_CONNECTED`.
	CodecEncryptionMode,
	/// String.  Read/Write. Comma separated list of available ciphers to encrypt the
	/// connection. The server will use the first cipher in the list that is also
	/// listed in the `CLIENT_ENCRYPTION_CIPHERS` of the connecting client.
	/// Clients will fail to connect if no match is found. Always available.
	EncryptionCiphers,

	/// Rare properties
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
	/// String.  Read only.  The path to the base directory used to store files
	/// transferred using file transfer. Available only on the server. Is set by
	/// `ts3server_enableFileManager`
	Filebase,
	/// The client permissions server group that a new client gets assigned
	DefaultServerGroup,
	/// The channel permissions group that a new client gets assigned when joining a channel
	DefaultChannelGroup,
	/// Only available on request (=> requestServerVariables)
	FlagPassword,
	/// The channel permissions group that a client gets assigned when creating a channel
	DefaultChannelAdminGroup,
	/// UInt64.  Read/Write. Maximum traffic in bytes the server can
	/// use for file transfer downloads. Only available on request
	/// (`ts3client_requestServerVariables`).
	MaxDownloadTotalBandwidth,
	/// UInt64.  Read/Write. Maximum traffic in bytes the server can
	/// use for file transfer uploads. Only available on request
	/// (`ts3client_requestServerVariables`).
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
	/// Integer. Read/Write. Boolean (1/0) indicating whether to include file
	/// transfer activities (uploading or downloading of files) in the server log.
	/// Always available.
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
	/// Only available on request (=> requestServerVariables)
	MinAndroidVersion,
	/// Only available on request (=> requestServerVariables)
	MinIosVersion,
	/// Only available on request (=> requestServerVariables)
	MinWinphoneVersion,
	/// Available when connected, always up-to-date
	Nickname,
	/// Internal use, contains base64 encoded token data
	AccountingToken,
	/// Internal use
	ProtocolVerifyKeypair,
	/// Only available on request (=> requestServerVariables)
	AntifloodPointsNeededPluginBlock,
	/// available when connected, not updated while connected
	CapabilityExtensions,
	/// Allowed filetransfer storage on this server (including chat attachments) in megabytes
	StorageQuota,
	/// internal use
	WebrtcCertificate,
	/// internal use
	WebrtcPrivateKey,
	/// the uuid of the server (uuid v5 of VIRTUALSERVER_UNIQUE_IDENTIFIER)
	Uuid,
	/// The domain which is responsible for this teamspeak server (which hosts its .well-known file)
	AdministrativeDomain,
	/// The canonical name under which the server is reachable
	CanonicalName,
	/// Only clients that have a valid mytsid can connect
	MytsidConnectOnly,
	/// How many matrix homebases this virtual server supports. -1 = no limit
	MaxHomebases,
	/// Allowed filetransfer storage for homebase attachments in megabytes
	HomebaseStorageQuota,
	Endmarker,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionProperties {
	/// UInt64. Round trip latency for the connection based on the last 5 seconds. On the server
	/// this is the average across all connected clients for the last 5 seconds.
	Ping = 0,
	/// Double. Standard deviation for the round trip latency in `CONNECTION_PING`
	PingDeviation,
	/// UInt64. Seconds the client has been connected.
	ConnectedTime,
	/// UInt64. Time in seconds since the last activity (voice transmission, switching channels,
	/// changing mic / speaker mute status) of the client.
	IdleTime,
	/// String. IP of this client (as seen from the server side)
	ClientIp,
	/// UInt64. Client side port of this client (as seen from the server side)
	ClientPort,
	/// String. The IP or hostname used to connect to the server. Only available on yourself.
	ServerIp,
	/// UInt64. The server port connected to. Only available on yourself.
	ServerPort,
	/// UInt64. The number of voice packets transmitted by the client.
	PacketsSentSpeech,
	/// UInt64. The number of keep alive packets transmitted by the client.
	PacketsSentKeepalive,
	/// UInt64. The number of command & control packets transmitted by the client.
	PacketsSentControl,
	/// UInt64. Total number of packets transmitted by the client. Equal to the sum of
	/// `CONNECTION_PACKETS_SENT_SPEECH`, `CONNECTION_PACKETS_SENT_KEEPALIVE` and
	/// `CONNECTION_PACKETS_SENT_CONTROL`
	PacketsSentTotal,
	/// UInt64. Outgoing traffic used for voice data by the client.
	BytesSentSpeech,
	/// UInt64. Outgoing traffic used for keeping the connection alive by the client.
	BytesSentKeepalive,
	/// UInt64. Outgoing traffic used for command & control data by the client.
	BytesSentControl,
	/// UInt64. Total outgoing traffic to the server by this client. Equal to the sum of
	/// `CONNECTION_BYTES_SENT_SPEECH`, `CONNECTION_BYTES_SENT_KEEPALIVE` and
	/// `CONNECTION_BYTES_SENT_CONTROL`
	BytesSentTotal,
	/// UInt64. Number of voice packets received by the client.
	PacketsReceivedSpeech,
	/// UInt64. Number of keep alive packets received by the client.
	PacketsReceivedKeepalive,
	/// UInt64. Number of command & control packets received by the client.
	PacketsReceivedControl,
	/// UInt64. Total number of packets received by the client. Equal to the sum of
	/// `CONNECTION_PACKETS_RECEIVED_SPEECH`,
	/// `CONNECTION_PACKETS_RECEIVED_KEEPALIVE` and
	/// `CONNECTION_PACKETS_RECEIVED_CONTROL`
	PacketsReceivedTotal,
	/// UInt64. Incoming traffic used by the client for voice data.
	BytesReceivedSpeech,
	/// UInt64. Incoming traffic used by the client to keep the connection alive.
	BytesReceivedKeepalive,
	/// UInt64. Incoming traffic used by the client for command & control data.
	BytesReceivedControl,
	/// UInt64. Total incoming traffic used by the client. Equal to the sum of
	/// `CONNECTION_BYTES_RECEIVED_SPEECH`, `CONNECTION_BYTES_RECEIVED_KEEPALIVE` and
	/// `CONNECTION_BYTES_RECEIVED_CONTROL`
	BytesReceivedTotal,
	/// Double. Percentage points of voice packets for the client that did not arrive at
	/// the client or server averaged across the last 5 seconds.
	PacketlossSpeech,
	/// Double. Percentage points of keep alive packets for the client that did not
	/// arrive at the client or server averaged across the last 5 seconds.
	PacketlossKeepalive,
	/// Double. Percentage points of command & control packets for the client that did
	/// not arrive at the client or server averaged across the last 5 seconds.
	PacketlossControl,
	/// Double. Cumulative chance in percentage points with which a packet round trip
	/// failed because a packet was lost
	PacketlossTotal,
	/// Double. Probability with which a voice packet sent by the server
	/// was not received by the client.
	Server2ClientPacketlossSpeech,
	/// Double. Probability with which a keepalive packet sent by the
	/// server was not received by the client.
	Server2ClientPacketlossKeepalive,
	/// Double. Probability with which a control packet sent by the server
	/// was not received by the client.
	Server2ClientPacketlossControl,
	/// Double. Probability with which a packet sent by the server was not
	/// received by the client.
	Server2ClientPacketlossTotal,
	/// Double. Probability with which a speech packet sent by the client
	/// was not received by the server.
	Client2ServerPacketlossSpeech,
	/// Double. Probability with which a keepalive packet sent by the
	/// client was not received by the server.
	Client2ServerPacketlossKeepalive,
	/// Double. Probability with which a control packet sent by the client
	/// was not received by the server.
	Client2ServerPacketlossControl,
	/// Double. Probability with which a packet sent by the client was not
	/// received by the server.
	Client2ServerPacketlossTotal,
	/// UInt64. Number of bytes sent for speech data in the last second.
	BandwidthSentLastSecondSpeech,
	/// UInt64. Number of bytes sent for keepalive data in the last second.
	BandwidthSentLastSecondKeepalive,
	/// UInt64. Number of bytes sent for control data in the last second.
	BandwidthSentLastSecondControl,
	/// UInt64. Number of bytes sent in the last second.
	BandwidthSentLastSecondTotal,
	/// UInt64. Bytes per second sent for speech data, averaged over the
	/// last minute.
	BandwidthSentLastMinuteSpeech,
	/// UInt64. Bytes per second sent for keepalive data, averaged
	/// over the last minute.
	BandwidthSentLastMinuteKeepalive,
	/// UInt64. Bytes per second sent for control data, averaged over
	/// the last minute.
	BandwidthSentLastMinuteControl,
	/// UInt64. Bytes per second sent, averaged over the last minute.
	BandwidthSentLastMinuteTotal,
	/// UInt64. Number of bytes received for speech data in the last second.
	BandwidthReceivedLastSecondSpeech,
	/// UInt64. Number of bytes received for keepalive data in the
	/// last second.
	BandwidthReceivedLastSecondKeepalive,
	/// UInt64. Number of bytes received for control data in the
	/// last second.
	BandwidthReceivedLastSecondControl,
	/// UInt64. Number of bytes received in the last second.
	BandwidthReceivedLastSecondTotal,
	/// UInt64. Bytes per second received for speech data, averaged
	/// over the last minute.
	BandwidthReceivedLastMinuteSpeech,
	/// UInt64. Bytes per second received for keepalive data,
	/// averaged over the last minute.
	BandwidthReceivedLastMinuteKeepalive,
	/// UInt64. Bytes per second received for control data, averaged
	/// over the last minute.
	BandwidthReceivedLastMinuteControl,
	/// UInt64. Bytes per second received, averaged over the last minute.
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
	/// UInt64. Current file transfer upstream activity in bytes per second.
	/// Only available on request (`ts3client_requestServerConnectionInfo`).
	FileTransferBandwidthSent,
	/// UInt64. Current file transfer downstream activity in bytes per
	/// second. Only available on request (
	/// `ts3client_requestServerConnectionInfo`).
	FiletransferBandwidthReceived,
	/// UInt64. Total downstream traffic, in bytes, used for file
	/// transfer since the server was started. Only available on request
	/// (\ref ts3client_requestServerConnectionInfo).
	FiletransferBytesReceivedTotal,
	/// UInt64. Total upstream traffic, in bytes, used for file transfer
	/// since the server was started. Only available on request (
	/// `ts3client_requestServerConnectionInfo`).
	FiletransferBytesSentTotal,
	Endmarker,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogTypes {
	/// Logging is disabled
	None         =  0,
	/// Log to regular log file
	File         =  1,
	/// Log to standard output / error
	Console      =  2,
	/// User defined logging. Will call the `ServerLibFunctions.onUserLoggingMessageEvent` callback for every message to be logged
	Userlogging  =  4,
	/// Not used
	NoNetlogging =  8,
	/// Log to database (deprecated, server only, no effect in SDK)
	Database     = 16,
	/// Log to syslog (only available on Linux)
	Syslog       = 32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum LogLevel {
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
	Devel,
}

/// Describes a client position in 3 dimensional space, used for 3D Sound.
#[repr(C)]
pub struct Ts3Vector {
	/// X co-ordinate in 3D space
	pub x: c_float,
	/// Y co-ordinate in 3D space
	pub y: c_float,
	/// Z co-ordinate in 3D space
	pub z: c_float,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupWhisperType {
	/// Whisper list consists of server groups
	Servergroup      = 0,
	/// Whisper list consists of channel groups
	Channelgroup     = 1,
	/// whisper to channel commanders
	Channelcommander = 2,
	/// whisper to all clients
	Allclients       = 3,
	Endmarker,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupWhisperTargetMode {
	All                   = 0,
	/// Whisper the current channel of the client
	Currentchannel        = 1,
	/// Whisper the parent channel of whatever channel the client is currently in
	Parentchannel         = 2,
	/// Whipser to the parent channel and all their parent channels as well
	Allparentchannel      = 3,
	/// Whisper to the current channel and all its sub channels
	Channelfamily         = 4,
	/// Whisper to the current channel, all its parent and sub channels.
	Ancestorchannelfamily = 5,
	/// Whisper to all sub channels of the current channel of the client
	Subchannels           = 6,
	Endmarker,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MonoSoundDestination {
	/// Send mono sound to all available speakers
	All               = 0,
	/// Send mono sound to front center speaker if available
	FrontCenter       = 1,
	/// Send mono sound to front left/right speakers if available
	FrontLeftAndRight = 2,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SecuritySaltOptions {
	/// Put nickname into security hash
	CheckNickname = 1,
	/// Put (game)meta data into security hash
	CheckMetaData = 2,
}

/// This enum is used to disable client commands on the server
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientCommand {
	/// disable client connection info request (client bandwidth usage, ip, port, ping)
	RequestConnectionInfo       = 0,
	/// disable moving clients
	RequestClientMove           = 1,
	/// disable muting other clients
	RequestXXMuteClients        = 2,
	/// disable kicking clients
	RequestClientKickFromXXX    = 3,
	/// disable creating channels
	FlushChannelCreation        = 4,
	/// disable editing channels
	FlushChannelUpdate          = 5,
	/// disable moving channels
	RequestChannelMove          = 6,
	/// disable deleting channels
	RequestChannelDelete        = 7,
	/// disable channel descriptions
	RequestChannelDescription   = 8,
	/// disable being able to see clients in channels other than the current channel the client is in
	RequestChannelXXSubscripeXX = 9,
	/// disable server connection info request (server bandwidth usage, ip, port, ping)
	RequestServerConnectionInfo = 10,
	/// disable text messaging
	RequestSendXXXTextMsg       = 11,
	/// disable file transfer
	FileTransfer                = 12,
	Endmarker,
}

/// Access Control List
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ACLType {
	None      = 0,
	WhiteList = 1,
	BlackList = 2,
}

/// File transfer actions
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileTransferAction {
	/// The virtual server is created. result->channelPath can be changed to create a different directory than the default 'virtualserver_x' where x is the virtual server.
	InitServer  = 0,
	/// A channel is created. result->channelPath can be changed to create a different directory then the default 'channel_x' where x is the channel id.
	InitChannel = 1,
	/// A file is being uploaded. All values in the result struct can be modified.
	Upload      = 2,
	/// A file is being downloaded. All values in the result struct can be modified.
	Download    = 3,
	/// A file is being deleted. All values in the result struct can be modified.
	Delete      = 4,
	/// A directory is being created in a channel. All values in the result struct can be modified.
	CreateDir   = 5,
	/// A file or folder is being renamed. The callback will be called twice! Once for the old and then for the new name. All values in the result struct can be modified.
	Rename      = 6,
	/// A directory listing is requested. All values in the result struct can be modified.
	FileList    = 7,
	/// Information of a file is requested. All values in the result struct can be modified.
	FileInfo    = 8,
}

/// File transfer status
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileTransferState {
	/// File transfer is establishing connection.
	Initialising = 0,
	/// File transfer is in progress
	Active,
	/// File transfer has finished
	Finished,
}

/// File transfer type
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileListType {
	/// The file entry is a directory
	Directory = 0,
	/// The file entry is a regular file
	File,
}

/// Some structs to handle variables in callbacks
#[repr(C)]
pub struct VariablesExportItem {
	/// Whether or not there is any data in this item. Ignore this item if this is 0.
	pub item_is_valid:   u8,
	/// The value in proposed is set. If 0 ignore proposed
	pub proposed_is_set: u8,
	/// Current value (stored in memory)
	pub current:         *const c_char,
	/// New value to change to (const, so no updates please)
	pub proposed:        *const c_char,
}

#[repr(C)]
pub struct VariablesExport {
	pub items: [VariablesExportItem; MAX_VARIABLES_EXPORT_COUNT],
}

#[repr(C)]
pub struct ClientMiniExport {
	/// id of the client
	pub id:       u16,
	/// the channel the client is in
	pub channel:  u64,
	/// client public identity
	pub ident:    *const c_char,
	/// client display name
	pub nickname: *const c_char,
}

/// Structure used to describe a file transfer in the \ref ServerLibFunctions.onTransformFilePath callback.
/// This describes the original values, and also contains hints for length limitations of the result parameter
/// of the callback.
/// Important: Which values of the struct can be modified is defined by the action value of the original parameter.
#[repr(C)]
pub struct TransformFilePathExport {
	/// The channel id of the file. 0 if action is \ref FT_INIT_SERVER
	pub channel:                        u64,
	/// utf8 encoded c string containing the original file name as intended by the client.
	pub filename:                       *const c_char,
	/// The action to be performed. One of the values from the \ref FTAction enum. Defines which values of the result struct can be modified.
	pub action:                         c_int,
	/// The maximum length the file name can be rewritten to.
	pub transformed_file_name_max_size: c_int,
	/// The maximum length the path can be rewritten to.
	pub channel_path_max_size:          c_int,
}

/// Structure to rewrite the file transfer file name and path in the \ref ServerLibFunctions.onTransformFilePath callback.
/// The lengths are limited as described in the original parameter.
/// Important: Which values of the struct can be modified is defined by the action value of the original parameter.
#[repr(C)]
pub struct TransformFilePathExportReturns {
	/// pointer to target file name. Fill the memory pointed to with an utf8 encoded c string containing the new file name. Limited to original->transformedFileNameMaxSize bytes.
	pub transformed_file_name: *mut c_char,
	/// pointer to memory for new path. Fill the memory pointed to with an utf8 encoded c string containing the new path. Limited to original->channelPathMaxSize bytes.
	pub channel_path:          *mut c_char,
	/// boolean (1/0). Whether to log this file transfer to the log. Action is not logged regardless of this value if the servers \ref VIRTUALSERVER_LOG_FILETRANSFER property is 0.
	pub log_file_action:       c_int,
}

#[repr(C)]
pub struct FileTransferCallbackExport {
	/// the client who started the file transfer
	pub client_id:          u16,
	/// local identifier of the transfer that has completed
	pub transfer_id:        u16,
	/// remote identifier of the transfer that has completed
	pub remote_transfer_id: u16,
	/// status of the transfer. One of the values from the \ref FileTransferState enum
	pub status:             c_uint,
	/// utf8 encoded c string containing a human readable description of the status
	pub status_message:     *const c_char,
	/// size in bytes of the complete file to be transferred
	pub remote_file_size:   u64,
	/// number of bytes transferred. Same as remotefileSize when the transfer completed entirely.
	pub bytes:              u64,
	/// boolean. 1 if the server is sending the file. 0 if the server is receiving the file.
	pub is_sender:          c_int,
}

pub const BANDWIDTH_LIMIT_UNLIMITED: u64 = std::u64::MAX;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GroupShowNameTreeMode {
	/// Dont group show name
	None = 0,
	/// Show group name before client name
	Before,
	/// Show group name behind client name
	Behind,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PluginTargetMode {
	/// Send plugincmd to all clients in current channel
	CurrentChannel = 0,
	/// Send plugincmd to all clients on server
	Server,
	/// Send plugincmd to all given client ids
	Client,
	/// Send plugincmd to all subscribed clients in current channel
	CurrentChannelSubscribedClients,
	Max,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerBinding {
	Virtualserver = 0,
	Serverquery   = 1,
	Filetransfer  = 2,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HostmessageMode {
	/// Dont display anything
	None = 0,
	/// Display message inside log
	Log,
	/// Display message inside a modal dialog
	Modal,
	/// Display message inside a modal dialog and quit/close server/connection
	Modalquit,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HostbannerMode {
	/// Do not adjust
	NoAdjust = 0,
	/// Do not adjust
	AdjustIgnoreAspect,
	/// Do not adjust
	AdjustKeepAspect,
}


#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientType {
	Normal = 0,
	Serverquery,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AwayStatus {
	None = 0,
	Zzz,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandLinePropertiesRare {
	Nothing = 0,
	Endmarker,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServerInstancePropertiesRare {
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
	ServerqueryMaxConnectionsPerIp,
	/// How many matrix homebase users this instance can have. -1 for no limit
	MaxHomebases,
	Endmarker,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LicenseIssue {
	Blacklisted,
	Greylisted,
}

bitflags! {
pub struct BBCodeTags: u32 {
	const BBCODE_B           = 0x00000001;
	const BBCODE_I           = 0x00000002;
	const BBCODE_U           = 0x00000004;
	const BBCODE_S           = 0x00000008;
	const BBCODE_SUP         = 0x00000010;
	const BBCODE_SUB         = 0x00000020;
	const BBCODE_COLOR       = 0x00000040;
	const BBCODE_SIZE        = 0x00000080;
	const BBCODE_GROUP_TEXT  = 0x000000FF;

	const BBCODE_LEFT        = 0x00001000;
	const BBCODE_RIGHT       = 0x00002000;
	const BBCODE_CENTER      = 0x00004000;
	const BBCODE_GROUP_ALIGN = 0x00007000;

	const BBCODE_URL         = 0x00010000;
	const BBCODE_IMAGE       = 0x00020000;
	const BBCODE_HR          = 0x00040000;

	const BBCODE_LIST        = 0x00100000;
	const BBCODE_LISTITEM    = 0x00200000;
	const BBCODE_GROUP_LIST  = 0x00300000;

	const BBCODE_TABLE       = 0x00400000;
	const BBCODE_TR          = 0x00800000;
	const BBCODE_TH          = 0x01000000;
	const BBCODE_TD          = 0x02000000;
	const BBCODE_GROUP_TABLE = 0x03C00000;

	const BBCODE_DEF_SIMPLE     = Self::BBCODE_B.bits() | Self::BBCODE_I.bits()
		| Self::BBCODE_U.bits() | Self::BBCODE_S.bits() | Self::BBCODE_SUP.bits()
		| Self::BBCODE_SUB.bits() | Self::BBCODE_COLOR.bits()
		| Self::BBCODE_URL.bits();
	const BBCODE_DEF_SIMPLE_IMG = Self::BBCODE_DEF_SIMPLE.bits()
		| Self::BBCODE_IMAGE.bits();
	const BBCODE_DEF_EXTENDED   = Self::BBCODE_GROUP_TEXT.bits()
		| Self::BBCODE_GROUP_ALIGN.bits() | Self::BBCODE_URL.bits()
		| Self::BBCODE_IMAGE.bits() | Self::BBCODE_HR.bits()
		| Self::BBCODE_GROUP_LIST.bits() | Self::BBCODE_GROUP_TABLE.bits();
}}

bitflags! {
pub struct MytsDataUnsetFlags: u32 {
	const BADGES = 1;
	const AVATAR = 2;
}}


// As they are only typedefs and I didn't found any usage, I'll just leave them here for now
//typedef int(*ExtraBBCodeValidator)(void* userparam, const char* tag, const char* paramValue, int paramValueSize, const char* childValue, int childValueSize);
//typedef const char* (*ExtraBBCodeParamTransform)(void* userparam, const char* tag, const char* paramValue);
