use util::*;

#[derive(Clone, Debug)]
pub struct Cpu {
	id: usize,
	core_id: usize,
	physical_package_id: usize,
	core_siblings: Vec<bool>,
	thread_siblings: Vec<bool>,
	node_id: usize,
}

impl Cpu {
	pub fn new(node: usize, id: usize) -> Result<Cpu, &'static str> {
		let cpu = Cpu {
			id: id,
			core_id: 0,
			physical_package_id: 0,
			core_siblings: Vec::new(),
			thread_siblings: Vec::new(),
			node_id: node,
		};
		cpu.init()
	}

	fn init(mut self) -> Result<Self, &'static str> {
		if let Ok(core_id) = get_core_id(self.node_id, self.id) {
			self.core_id = core_id;
		} else {
			return Err("invalid core_id");
		}
		if let Ok(physical_package_id) = get_physical_package_id(self.id) {
			self.physical_package_id = physical_package_id;
		} else {
			return Err("invalid physical_package_id");
		}
		if let Ok(core_siblings) = get_core_siblings(self.id) {
			self.core_siblings = core_siblings;
		} else {
			return Err("invalid core_siblings");
		}
		if let Ok(thread_siblings) = get_thread_siblings(self.id) {
			self.thread_siblings = thread_siblings;
		} else {
			return Err("invalid thread_siblings");
		}
		Ok(self)
	}

	pub fn id(&self) -> usize {
		self.id
	}

	pub fn core_id(&self) -> usize {
		self.core_id
	}

	pub fn set_node_id(&mut self, node_id: usize) {
		self.node_id = node_id;
	}

	pub fn node_id(&self) -> usize {
		self.node_id
	}

	pub fn physical_package_id(&self) -> usize {
		self.physical_package_id
	}

	pub fn core_siblings(&self) -> Vec<bool> {
		self.core_siblings.clone()
	}

	pub fn thread_siblings(&self) -> Vec<bool> {
		self.thread_siblings.clone()
	}

	pub fn is_core_sibling(&self, id: usize) -> bool {
		self.core_siblings[id]
	}

	pub fn is_thread_sibling(&self, id: usize) -> bool {
		self.thread_siblings[id]
	}
}

fn get_core_id(node: usize, id: usize) -> Result<usize, &'static str> {
	let path = format!("/sys/devices/system/node/node{}/cpu{}/topology/core_id", node, id);
	usize_from_file(path)
}

fn get_core_siblings(id: usize) -> Result<Vec<bool>, &'static str> {
	let path = format!("/sys/devices/system/cpu/cpu{}/topology/core_siblings", id);
	bitmask_from_hex_file(path)
}

fn get_thread_siblings(id: usize) -> Result<Vec<bool>, &'static str> {
	let path = format!("/sys/devices/system/cpu/cpu{}/topology/thread_siblings", id);
	bitmask_from_hex_file(path)
}

fn get_physical_package_id(id: usize) -> Result<usize, &'static str> {
	let path = format!("/sys/devices/system/cpu/cpu{}/topology/physical_package_id", id);
	usize_from_file(path)
}
