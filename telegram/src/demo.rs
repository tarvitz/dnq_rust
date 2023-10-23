use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Container<'a> {
	id: &'a str,
	contents: Vec<&'a str>
}

#[cfg(test)]
mod unit_tests {
	use std::collections::HashMap;
	use super::*;

	#[test]
	fn test_internal_object(){
		let container = Container{id: "test", contents: vec!["this", "is", "test"]};
		println!("container: {:?}", container);
	}

	#[test]
	fn test_copy(){

		fn copy<'a>(mut to: HashMap<&'a str, Vec<Container<'a>>>, sources: Vec<Container<'a>>) -> HashMap<&'a str, Vec<Container<'a>>>{
			for container in sources {
				if !to.contains_key(container.id) {
					to.insert(container.id, Vec::new());
				}
				to.get_mut(container.id).unwrap().push(container);
			}

			to
		}

		let map : HashMap<&str, Vec<Container>> = HashMap::new();
		let sources = vec![
			Container{id: "test", contents:vec!["this", "is", "a", "test"]},
			Container{id: "me", contents:vec!["me", "mine"]},
		];
		let map = copy(map, sources);
		assert_eq!(2, map.len());
	}

	#[test]
	fn from_string(){
		let raw = String::from(r#"
id: "this is the test"
contents: ["test", "me"]
"#);
		let obj:Container = serde_yaml::from_str(raw.as_str()).unwrap();
		println!("This is container: {:?}", obj);
	}
}