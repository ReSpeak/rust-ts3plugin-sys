#![allow(dead_code)]
// Uses lots of alignment which rustfmt destroys
#![cfg_attr(rustfmt, rustfmt::skip)]

//! This file contains the definitions of public_erros.h and public_erros_rare.h

use ::std::fmt;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
	/// General
	/// Indicates success.
	Ok                                          = 0x0000,
	Undefined                                   = 0x0001,
	/// The attempted operation is not available in this context
	NotImplemented                              = 0x0002,
	/// Indicates success, but no change occurred. Returned for example upon flushing (e.g. using `ts3client_flushChannelUpdates`) when all indicated changes already matched the current state.
	OkNoUpdate                                  = 0x0003,
	DontNotify                                  = 0x0004,
	LibTimeLimitReached                         = 0x0005,
	/// Not enough system memory to perform operation
	OutOfMemory                                 = 0x0006,
	Canceled                                    = 0x0007,

	/// Dunno
	CommandNotFound                             = 0x0100,
	/// Unspecified failure to create a listening port
	UnableToBindNetworkPort                     = 0x0101,
	/// Failure to initialize a listening port for FileTransfer
	NoNetworkPortAvailable                      = 0x0102,
	/// Specified port is already in use by a different application
	PortAlreadyInUse                            = 0x103,

	/// Client
	/// Client no longer connected
	ClientInvalidId                             = 0x0200,
	/// Client name is already in use. Client names must be unique
	ClientNicknameInuse                         = 0x0201,
	/// Too many clients on the server
	ClientProtocolLimitReached                  = 0x0203,
	/// Function called for normal clients that is only available for query clients or vice versa
	ClientInvalidType                           = 0x0204,
	/// Attempting to subscribe to a channel already subsribed to
	ClientAlreadySubscribed                     = 0x0205,
	ClientNotLoggedIn                           = 0x0206,
	/// Identity not valid or insufficient security level
	ClientCouldNotValidateIdentity              = 0x0207,
	ClientInvalidPassword                       = 0x0208,
	ClientTooManyClonesConnected                = 0x0209,
	/// Server requires newer client version as determined by the min_client_version properties
	ClientVersionOutdated                       = 0x020a,
	ClientIsOnline                              = 0x020b,
	/// Triggered flood protection. Further information is supplied in the extra message if applicable.
	ClientIsFlooding                            = 0x020c,
	ClientHacked                                = 0x020d,
	ClientCannotVerifyNow                       = 0x020e,
	ClientLoginNotPermitted                     = 0x020f,
	/// Action is only available on subscribed channels
	ClientNotSubscripted                        = 0x0210,

	/// Channel
	/// Channel does not exist on the server (any longer)
	ChannelInvalidId                            = 0x0300,
	/// Too many channels on the server
	ChannelProtocolLimitReached                 = 0x0301,
	/// Attempting to move a client or channel to its current channel
	ChannelAlreadyIn                            = 0x0302,
	/// Channel name is already taken by another channel. Channel names must be unique
	ChannelnameInuse                            = 0x0303,
	/// Attempting to delete a channel with clients or sub channels in it
	ChannelNotEmpty                             = 0x0304,
	/// Default channel cannot be deleted. Set a new default channel first (see `ts3client_setChannelVariableAsInt` or `ts3server_setChannelVariableAsInt` )
	ChannelCannotDeleteDefault                  = 0x0305,
	/// Attempt to set a non permanent channel as default channel. Set channel to permanent first (see `ts3client_setChannelVariableAsInt` or `ts3server_setChannelVariableAsInt` )
	ChannelDefaultRequirePermanent              = 0x0306,
	/// Invalid combination of `ChannelProperties`, trying to remove `CHANNEL_FLAG_DEFAULT` or set a password on the default channel
	ChannelInvalidFlags                         = 0x0307,
	/// Attempt to move a permanent channel into a non-permanent one, or set a channel to be permanent that is a sub channel of a non-permanent one
	ChannelParentNotPermanent                   = 0x0308,
	/// Channel is full as determined by its `CHANNEL_MAXCLIENTS` setting
	ChannelMaxclientsReached                    = 0x0309,
	/// Channel tree is full as determined by its `CHANNEL_MAXFAMILYCLIENTS` setting
	ChannelMaxfamilyReached                     = 0x030a,
	/// Invalid value for the `CHANNEL_ORDER` property. The specified channel must exist on the server and be on the same level.
	ChannelInvalidOrder                         = 0x030b,
	/// Invalid `CHANNEL_FILEPATH` set for the channel
	ChannelNoFiletransferSupported              = 0x030c,
	/// Channel has a password not matching the password supplied in the call
	ChannelInvalidPassword                      = 0x030d,
	ChannelIsPrivateChannel                     = 0x030e,
	ChannelInvalidSecurityHash                  = 0x030f,

	/// Server
	/// Chosen virtual server does not exist or is offline
	ServerInvalidId                             = 0x0400,
	/// attempting to delete a server that is running. Stop the server before deleting it.
	ServerRunning                               = 0x0401,
	/// Client disconnected because the server is going offline
	ServerIsShuttingDown                        = 0x0402,
	/// Given in the onConnectStatusChange event when the server has reached its maximum number of clients as defined by the \ref VIRTUALSERVER_MAXCLIENTS property
	ServerMaxclientsReached                     = 0x0403,
	/// Specified server password is wrong. Provide the correct password in the \ref ts3client_startConnection / \ref ts3client_startConnectionWithChannelID call.
	ServerInvalidPassword                       = 0x0404,
	ServerDeploymentActive                      = 0x0405,
	ServerUnableToStopOwnServer                 = 0x0406,
	/// Server is in virtual status. The attempted action is not possible in this state. Start the virtual server first.
	ServerIsVirtual                             = 0x0407,
	ServerWrongMachineid                        = 0x0408,
	/// Attempting to stop a server that is not online.
	ServerIsNotRunning                          = 0x0409,
	ServerIsBooting                             = 0x040a,
	ServerStatusInvalid                         = 0x040b,
	ServerModalQuit                             = 0x040c,
	/// Attempt to connect to an outdated server version. The server needs to be updated.
	ServerVersionOutdated                       = 0x040d,
	/// This server is already running within the instance. Each virtual server may only exist once.
	ServerDuplicatedRunning                     = 0x040e,
	ServerTimeDifferenceTooLarge                = 0x040f,
	ServerBlacklisted                           = 0x0410,
	ServerShutdown                              = 0x0411,

	/// Database
	Database                                    = 0x0500,
	DatabaseEmptyResult                         = 0x0501,
	DatabaseDuplicateEntry                      = 0x0502,
	DatabaseNoModifications                     = 0x0503,
	DatabaseConstraint                          = 0x0504,
	DatabaseReinvoke                            = 0x0505,

	/// Parameter
	ParameterQuote                              = 0x0600,
	/// Attempt to flush changes without previously calling set*VariableAs* since the last flush
	ParameterInvalidCount                       = 0x0601,
	/// At least one of the supplied parameters did not meet the criteria for that parameter
	ParameterInvalid                            = 0x0602,
	/// Failure to supply all the necessary parameters
	ParameterNotFount                           = 0x0603,
	/// Invalid type supplied for a parameter, such as passing a string (ie. "five") that expects a number.
	ParameterConvert                            = 0x0604,
	/// Value out of allowed range. Such as strings are too long/short or numeric values outside allowed range
	ParameterInvalidSize                        = 0x0605,
	/// Neglecting to specify a required parameter
	ParameterMissing                            = 0x0606,
	/// Attempting to deploy a modified snapshot
	ParameterChecksum                           = 0x0607,

	/// Unsorted, needs further investigation
	/// Failure to create default channel
	VsCritical                                  = 0x0700,
	/// Generic error with the connection.
	ConnectionLost                              = 0x0701,
	/// Attempting to call functions with a serverConnectionHandler that is not connected. You can use \ref ts3client_getConnectionStatus to check whether the connection handler is connected to a server
	NotConnected                                = 0x0702,
	/// Attempting to query connection information (bandwidth usage, ping, etc) without requesting them first using \ref ts3client_requestConnectionInfo
	NoCachedConnectionInfo                      = 0x0703,
	/// Requested information is not currently available. You may have to call \ref ts3client_requestClientVariables or \ref ts3client_requestServerVariables
	CurrentlyNotPossible                        = 0x0704,
	/// No TeamSpeak server running on the specified IP address and port
	FailedConnectionInitialisation              = 0x0705,
	/// Failure to resolve the specified hostname to an IP address
	CouldNotResolveHostname                     = 0x0706,
	/// Attempting to perform actions on a non-existent server connection handler
	InvalidServerConnectionoHandlerId           = 0x0707,
	/// Not used
	CouldNotInitialiseInputManager              = 0x0708,
	/// Calling client library functions without successfully calling \ref ts3client_initClientLib before
	ClientlibraryNotInitialised                 = 0x0709,
	/// Calling server library functions without successfully calling \ref ts3server_initServerLib before
	ServerlibraryNotInitialised                 = 0x070a,
	/// Using a whisper list that contain more clients than the servers \ref VIRTUALSERVER_MIN_CLIENTS_IN_CHANNEL_BEFORE_FORCED_SILENCE property
	WhisperTooManyTargets                       = 0x070b,
	/// The active whisper list is empty or no clients matched the whisper list (e.g. all channels in the list are empty)
	WhisperNoTargets                            = 0x070c,
	/// Invalid or unsupported protocol (e.g. attempting an IPv6 connection on an IPv4 only machine)
	ConnectionIpProtocolMissing                 = 0x070d,
	// 0x070e is reserved
	IllegalServerLicense                        = 0x070f,

	/// File transfer
	/// Invalid UTF8 string or not a valid file
	FileInvalidName                             = 0x0800,
	/// Permissions prevent opening the file
	FileInvalidPermissions                      = 0x0801,
	/// Target path already exists as a directory
	FileAlreadyExists                           = 0x0802,
	/// Attempt to access or move non existing file
	FileNotFound                                = 0x0803,
	/// Generic file input / output error
	FileIoError                                 = 0x0804,
	/// Attempt to get information about a file transfer after it has already been cleaned up. File transfer information is not available indefinitely after the transfer completed
	FileInvalidTransferId                       = 0x0805,
	/// specified path contains invalid characters or does not start with "/"
	FileInvalidPath                             = 0x0806,
	/// Not used
	FileNoFilesAvailable                        = 0x0807,
	/// File overwrite and resume are mutually exclusive. Only one or neither can be 1.
	FileOverwriteExcludesResume                 = 0x0808,
	/// Attempt to write more bytes than claimed file size.
	FileInvalidSize                             = 0x0809,
	/// File is currently not available, try again later.
	FileAlreadyInUse                            = 0x080a,
	/// Generic failure in file transfer connection / other party did not conform to file transfer protocol
	FileCouldNotOpenConnection                  = 0x080b,
	/// Operating system reports hard disk is full. May be caused by quota limitations.
	FileNoSpaceLeftOnDevice                     = 0x080c,
	/// File is too large for the file system of the target device.
	FileExceedsFileSystemMaximumSize            = 0x080d,
	/// Not used
	FileTransferConnectionTimeout               = 0x080e,
	/// File input / output timeout or connection failure
	FileConnectionLost                          = 0x080f,
	/// Not used
	FileExceedsSuppliedSize                     = 0x0810,
	/// Indicates successful completion
	FileTransferComplete                        = 0x0811,
	/// Transfer was cancelled through @ref ts3client_haltTransfer
	FileTransferCanceled                        = 0x0812,
	/// Transfer failed because the server is shutting down, or network connection issues
	FileTransferInterrupted                     = 0x0813,
	/// Transfer terminated due to server bandwidth quota being exceeded. No client can transfer files.
	FileTransferServerQuotaExceeded             = 0x0814,
	/// Attempt to transfer more data than allowed by this clients' bandwidth quota. Other clients may continue to transfer files.
	FileTransferClientQuotaExceeded             = 0x0815,
	/// Not used
	FileTransferReset                           = 0x0816,
	/// Too many file transfers are in progress. Try again later
	FileTransferLimitReached                    = 0x0817,
	/// TODO: Invalid storage class for HTTP FileTransfer (what is a storage class?)
	FileInvalidStorageClass                     = 0x0818,
	/// Avatar image exceeds maximum width or height accepted by the server.
	FileInvalidDimension                        = 0x0819,
	/// Transfer failed because the channel quota was exceeded. Uploading to this channel is not possible, but other channels may be fine.
	FileTransferChannelQuotaExceeded            = 0x081a,

	/// Sound
	/// Cannot set or query pre processor variables with preprocessing disabled
	SoundPreprocessorDisabled                   = 0x09_00,
	SoundInternalPreprocessor                   = 0x09_01,
	SoundInternalEncoder                        = 0x09_02,
	SoundInternalPlayback                       = 0x09_03,
	/// No audio capture devices are available
	SoundNoCaptureDeviceAvailable               = 0x09_04,
	/// No audio playback devices are available
	SoundNoPlaybackDeviceAvailable              = 0x09_05,
	/// Error accessing audio device, or audio device does not support the requested mode
	SoundCouldNotOpenCaptureDevice              = 0x09_06,
	/// Error accessing audio device, or audio device does not support the requested mode
	SoundCouldNotOpenPlaybackDevice             = 0x09_07,
	/// Attempt to open a sound device on a connection handler which already has an open device. Close the already open device first using \ref ts3client_closeCaptureDevice or \ref ts3client_closePlaybackDevice
	SoundHandlerHasDevice                       = 0x09_08,
	/// Attempt to use a device for capture that does not support capturing audio
	SoundInvalidCaptureDevice                   = 0x09_09,
	/// Attempt to use a device for playback that does not support playback of audio
	SoundInvalidPlaybackDevice                  = 0x09_0a,
	/// Attempt to use a non WAV file in \ref ts3client_playWaveFile or \ref ts3client_playWaveFileHandle
	SoundInvalidWave                            = 0x09_0b,
	/// Unsupported wave file used in \ref ts3client_playWaveFile or \ref ts3client_playWaveFileHandle.
	SoundUnsupportedWave                        = 0x09_0c,
	/// Failure to open the specified sound file
	SoundOpenWave                               = 0x09_0d,
	SoundInternalCapture                        = 0x09_0e,
	/// Attempt to unregister a custom device that is being used. Close the device first using \ref ts3client_closeCaptureDevice or \ref ts3client_closePlaybackDevice
	SoundDeviceInUse                            = 0x09_0f,
	/// Attempt to register a custom device with a device id that has already been used in a previous call. Device ids must be unique.
	SoundDeviceAlreadyRegisterred               = 0x09_10,
	/// Attempt to open, close, unregister or use a device which is not known. Custom devices must be registered before being used (see \ref ts3client_registerCustomDevice)
	SoundUnknownDevice                          = 0x09_11,
	SoundUnsupportedFrequency                   = 0x09_12,
	/// Invalid device audio channel count, must be > 0
	SoundInvalidChannelCount                    = 0x09_13,
	/// Failure to read sound samples from an opened wave file. Is this a valid wave file?
	SoundReadWave                               = 0x09_14,
	/// for internal purposes only
	SoundNeedMoreData                           = 0x09_15,
	/// for internal purposes only
	SoundDeviceBusy                             = 0x09_16,
	/// Indicates there is currently no data for playback, e.g. nobody is speaking right now.
	SoundNoData                                 = 0x09_17,
	/// Opening a device with an unsupported channel count
	SoundChannelMaskMismatch                    = 0x09_18,

	/// Permissions
	PermissionsInvalidGroupId                   = 0x0a_00,
	PermissionsDuplicateEntry                   = 0x0a_01,
	PermissionsInvalidPermId                    = 0x0a_02,
	PermissionsEmptyResult                      = 0x0a_03,
	PermissionsDefaultGroupForbidden            = 0x0a_04,
	PermissionsInvalidSize                      = 0x0a_05,
	PermissionsInvalidValue                     = 0x0a_06,
	PermissionsGroupNotEmpty                    = 0x0a_07,
	/// Not enough permissions to perform the requested activity
	PermissionsClientInsufficient               = 0x0a_08,
	PermissionsInsufficientGroupPower           = 0x0a_09,
	PermissionsInsufficientPermissionPower      = 0x0a_0a,
	PermissionsTemplateGroupIsUsed              = 0x0a_0b,
	/// Permissions to use sound device not granted by operating system, e.g. Windows denied microphone access.
	Permissions                                 = 0x0a_0c,
	PermissionUsedByIntegration                 = 0x0a_0d,

	/// Accounting
	/// Attempt to use more virtual servers than allowed by the license
	AccountingVirtualserverLimitReached         = 0x0b_00,
	/// Attempt to set more slots than allowed by the license
	AccountingSlotLimitReached                  = 0x0b_01,
	/// Not used
	AccountingLicenseFileNotFound               = 0x0b_02,
	/// License expired or not valid yet
	AccountingLicenseDateNotOk                  = 0x0b_03,
	/// Failure to communicate with accounting backend
	AccountingUnableToConnectToServer           = 0x0b_04,
	/// Failure to write update license file
	AccountingUnknownError                      = 0x0b_05,
	/// Not used
	AcountingServerError                        = 0x0b_06,
	/// More than one process of the server is running
	AccountingInstanceLimitReached              = 0x0b_07,
	/// Shared memory access failure.
	AccountingInstanceCheckError                = 0x0b_08,
	/// License is not a TeamSpeak license
	AccountingLicenseFileInvalid                = 0x0b_09,
	/// A copy of this server is already running in another instance. Each server may only exist once.
	AccountingRunningElsewhere                  = 0x0b_0a,
	/// A copy of this server is running already in this process. Each server may only exist once.
	AccountingInstanceDuplicated                = 0x0b_0b,
	/// Attempt to start a server that is already running
	AccountingAlreadyStarted                    = 0x0b_0c,
	AccountingNotStarted                        = 0x0b_0d,
	/// Starting instance / virtual servers too often in too short a time period
	AccountingToManyStarts                      = 0x0b_0e,

	/// Messages
	MessageInvalidId                            = 0x0c_00,

	/// Ban
	BanInvalidId                                = 0x0d_00,
	ConnectFailedBanned                         = 0x0d_01,
	RenameFailedBanned                          = 0x0d_02,
	BanFlooding                                 = 0x0d_03,

	/// Text to speech
	TtsUnableToInitialize                       = 0x0e_00,

	/// Privilege key
	PrivilegeKeyInvalid                         = 0x0f_00,

	/// Voip
	VoipPjsua                                   = 0x10_00,
	VoipAlreadyInitialized                      = 0x10_01,
	VoipTooManyAccounts                         = 0x10_02,
	VoipInvalidAccount                          = 0x10_03,
	VoipInternalError                           = 0x10_04,
	VoipInvalidConnectionId                     = 0x10_05,
	VoipCannotAnswerInitiatedCall               = 0x10_06,
	VoipNotInitialized                          = 0x10_07,

	/// Provisioning server
	ProvisioningInvalidPassword                 = 0x11_00,
	ProvisioningInvalidRequest                  = 0x11_01,
	ProvisioningNoSlotsAvailable                = 0x11_02,
	ProvisioningPoolMissing                     = 0x11_03,
	ProvisioningPoolUnkown                      = 0x11_04,
	ProvisioningUnknownIpLocation               = 0x11_05,
	ProvisioningInternalTriedExceeded           = 0x11_06,
	ProvisioningTooManySlotsRequested           = 0x11_07,
	ProvisioningTooManyReserved                 = 0x11_08,
	ProvisioningCouldNotConnect                 = 0x11_09,
	ProvisioningAuthServerNotConnected          = 0x11_10,
	ProvisioningAuthDataTooLarge                = 0x11_11,
	ProvisioningAlreadyInitialized              = 0x11_12,
	ProvisioningNotInitialized                  = 0x11_13,
	ProvisioningConnecting                      = 0x11_14,
	ProvisioningAlreadyConnected                = 0x11_15,
	ProvisioningNotConnected                    = 0x11_16,
	ProvisioningIoError                         = 0x11_17,
	ProvisioningInvalidTimeout                  = 0x11_18,
	ProvisioningTs3severNotFound                = 0x11_19,
	ProvisioningNoPermission                    = 0x11_1A,

	/// Mytsid - client
	InvalidMytsidData                           = 0x12_00,
	InvalidIntegration                          = 0x12_01,
	MytsidNeeded                                = 0x12_02,

	/// Ed25519
	Ed25519RngFail                              = 0x13_00,
	Ed25519SignatureInvalid                     = 0x13_01,
	Ed25519InvalidKey                           = 0x13_02,
	Ed25519UnableToCreateValidKey               = 0x13_03,
	Ed25519OutOfMemory                          = 0x13_04,
	Ed25519Exists                               = 0x13_05,
	Ed25519ReadBeyondEof                        = 0x13_06,
	Ed25519WriteBeyondEof                       = 0x13_07,
	Ed25519Version                              = 0x13_08,
	Ed25519Invalid                              = 0x13_09,
	Ed25519InvalidDate                          = 0x13_0a,
	Ed25519Unauthorized                         = 0x13_0b,
	Ed25519InvalidType                          = 0x13_0c,
	Ed25519AddressNomatch                       = 0x13_0d,
	Ed25519NotValidYet                          = 0x13_0e,
	Ed25519Expired                              = 0x13_0f,
	Ed25519IndexOutOfRange                      = 0x13_10,
	Ed25519InvalidSize                          = 0x13_11,

	// Api key
	ApikeyOutofscope                            = 0x14_00,
	ApikeyCryptoError                           = 0x14_01,
	ApikeyInvalid                               = 0x14_02,
	ApikeyInvalidId                             = 0x14_03,
	ApikeyMissing                               = 0x14_04,

	// Homebase
	HomebaseNoSlotsAvailable                    = 0x15_00,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl ::std::error::Error for Error {
	fn description(&self) -> &str {
		"TeamSpeak error"
	}
}
