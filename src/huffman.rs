
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node<'z> {
    pub symbol: &'z str,
    pub count: u64,
    /// Box reallocates the node from stack to heap.
    /// Otherwise, the compiler will complain that "recursive type `Node` has infinite size"
    pub left: Option<Box<Node<'z>>>,
    pub right: Option<Box<Node<'z>>>,
}

impl<'a> Node<'a> {
    pub fn from_input(mut input: &'a str, tokens: &[&'a str]) -> Node<'a> {
        let mut tokens = tokens
            .into_iter()
            .filter(|&s|s != &"")
            .map(|&s|(s, 0))
            .collect::<Vec<_>>();
        tokens.sort_by_key(|t|t.0.len());
        tokens.reverse();
        
        'outer:
        while input != "" {
            for candidate in &mut tokens {
                if input.starts_with(candidate.0) {
                    candidate.1 += 1;
                    input = &input[candidate.0.len()..];
                    continue 'outer;
                }
            }
            panic!(format!("Unknown prefix: {}", input));
        }

        let mut nodes = tokens
            .into_iter()
            .map(|t| Node {
                symbol: t.0,
                count: t.1,
                left: None,
                right: None
            })
            .collect::<Vec<_>>();
        nodes.sort_by_key(|n|n.count);
        nodes.reverse();
        
        while nodes.len() > 1 {
            let left = nodes.pop().unwrap();
            let right = nodes.pop().unwrap();
            let new = Node {
                symbol: "",
                count: left.count + right.count,
                left: Some(Box::new(left)),
                right: Some(Box::new(right))
            };

            let insert_at = nodes
                .iter()
                .enumerate()
                .rev()
                .find(|&(_, n)|n.count > new.count)
                .map(|t|t.0 + 1)
                .unwrap_or(0);
            nodes.insert(insert_at, new);
        }
        
        nodes.pop().expect("The root node")
    }

    pub fn build_dictionary(&self,
                            result_list: &[(&Node, Vec<i32>)])
                            -> HashMap<String, Vec<i32>> {

        /// Create dictionary based on nodes in the vector.
        /// TODO: create this using map()

        result_list
            .into_iter()
            .map(|&(node, ref codes)|(node.symbol, codes))
            .filter(|t| t.0 != "*" && t.0 != "")
            .map(|(symbol, codes)| (symbol.into(), codes.clone()))
            .collect()
    }

    pub fn generate_codes(&self) -> Vec<(&Node, Vec<i32>)> {

        /// Traverse the tree using a simple queue algorithm.
        /// Store results in pair (Node, turned) where "turned" determines
        /// if we descended left or right.

        let mut stack = Vec::<(&Node, Vec<i32>)>::new();
        let mut result = Vec::<(&Node, Vec<i32>)>::new();

        stack.push((self, vec![]));

        /// Loop until you reach all nodes that don't have left or right children.
        while !stack.is_empty() {

            let (node, codes) = stack.pop().unwrap();
            let copied_node = node.clone();
            let copied_codes = codes.clone();

            /// Push left or right child if node has reference to it.
            /// Add copied vector of codes with 1 or 0 at the end for right/left.

            result.push((copied_node, copied_codes));

            if let Some(ref nod) = node.right {
                let mut new_codes = codes.clone();
                new_codes.push(1);
                stack.push((&nod, new_codes));
            }

            if let Some(ref nod) = node.left {
                let mut new_codes = codes.clone();
                new_codes.push(0);
                stack.push((&nod, new_codes));
            }

        }

        result
    }

    pub fn create_branch(nodes: &mut Vec<Node>) {
        if nodes.len() < 2 {
            return;
        }

        /// Create new branch whit two lowest nodes as (left and right) children.

        /// Sort by probability and pop two lowest.
        nodes.sort_by_key(|a| a.count);
        nodes.reverse();
        
        let (first, second) = (nodes.pop().unwrap(), nodes.pop().unwrap());

        /// Create new node and push it to the vector.
        let new_node = Node {
            symbol: "*",
            count: first.count  + second.count,
            left: Some(Box::new(first)),
            right: Some(Box::new(second)),
        };

        nodes.push(new_node);
    }
}



#[cfg(test)]
mod test {
    use huffman::Node;
    use std::collections::HashMap;

    #[test]
    fn test_build_dictionary_success() {
        let node_a = Node {
            symbol: "A",
            count: 5,
            left: None,
            right: None,
        };
        let node_b = Node {
            symbol: "B",
            count: 5,
            left: None,
            right: None,
        };
        let node_root = Node {
            symbol: "*",
            count: 10,
            left: None,
            right: None,
        };

        let result_list = vec![(&node_root, vec![1]), (&node_a, vec![0]), (&node_b, vec![1])];

        let dictionary = node_root.build_dictionary(&result_list);
        let mut expected_dict = HashMap::new();
        expected_dict.insert("A".to_string(), vec![0]);
        expected_dict.insert("B".to_string(), vec![1]);

        assert_eq!(expected_dict, dictionary);
    }

    #[test]
    fn test_create_branch_success() {
        let mut nodes = vec![Node {
                                 symbol: "A",
                                 count: 1,
                                 left: None,
                                 right: None,
                             },
                             Node {
                                 symbol: "B",
                                 count: 1,
                                 left: None,
                                 right: None,
                             }];

        Node::create_branch(&mut nodes);

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].symbol, "*");
        assert_eq!(nodes[0].count, 2);
        assert_eq!(nodes[0].left.is_none(), false);
        assert_eq!(nodes[0].right.is_none(), false);
    }
}