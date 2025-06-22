use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Tree {
    pub nodes: HashMap<String, Node>,
    pub tree_attributes: HashMap<String, String>,
}

pub fn parse_tree(input: &str) -> Result<Tree, String> {
    let mut in_tree = false;
    let mut tree_attributes = HashMap::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut parent_map: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with(":start-tree") {
            // Handle :start-tree[...]:
            if let Some(attr_start) = line.find('[') {
                let attr_end = line.find(']').ok_or("Missing closing ] on :start-tree:")?;
                let attrs = &line[(attr_start + 1)..attr_end];
                for attr in attrs.split(',') {
                    let parts: Vec<_> = attr.splitn(2, '=').collect();
                    if parts.len() != 2 {
                        return Err(format!("Invalid tree attribute: {}", attr));
                    }
                    tree_attributes
                        .insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
            in_tree = true;
            continue;
        }

        if line == ":end-tree:" {
            break;
        }

        if !in_tree {
            continue;
        }

        for token in line.split(';') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }

            let mut name = String::new();
            let mut parent = None;
            let mut attributes = HashMap::new();

            // Extract [attributes]
            let (token, attr_part) = if let Some(attr_start) = token.find('[') {
                let attr_end = token.find(']').ok_or("Missing closing ]")?;
                (
                    &token[..attr_start],
                    Some(&token[(attr_start + 1)..attr_end]),
                )
            } else {
                (token, None)
            };

            // Parse attributes
            if let Some(attr_str) = attr_part {
                for attr in attr_str.split(',') {
                    let parts: Vec<_> = attr.splitn(2, '=').collect();
                    if parts.len() != 2 {
                        return Err(format!("Invalid node attribute format: {}", attr));
                    }
                    attributes.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }

            // Parse name and optional parent
            if let Some(start_idx) = token.find('(') {
                let end_idx = token.find(')').ok_or("Missing closing )")?;
                name = token[..start_idx].to_string();
                parent = Some(token[(start_idx + 1)..end_idx].to_string());
            } else {
                name = token.to_string();
            }

            nodes.entry(name.clone()).or_insert(Node {
                name: name.clone(),
                children: vec![],
                attributes,
            });

            if let Some(p) = parent {
                parent_map.insert(name.clone(), p);
            }
        }
    }

    // Attach children
    for (child, parent) in parent_map {
        if let Some(parent_node) = nodes.get_mut(&parent) {
            parent_node.children.push(child);
        } else {
            return Err(format!("Parent '{}' not found", parent));
        }
    }

    Ok(Tree {
        nodes,
        tree_attributes,
    })
}
