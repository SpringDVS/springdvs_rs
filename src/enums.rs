/* Notice:	Copyright 2016, The Care Connections Initiative c.i.c.
 * Author: 	Charlie Fyvie-Gauld (cfg@zunautica.org)
 * License: GPLv3 (http://www.gnu.org/licenses/gpl-3.0.txt)
 */

use std::fmt;

// ----- Enumeration Lists ----- \\
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DvspCmdType {
	Undefined = 0,
	GsnRegistration = 1,
	GsnResolution = 2,
	GsnArea = 3,
	GsnState = 4,
	GsnNodeInfo = 5,
	GsnNodeStatus = 6,
	GsnRequest = 7,
	GsnTypeRequest = 8,
	
	GtnRegistration = 22,
	GtnGeosubNodes = 23,

	GsnResponse = 30,
	GsnResponseNodeInfo = 31,
	GsnResponseNetwork = 32,
	GsnResponseHigh = 33,
	GsnResponseStatus = 34,
	
	UnitTest = 101,

}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Response {
	NetspaceError,
	NetspaceDuplication,
	NetworkError,
	UnsupportedAction,
	UnsupportedService,
	MalformedContent,
	Ok,
	Chunk,
	Eot,
}

impl Response {
	pub fn from_str(s: &str) -> Option<Self> {
		match s {
			"101" => Some(Response::NetspaceError),
			"102" => Some(Response::NetspaceDuplication),
			"103" => Some(Response::NetworkError),
			"104" => Some(Response::MalformedContent),
			
			"121" => Some(Response::UnsupportedAction),
			"122" => Some(Response::UnsupportedService),

			"200" => Some(Response::Ok),
			"201" => Some(Response::Chunk),
			"202" => Some(Response::Eot), 
			
			_ => None,
		}
	}
}

impl fmt::Display for Response {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let out = match *self {
				Response::NetspaceError => "101",
				Response::NetspaceDuplication => "102",
				Response::NetworkError => "103",
				Response::MalformedContent => "104",

				Response::UnsupportedAction => "121",
				Response::UnsupportedService => "122",

				Response::Ok => "200",
				Response::Chunk => "201",
				Response::Eot => "202",
		};
		write!(f, "{}", out)
	} 
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NodeRole {
	Undefined = 0,
	Hub = 1,
	Org = 2,
	Hybrid = 3,
}


impl NodeRole {
	pub fn from_str(s: &str) -> Option<Self> {
		match s {
			"org" => Some(NodeRole::Org),
			"hub" => Some(NodeRole::Hub),
			"hybrid" => Some(NodeRole::Hybrid),
			_ => None,
		}
	}
	
	pub fn from_i64(n: i64) -> Self {
		match n {
			1 => NodeRole::Hub,
			2 => NodeRole::Org,
			3 => NodeRole::Hybrid,
			_ => NodeRole::Undefined,
		}
	}
}

impl fmt::Display for NodeRole {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let out = match self {
				&NodeRole::Hub => "hub",
				&NodeRole::Org => "org",
				&NodeRole::Hybrid => "hybrid",
				_ => "undefined",				
		};
		write!(f, "{}", out)
	} 
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NodeService {
	Undefined = 0,
	Dvsp = 1,
	Http = 2,
}

impl NodeService {
	pub fn from_str(s: &str) -> Option<Self> {
		match s {
			"dvsp" => Some(NodeService::Dvsp),
			"http" => Some(NodeService::Http),
			_ => None,
		}
	}
	
	pub fn from_i64(n: i64) -> Self {
		match n {
			1 => NodeService::Dvsp,
			2 => NodeService::Http,
			_ => NodeService::Undefined,
		}
	}
}

impl fmt::Display for NodeService {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let out = match self {
				&NodeService::Dvsp => "dvsp",
				&NodeService::Http => "http",
				_ => "undefined",							
		};
		write!(f, "{}", out)
	} 
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NodeState {
	Disabled = 0,
	Enabled = 1,
	Unresponsive = 2,
	Unspecified = 3,
}

impl NodeState {
	pub fn from_str(s: &str) -> Option<Self> {
		match s {
			"disabled" => Some(NodeState::Disabled),
			"enabled" => Some(NodeState::Enabled),
			"unresponsive" => Some(NodeState::Unresponsive),
			"unspecified" => Some(NodeState::Unspecified),
			_ => None,
		}
	}

	pub fn from_i64(n: i64) -> Self {
		match n {
			1 => NodeState::Enabled,
			2 => NodeState::Unresponsive,
			0 => NodeState::Disabled,
			_ => NodeState::Unspecified,
		}
	}
}

impl fmt::Display for NodeState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let out = match self {
				&NodeState::Disabled => "disabled",
				&NodeState::Enabled => "enabled",
				&NodeState::Unresponsive => "unresponsive",
				&NodeState::Unspecified => "unspecified",							
		};
		write!(f, "{}", out)
	} 
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Bounds {
	MaxNodeType = 3,
	PacketContentSize = 512,
	FrameRegisterLen = 124,
	NodeRegister = 125,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Failure {
	OutOfBounds, InvalidArgument, InvalidBytes, InvalidConversion, InvalidFormat,
	Duplicate,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParseFailure {
	ConversionError,
	UnexpectedContent,
	InvalidCommand,
	InvalidContentFormat,
	InvalidInternalState,
	InvalidRole,
	InvalidNaming,
	InvalidService,
	InvalidAddress,
	InvalidState,
	InvalidProperty,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Success {
	Ok	
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UnitTestAction {
	Undefined = 0,
	Reset = 1,
	UpdateAddress = 2,
	AddGeosubRoot = 3,
}
