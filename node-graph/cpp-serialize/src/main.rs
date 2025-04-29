#![allow(unused)]

use futures::executor::block_on;
use glam::{Mat4, Vec2};
use graph_craft::document::value::*;
use graph_craft::document::*;
use graph_craft::graphene_compiler::*;
use graph_craft::*;
use graphene_core::OwnedContextImpl;
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
	pub depth: f32,
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
fn create_rectangle<'a: 'n>(_: Context<'static>) -> Vec<RltRectangle> {
	vec![RltRectangle {
		mat: Mat4::IDENTITY,
		width: 1.0,
		height: 1.0,
		material: 0,
		uv_min: Vec2::ZERO,
		uv_max: Vec2::ZERO,
	}]
}

#[node_macro::node(category("Renderlet"))]
fn extrude_rectangle<'a: 'n>(_: Context<'static>, rects: Vec<RltRectangle>, depth: f64) -> RltCuboid {
	rects
		.iter()
		.map(|rect| RltCuboid {
			mat: rect.mat,
			width: rect.width,
			height: rect.height,
			depth: depth as f32,
			material: rect.material,
			uv_min: rect.uv_min,
			uv_max: rect.uv_max,
		})
		.next()
		.unwrap()
}

#[node_macro::node(category("Renderlet"))]
fn cuboid_get_depth<'a: 'n>(_: Context<'static>, cuboid: RltCuboid) -> f64 {
	cuboid.depth as f64
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

#[node_macro::node]
async fn empty_context<T>(_: (), #[implementations(Context->f64)] input: impl Node<Context<'static>, Output = T>) -> T {
	input.eval(None).await
}

fn main() {
	// Create a simple network that adds two numbers
	// let network = NodeNetwork {
	// 	exports: vec![NodeInput::node(NodeId(2), 0)],

	// 	nodes: [
	// 	(
	// 		NodeId(0),
	// 		DocumentNode {
	// 			inputs: vec![NodeInput::value(TaggedValue::U32(5), false), NodeInput::value(TaggedValue::U32(42), false)],
	// 			implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("graphene_core::ops::AddNode")),
	// 			manual_composition: Some(concrete!(Context)),
	// 			..Default::default()
	// 		},
	// 	),
	// 	(
	// 		NodeId(1),
	// 		DocumentNode {
	// 			inputs: vec![NodeInput::node(NodeId(0), 0), NodeInput::value(TaggedValue::U32(1), false)],
	// 			implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("graphene_core::ops::AddNode")),
	// 			manual_composition: Some(concrete!(Context)),
	// 			..Default::default()
	// 		},
	// 	),
	// 	(
	// 		NodeId(2),
	// 		DocumentNode {
	// 			inputs: vec![ NodeInput::node(NodeId(1), 0)],
	// 			implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("graphene_core::context::EmptyContextNode")),
	// 			manual_composition: Some(concrete!(())),
	// 			..Default::default()
	// 		},
	// 	)
	// 	]
	// 	.into_iter()
	// 	.collect(),
	// 	..Default::default()
	// };

	let network = NodeNetwork {
		exports: vec![NodeInput::node(NodeId(3), 0)],
		nodes: [
			(
				NodeId(0),
				DocumentNode {
					inputs: vec![],
					implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("cpp_serialize::CreateRectangleNode")),
					manual_composition: Some(concrete!(Context)),
					..Default::default()
				},
			),
			(
				NodeId(1),
				DocumentNode {
					inputs: vec![NodeInput::node(NodeId(0), 0), NodeInput::value(TaggedValue::F64(1.5), false)],
					implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("cpp_serialize::ExtrudeRectangleNode")),
					manual_composition: Some(concrete!(Context)),
					..Default::default()
				},
			),
			(
				NodeId(2),
				DocumentNode {
					inputs: vec![NodeInput::node(NodeId(1), 0)],
					implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("cpp_serialize::CuboidGetDepthNode")),
					manual_composition: Some(concrete!(Context)),
					..Default::default()
				},
			),
			(
				NodeId(3),
				DocumentNode {
					inputs: vec![NodeInput::node(NodeId(2), 0)],
					implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("cpp_serialize::EmptyContextNode")),
					manual_composition: Some(concrete!(())),
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

	let result = block_on((&executor).execute(())).expect("Failed to execute network");
	println!("Extrude depth = {}", result);

	// let exec = block_on(DynamicExecutor::new(proto_network));
	// if let Ok(executor) = exec {
	// 	// Execute the network with input 5
	// 	let result = block_on((&executor).execute(None::<()>)).expect("Failed to execute network");
	// 	println!("5 + 42 = {}", result);
	// } else {
	// 	println!("Failed to create executor");
	// }
}
