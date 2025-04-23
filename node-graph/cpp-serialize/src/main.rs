#![allow(unused)]

use futures::executor::block_on;
use glam::{Mat4, Vec2};
use graph_craft::{document::*, graphene_compiler::Compiler};
use graphene_core::{Context, concrete};
use interpreted_executor::dynamic_executor::DynamicExecutor;

#[derive(Debug, Clone, Copy, dyn_any::DynAny)]
pub struct RltRectangle {
	mat: Mat4,
	width: f32,
	height: f32,
	material: u32,
	uv_min: Vec2,
	uv_max: Vec2,
}

#[derive(Debug, Clone, Copy, dyn_any::DynAny)]
pub struct RltCuboid {
	mat: Mat4,
	width: f32,
	height: f32,
	depth: f32,
	material: u32,
	uv_min: Vec2,
	uv_max: Vec2,
}

#[derive(Debug, Clone, Copy, dyn_any::DynAny)]
pub struct Components(u32);

impl Components {
	const NONE: u32 = 0;
	const FRONT: u32 = 1 << 1;
	const BACK: u32 = 1 << 2;
	const LEFT: u32 = 1 << 3;
	const RIGHT: u32 = 1 << 4;
	const TOP: u32 = 1 << 5;
	const BOTTOM: u32 = 1 << 6;

	fn count_components(&self) -> u32 {
		self.0.count_ones()
	}

	fn side() -> Self {
		Self(Self::FRONT | Self::BACK | Self::LEFT | Self::RIGHT)
	}

	fn contains(&self, component: u32) -> bool {
		self.0 & component != 0
	}
}

impl std::ops::BitOr for Components {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

#[node_macro::node(category("Renderlet"))]
fn extrude_rectangle<'a: 'n>(_: Context<'static>, rects: &'a [RltRectangle], depth: f32) -> Vec<RltCuboid> {
	rects
		.iter()
		.map(|rect| RltCuboid {
			mat: rect.mat,
			width: rect.width,
			height: rect.height,
			depth: depth,
			material: rect.material,
			uv_min: rect.uv_min,
			uv_max: rect.uv_max,
		})
		.collect()
}

#[node_macro::node(category("Renderlet"))]
fn comp_cuboid<'a: 'n>(_: Context<'static>, cuboids: &'a [RltCuboid], components: Components) -> Vec<RltRectangle> {
	let mut output = Vec::with_capacity(components.count_components() as usize * cuboids.len());

	for cuboid in cuboids {
		if components.contains(Components::FRONT) {
			output.push(RltRectangle {
				mat: cuboid.mat,
				width: cuboid.width,
				height: cuboid.height,
				material: cuboid.material,
				uv_min: cuboid.uv_min,
				uv_max: cuboid.uv_max,
			});
		}
		if components.contains(Components::BACK) {
			output.push(RltRectangle {
				mat: cuboid.mat,
				width: cuboid.width,
				height: cuboid.height,
				material: cuboid.material,
				uv_min: cuboid.uv_min,
				uv_max: cuboid.uv_max,
			});
		}
		if components.contains(Components::LEFT) {
			output.push(RltRectangle {
				mat: cuboid.mat,
				width: cuboid.width,
				height: cuboid.height,
				material: cuboid.material,
				uv_min: cuboid.uv_min,
				uv_max: cuboid.uv_max,
			});
		}
		if components.contains(Components::RIGHT) {
			output.push(RltRectangle {
				mat: cuboid.mat,
				width: cuboid.width,
				height: cuboid.height,
				material: cuboid.material,
				uv_min: cuboid.uv_min,
				uv_max: cuboid.uv_max,
			});
		}
		if components.contains(Components::TOP) {
			output.push(RltRectangle {
				mat: cuboid.mat,
				width: cuboid.width,
				height: cuboid.height,
				material: cuboid.material,
				uv_min: cuboid.uv_min,
				uv_max: cuboid.uv_max,
			});
		}
		if components.contains(Components::BOTTOM) {
			output.push(RltRectangle {
				mat: cuboid.mat,
				width: cuboid.width,
				height: cuboid.height,
				material: cuboid.material,
				uv_min: cuboid.uv_min,
				uv_max: cuboid.uv_max,
			});
		}
	}
	output
}

fn main() {
	// Create a simple network that adds two numbers
	let network = NodeNetwork {
		exports: vec![NodeInput::node(NodeId(1), 0)],
		nodes: [
			// Input node that takes a number
			(
				NodeId(0),
				DocumentNode {
					inputs: vec![NodeInput::network(concrete!(u32), 0)],
					implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("graphene_core::ops::IdentityNode")),
					..Default::default()
				},
			),
			// Add node that adds the input with a constant value
			(
				NodeId(1),
				DocumentNode {
					inputs: vec![NodeInput::node(NodeId(0), 0), NodeInput::value(TaggedValue::U32(42), false)],
					implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("graphene_core::ops::AddNode")),
					..Default::default()
				},
			),
		]
		.into_iter()
		.collect(),
		..Default::default()
	};

	// Compile the network
	let compiler = Compiler {};
	let proto_network = compiler.compile_single(network).expect("Failed to compile network");

	// Create the executor
	let executor = block_on(DynamicExecutor::new(proto_network)).expect("Failed to create executor");

	// Execute the network with input 5
	let result = block_on(executor.execute(5u32)).expect("Failed to execute network");
	println!("5 + 42 = {}", result);
}

fn main2() {
	let start_time = std::time::Instant::now();

	let rectangles = vec![RltRectangle {
		mat: Mat4::IDENTITY,
		width: 1.0,
		height: 1.0,
		material: 0,
		uv_min: Vec2::ZERO,
		uv_max: Vec2::ZERO,
	}];
	let components = Components::side();

	for i in 0..4000 {
		let cuboids = extrude_rectangle(None, &rectangles, 1.0);
		let rectangles = comp_cuboid(None, &cuboids, components);

		let cuboids = extrude_rectangle(None, &rectangles, 1.0);
		let rectangles = comp_cuboid(None, &cuboids, components);

		let cuboids = extrude_rectangle(None, &rectangles, 1.0);
		let rectangles = comp_cuboid(None, &cuboids, components);

		let cuboids = extrude_rectangle(None, &rectangles, 1.0);
		let rectangles = comp_cuboid(None, &cuboids, components);

		let cuboids = extrude_rectangle(None, &rectangles, 1.0);
		let rectangles = comp_cuboid(None, &cuboids, components);

		let cuboids = extrude_rectangle(None, &rectangles, 1.0);
		let rectangles = comp_cuboid(None, &cuboids, components);

		let cuboids = extrude_rectangle(None, &rectangles, 1.0);
		let rectangles = comp_cuboid(None, &cuboids, components);
	}

	let duration = start_time.elapsed();
	println!("Execution time: {:?}", duration);
}
