#![allow(unused)]

use futures::executor::block_on;
use glam::{Mat4, Vec2};
use graph_craft::document::value::*;
use graph_craft::document::*;
use graph_craft::graphene_compiler::*;
use graph_craft::*;
use graphene_core::{Context, concrete};
use interpreted_executor::dynamic_executor::DynamicExecutor;

fn main() {
	// Create a simple network that adds two numbers
	let network = NodeNetwork {
		exports: vec![NodeInput::node(NodeId(0), 0)],

		nodes: [
			(
			NodeId(0),
			DocumentNode {
				inputs: vec![NodeInput::value(TaggedValue::U32(5), false), NodeInput::value(TaggedValue::U32(42), false)],
				implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("graphene_core::ops::AddNode")),
				manual_composition: Some(concrete!(Context)),
				..Default::default()
			},
		)
		/*(
                NodeId(0),
                DocumentNode {
                    inputs: vec![NodeInput::network(concrete!(u32), 0)],
                    implementation: DocumentNodeImplementation::ProtoNode(ProtoNodeIdentifier::new("graphene_core::ops::IdentityNode")),
                    //manual_composition: Some(concrete!(Context)),
					..Default::default()
                },
            )**/
		]
		/* 
		nodes: [
			(
				NodeId(0),
				DocumentNode {
					inputs: vec![NodeInput::network(concrete!(u32), 0), NodeInput::value(TaggedValue::U32(42), false)],
					implementation: DocumentNodeImplementation::ProtoNode("graphene_core::structural::ConsNode".into()),
					manual_composition: Some(concrete!(Context)),
					..Default::default()
				},
			),
			(
				NodeId(1),
				DocumentNode {
					inputs: vec![NodeInput::node(NodeId(0), 0)],
					implementation: DocumentNodeImplementation::ProtoNode("graphene_core::ops::AddPairNode".into()),
					..Default::default()
				},
			),
		]*/
		.into_iter()
		.collect(),
		..Default::default()
	};

	// Compile the network
	let compiler = Compiler {};
	let proto_network = compiler.compile_single(network).expect("Failed to compile network");

	// Create the executor
	let executor = block_on(DynamicExecutor::new(proto_network)).expect("Failed to create executor");

	let context = Context::default();

	let result = block_on((&executor).execute(context)).expect("Failed to execute network");
	println!("5 + 42 = {}", result);

	// let exec = block_on(DynamicExecutor::new(proto_network));
	// if let Ok(executor) = exec {
	// 	// Execute the network with input 5
	// 	let result = block_on((&executor).execute(None::<()>)).expect("Failed to execute network");
	// 	println!("5 + 42 = {}", result);
	// } else {
	// 	println!("Failed to create executor");
	// }
}
