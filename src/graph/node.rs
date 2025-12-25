use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use crate::graph::nodes::{add::AddNode, complex::ComplexNode, number::NumberNode};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[enum_dispatch]
pub enum PergaminoNode {
	Number(NumberNode),
	Add(AddNode),
	Complex(ComplexNode),
}

// impl PergaminoNodeBehavior for PergaminoNode {
// 	fn title(&self) -> String {
// 		match self {
//         	PergaminoNode::Number(n) => n.title(),
//         	PergaminoNode::Add(n) => n.title(),
//         	PergaminoNode::Complex(n) => n.title(),
//         }
// 	}

// 	// ...
//	// Irrelevant: enum_dispatch takes care of it
// }

