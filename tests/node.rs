extern crate spring_dvs;

use spring_dvs::node::*;

#[test]
fn ts_node_from_str_format_node_single_pass() {
	let o = Node::from_str("foobar");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
}

#[test]
fn ts_node_from_str_format_node_double_pass() {
	let o = Node::from_str("foobar,barfoo");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.hostpath(), "");
}

#[test]
fn ts_node_from_str_format_node_double_2_pass() {
	let o = Node::from_str("foobar,barfoo/path");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.hostpath(), "path");
}

#[test]
fn ts_node_from_str_format_node_triple_pass() {
	let o = Node::from_str("foobar,barfoo,127.3.4.5");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.address(), "127.3.4.5");
	assert_eq!(n.hostpath(), "");
}

#[test]
fn ts_node_from_str_format_node_triple_2_pass() {
	let o = Node::from_str("foobar,barfoo/path,127.3.4.5");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.hostpath(), "path");
	assert_eq!(n.address(), "127.3.4.5");
}

#[test]
fn ts_node_from_str_format_node_quad_pass() {
	let o = Node::from_str("foobar,barfoo,127.3.4.5,http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.hostpath(), "");
	assert_eq!(n.address(), "127.3.4.5");
	assert_eq!(n.service(), NodeService::Http);
}


#[test]
fn ts_node_from_str_format_node_quad_2_pass() {
	let o = Node::from_str("foobar,barfoo/path,127.3.4.5,http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.hostpath(), "path");
	assert_eq!(n.address(), "127.3.4.5");
	assert_eq!(n.service(), NodeService::Http);
}

#[test]
fn ts_node_from_str_format_node_info_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo,address:127.3.4.5,role:hybrid");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.hostpath(), "");
	assert_eq!(n.address(), "127.3.4.5");
	assert_eq!(n.role(), NodeRole::Hybrid);
}

#[test]
fn ts_node_from_str_format_node_info_2_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo/path,address:127.3.4.5,role:hybrid");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	assert_eq!(n.springname(), "foobar");
	assert_eq!(n.hostname(), "barfoo");
	assert_eq!(n.hostpath(), "path");
	assert_eq!(n.address(), "127.3.4.5");
	assert_eq!(n.role(), NodeRole::Hybrid);
}

#[test]
fn ts_node_from_str_format_node_info_fail() {
	let o = Node::from_str("spring:foobar,hosting:barfoo,address:127.3.4.5,role:hybrid");
	assert!(o.is_err());
	let o = Node::from_str("spring:foobar,hostbarfoo,address:127.3.4.5,role:hybrid");
	assert!(o.is_err());
}

#[test]
fn ts_node_to_node_single_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let ns : NodeSingleFmt = match n.to_node_single() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(ns.spring, "foobar");
}

#[test]
fn ts_node_to_node_single_fail() {
	let o = Node::from_str("host:barfoo,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	
	assert_eq!(n.to_node_single(), None);
}

#[test]
fn ts_node_to_node_double_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let f : NodeDoubleFmt = match n.to_node_double() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(f.spring, "foobar");
	assert_eq!(f.host, "barfoo");
}

#[test]
fn ts_node_to_node_double_2_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo/path,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let f : NodeDoubleFmt = match n.to_node_double() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(f.spring, "foobar");
	assert_eq!(f.host, "barfoo/path");
}

#[test]
fn ts_node_to_node_double_fail() {
	let o = Node::from_str("spring:foobar,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	
	assert_eq!(n.to_node_double(), None);
}

#[test]
fn ts_node_to_node_triple_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let f : NodeTripleFmt = match n.to_node_triple() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(f.spring, "foobar");
	assert_eq!(f.host, "barfoo");
	assert_eq!(f.address, "127.3.4.5");
}

#[test]
fn ts_node_to_node_triple_2_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo/path,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let f : NodeTripleFmt = match n.to_node_triple() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(f.spring, "foobar");
	assert_eq!(f.host, "barfoo/path");
	assert_eq!(f.address, "127.3.4.5");
}

#[test]
fn ts_node_to_node_triple_fail() {
	let o = Node::from_str("spring:foobar,host:barfoo,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	
	assert_eq!(n.to_node_triple(), None);
}

#[test]
fn ts_node_to_node_quad_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let f : NodeQuadFmt = match n.to_node_quad() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(f.spring, "foobar");
	assert_eq!(f.host, "barfoo");
	assert_eq!(f.address, "127.3.4.5");
	assert_eq!(f.service, NodeService::Http);
}

#[test]
fn ts_node_to_node_quad_2_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo/path,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let f : NodeQuadFmt = match n.to_node_quad() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(f.spring, "foobar");
	assert_eq!(f.host, "barfoo/path");
	assert_eq!(f.address, "127.3.4.5");
	assert_eq!(f.service, NodeService::Http);
}


#[test]
fn ts_node_to_node_quad_fail() {
	let o = Node::from_str("spring:foobar,role:hybrid,state:enabled");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	
	assert_eq!(n.to_node_quad(), None);
}

#[test]
fn ts_node_to_node_info_pass() {
	let o = Node::from_str("spring:foobar,host:barfoo,address:127.3.4.5,role:hybrid,state:enabled,service:http");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	let f : NodeInfoFmt = match n.to_node_info() {
		Some(n) => n,
		None => return
	};
	
	assert_eq!(f.spring, "foobar");
	assert_eq!(f.host, "barfoo");
	assert_eq!(f.address, "127.3.4.5");
	assert_eq!(f.service, NodeService::Http);
	assert_eq!(f.state, NodeState::Enabled);
	assert_eq!(f.role, NodeRole::Hybrid);
}

#[test]
fn ts_node_to_node_info_fail() {
	let o = Node::from_str("spring:foobar,role:hybrid,state:enabled");
	assert!(o.is_ok());
	
	let n : Node = o.unwrap();
	
	assert_eq!(n.to_node_info(), None);
}

#[test]
fn ts_nodevec_quadvec() {
	let  v = vec![
		Node::from_str("foobar,barfoo,127.3.4.5,http").unwrap(),
		Node::from_str("barfoo,barfoo,127.3.4.5,http").unwrap(),
		Node::from_str("farboo,barfoo,127.3.4.5,http").unwrap(),
	];
	
	let qv = nodevec_quadvec(v);
	
	assert_eq!(qv.len(), 3);
	assert_eq!(qv[0].spring, "foobar");
	assert_eq!(qv[1].spring, "barfoo");
	assert_eq!(qv[2].spring, "farboo");
	
		
}