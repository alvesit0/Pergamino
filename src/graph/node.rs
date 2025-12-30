use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};

use crate::graph::nodes::{add::AddNode, complex::ComplexNode, number::NumberNode};

// #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
// #[enum_dispatch]
// pub enum PergaminoNode {
// 	Number(NumberNode),
// 	Add(AddNode),
// 	Complex(ComplexNode),
// }

macro_rules! define_node_enum {
	(
		$(#[$meta:meta])*
		pub enum $enum_name:ident {
			$($variant:ident($inner:ty)),* $(,)?
		}
	) => {
		$(#[$meta])*
		#[enum_dispatch(PergaminoNodeBehavior)]
		#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
		pub enum $enum_name {
			$(
				$variant($inner),
			)*
		}

		impl $enum_name {
			pub fn prototypes() -> Vec<Self> {
				vec![
					$(
						$enum_name::$variant(<$inner>::default()),
					)*
				]
			}
		}
	};
}

define_node_enum! {
	pub enum PergaminoNode {
		Number(NumberNode),
		Add(AddNode),
		Complex(ComplexNode),
		// ADD NEW NODES HERE
	}
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

