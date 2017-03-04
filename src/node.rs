use std::io::prelude::*;
use std::fs::File;

use cpu::Cpu;
use util;

#[derive(Clone, Debug)]
pub struct Node {
	id: usize,
	cpus: Vec<Cpu>,
	cpumask: Vec<bool>,
	mem_free: u64,
	mem_total: u64,
	mem_used: u64,
	hugepages_2m_free: usize,
	hugepages_2m_total: usize,
	hugepages_1g_free: usize,
	hugepages_1g_total: usize,
}

impl Node {
	pub fn new(id: usize) -> Result<Node, &'static str> {
		let node = Node {
			id: id,
			cpus: Vec::new(),
			cpumask: Vec::new(),
			mem_free: 0, // TODO: implement
			mem_total: 0, // TODO: implement
			mem_used: 0, // TODO: implement
			hugepages_1g_total: 0,
			hugepages_1g_free: 0,
			hugepages_2m_total: 0,
			hugepages_2m_free: 0,
		};
		node.init()
	}

	fn init(mut self) -> Result<Self, &'static str> {
		if let Ok(cpumask) = util::bitmask_from_hex_file(format!("/sys/devices/system/node/node{}/cpumap", self.id)) {
			self.cpumask = cpumask.clone();
			for i in 0..cpumask.len() {
				if cpumask[i] {
					let id = 2u32.pow(i as u32) as usize - 1;
					if let Ok(mut cpu) = Cpu::new(id) {
						cpu.set_node_id(self.id);
						self.cpus.push(cpu);
					} else {
						break;
					}
				}
			}
			self.mem_init();
			Ok(self)
		} else {
			Err("no node found")
		}
	}

	fn mem_init(&mut self) {
		let path = format!("/sys/devices/system/node/node{}/meminfo", self.id);
		if let Ok(mut f) = File::open(&path) {
			let mut s = String::new();
			if let Ok(_) = f.read_to_string(&mut s) {
				for l in s.lines() {
					let tokens: Vec<&str> = l.split_whitespace().collect();
					match tokens[2] {
						"MemTotal:" => {
							self.mem_total = tokens[3].parse().unwrap_or(0);
						},
						"MemUsed:" => {
							self.mem_used = tokens[3].parse().unwrap_or(0);
						},
						"MemFree:" => {
							self.mem_free = tokens[3].parse().unwrap_or(0);
						},
						_ => {}
					}
				}
			}
		}
		self.hugepages_2m_free = util::usize_from_file(format!("/sys/devices/system/node/node{}/hugepages/hugepages-2048kB/free_hugepages", self.id)).unwrap_or(0);
		self.hugepages_2m_total = util::usize_from_file(format!("/sys/devices/system/node/node{}/hugepages/hugepages-2048kB/nr_hugepages", self.id)).unwrap_or(0);
		self.hugepages_1g_free = util::usize_from_file(format!("/sys/devices/system/node/node{}/hugepages/hugepages-1048576kB/free_hugepages", self.id)).unwrap_or(0);
		self.hugepages_1g_total = util::usize_from_file(format!("/sys/devices/system/node/node{}/hugepages/hugepages-1048576kB/nr_hugepages", self.id)).unwrap_or(0);
	}

	pub fn id(&self) -> usize {
		self.id
	}

	pub fn cpus(&self) -> &Vec<Cpu> {
		&self.cpus
	}

	pub fn cpumask(&self) -> &Vec<bool> {
		&self.cpumask
	}

	pub fn mem_free(&self) -> u64 {
		self.mem_free
	}

	pub fn mem_total(&self) -> u64 {
		self.mem_total
	}

	pub fn mem_used(&self) -> u64 {
		self.mem_used
	}

	pub fn hugepages_2m_free(&self) -> usize {
		self.hugepages_2m_free
	}

	pub fn hugepages_2m_total(&self) -> usize {
		self.hugepages_2m_total
	}

	pub fn hugepages_1g_free(&self) -> usize {
		self.hugepages_1g_free
	}

	pub fn hugepages_1g_total(&self) -> usize {
		self.hugepages_1g_total
	}
}
