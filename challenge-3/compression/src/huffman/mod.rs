use std::collections::{BinaryHeap, HashMap};
use base64::{engine::general_purpose, Engine as _};


#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: Option<char>,
    frequency: u32,
}
impl Node {
    fn new_node(left: Option<Node>, right: Option<Node>) -> Option<Node> {
        match (left, right) {
            (Some(left), Some(right)) => {
                let frequency = left.frequency + right.frequency;
                Some(Node {
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                    value: None,
                    frequency,
                })
            }
            (Some(left), None) => {
                let frequency = left.frequency;
                Some(Node {
                    left: Some(Box::new(left)),
                    right: None,
                    value: None,
                    frequency,
                })
            }
            (None, Some(right)) => {
                let frequency = right.frequency;
                Some(Node {
                    left: None,
                    right: Some(Box::new(right)),
                    value: None,
                    frequency,
                })
            }
            _ => None,
        }

        //   let frequency = match (left, right) {
        //       (Some(left), Some(right)) => left.frequency + right.frequency,
        //       (Some(left), None) => left.frequency,
        //       (None, Some(right)) => right.frequency,
        //       _ => 0,
        //   };

        //   let left_node = match left {
        //       Some(left) => Some(Box::new(left)),
        //       None => None,
        //   };

        //   let right_node = match right {
        //       Some(right) => Some(Box::new(right)),
        //       None => None,
        //   };

        //   Node {
        //     left:  left_node,
        //     right: right_node,
        //     value: None,
        //     frequency,
        //   }
    }

    fn new_leaf(value: char, frequency: u32) -> Node {
        Node {
            left: None,
            right: None,
            value: Some(value),
            frequency: frequency,
        }
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.frequency.partial_cmp(&self.frequency)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn compress(str: String) -> String {
    let frequency_table = get_frequency_table(str.clone());
    let tree = build_tree(frequency_table).unwrap();
    let encoding_table = build_encoding_table(&tree);
    let encoded = encode(str, &encoding_table);

    let encoding_table_out = encoding_table
      .iter()
      .map(|(key, value)|
        match key {
          ' ' => format!(":space {}", value),
          '\n' => format!(":newline {}", value),
          '\r' => format!(":return {}", value),
          '\t' => format!(":tab {}", value),
          _ => format!("{} {}", key, value)
        }
      ).collect::<Vec<String>>().join("\n");
    let result = format!("{}\n{}\n{}", encoding_table.len(), encoding_table_out, encoded);
    return result;
}

pub fn decompress(str: String) -> String {

  let mut lines = str.lines();
  let count = lines.next().unwrap().to_string().parse::<i32>().unwrap();
  
  let mut decoding_table: HashMap<String, char> = HashMap::new();
  for _ in 0..count {
    let line = lines.next().unwrap().to_string();
    let mut parts = line.split_whitespace();

    let token = parts.next().unwrap().to_string();
    let code: String = parts.next().unwrap().to_string();

    let char = match token.as_str() {
      ":space" => ' ',
      ":newline" => '\n',
      ":return" => '\r',
      ":tab" => '\t',
      _ => token.parse::<char>().unwrap()
    };

    decoding_table.insert(code, char);
  }

  let encoded = lines.next().unwrap().to_string();
  let bytes = general_purpose::STANDARD.decode(&encoded).unwrap();
  let mut output: Vec<char> = vec![];
  let mut pattern: String = String::new();
  for i in 0..bytes.len() {
    let byte = bytes[i];
    
    for j in 0..8 {
      let bit = byte & (1 << 7-j) != 0;
      let next = match bit {
        false => '0',
        true => '1'
      };
      pattern.push(next);
      if decoding_table.contains_key(&pattern) {
        let next_char = decoding_table.get(&pattern).unwrap().clone();
        output.push(next_char);
        pattern.clear();
      }
    }
  }

  return output.into_iter().collect();
}

fn get_frequency_table(str: String) -> HashMap<char, u32> {
    let mut frequency_table: HashMap<char, u32> = HashMap::new();

    for c in str.chars() {
        let count = frequency_table.entry(c).or_insert(0);
        *count += 1;
    }

    return frequency_table;
}

fn frequency_table_to_heap(frequency_table: HashMap<char, u32>) -> BinaryHeap<Node> {
    let nodes: Vec<Node> = frequency_table
        .into_iter()
        .map(|(value, frequency)| Node::new_leaf(value, frequency))
        .collect();
    let heap: BinaryHeap<Node> = BinaryHeap::from(nodes);
    return heap;
}

fn build_tree(frequencies: HashMap<char, u32>) -> Result<Node, String> {
    if frequencies.len() == 0 {
        return Err("Empty frequency table".to_string());
    }

    if frequencies.len() == 1 {
        let (value, frequency) = frequencies.into_iter().next().unwrap();
        let node = Node::new_leaf(value, frequency);
        return Ok(node);
    }

    let mut heap = frequency_table_to_heap(frequencies);

    while heap.len() > 1 {
        let left = heap.pop();
        let right = heap.pop();
        let head = Node::new_node(left, right);
        heap.push(head.unwrap());
    }
    return heap.pop().ok_or("".to_string());
}

fn build_encoding_table(root: &Node) -> HashMap<char, String> {
  let mut encoding_table: HashMap<char, String> = HashMap::new();
  let mut stack: Vec<(char, String, &Node)> = Vec::new();
  stack.push((' ', String::new(), root));

  while let Some((value, code, node)) = stack.pop() {
    match node.value {
      Some(value) => {
        encoding_table.insert(value, code);
      }
      None => {
        let left = node.left.as_ref().unwrap();
        let right = node.right.as_ref().unwrap();
        stack.push((value, code.clone() + "0", left));
        stack.push((value, code + "1", right));
      }
    }
  }
  return encoding_table;
}

fn encode(str: String, encoding_table: &HashMap<char, String>) -> String {
  let mut packed: Vec<u8> = Vec::new();
  let mut current_byte = 0;
  let mut bit_cursor = 0;

  for c in str.chars() {
    let code = encoding_table.get(&c).unwrap();
    for bit in code.chars() {
      if bit_cursor == 8 {
        packed.push(current_byte);
        current_byte = 0;
        bit_cursor = 0;
      }
      current_byte = current_byte << 1;
      if bit == '1' {
        current_byte += 1;
      }
      bit_cursor += 1;
    }
  }

  general_purpose::STANDARD.encode(&packed)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_frequency_table_hello_world() {
        // Arrange
        let str = String::from("Hello, world!");

        //Act
        let frequency_table = get_frequency_table(str);

        // Assert
        assert_eq!(frequency_table.get(&'H'), Some(&1));
        assert_eq!(frequency_table.get(&'e'), Some(&1));
        assert_eq!(frequency_table.get(&'l'), Some(&3));
        assert_eq!(frequency_table.get(&'o'), Some(&2));
        assert_eq!(frequency_table.get(&','), Some(&1));
        assert_eq!(frequency_table.get(&' '), Some(&1));
        assert_eq!(frequency_table.get(&'w'), Some(&1));
        assert_eq!(frequency_table.get(&'r'), Some(&1));
        assert_eq!(frequency_table.get(&'d'), Some(&1));
        assert_eq!(frequency_table.get(&'!'), Some(&1));
    }

    #[test]
    fn test_get_frequency_table_empty_string() {
        // Arrange
        let str = String::from("");

        //Act
        let frequency_table = get_frequency_table(str);

        // Assert
        assert_eq!(frequency_table.len(), 0);
    }

    #[test]
    fn test_build_tree_opendsa_example() {
        //Arrange
        let mut frequency_table: HashMap<char, u32> = HashMap::new();
        frequency_table.insert('c', 32);
        frequency_table.insert('d', 42);
        frequency_table.insert('e', 120);
        frequency_table.insert('k', 7);
        frequency_table.insert('l', 43);
        frequency_table.insert('m', 24);
        frequency_table.insert('u', 37);
        frequency_table.insert('z', 2);

        //Act
        let result = build_tree(frequency_table);

        //Assert
        assert!(result.is_ok());

        macro_rules! assert_node {
            ($node:expr, $frequency:expr) => {{
                assert_eq!($node.frequency, $frequency);
                assert!($node.value.is_none());
                assert!($node.left.is_some());
                assert!($node.right.is_some());

                ($node.left.unwrap(), $node.right.unwrap())
            }};
        }

        macro_rules! assert_node_left_leaf {
            ($node:expr, $frequency:expr, $left_frequency:expr, $left_value:expr) => {{
                assert_eq!($node.frequency, $frequency);
                assert!($node.value.is_none());
                assert!($node.left.is_some());
                assert!($node.right.is_some());

                let left = $node.left.unwrap();
                assert_eq!(left.frequency, $left_frequency);
                assert_eq!(left.value, Some($left_value));

                $node.right.unwrap()
            }};
        }

        macro_rules! assert_node_right_leaf {
            ($node:expr, $frequency:expr, $right_frequency:expr, $right_value:expr) => {{
                assert_eq!($node.frequency, $frequency);
                assert!($node.value.is_none());
                assert!($node.left.is_some());
                assert!($node.right.is_some());

                let right = $node.right.unwrap();
                assert_eq!(right.frequency, $right_frequency);
                assert_eq!(right.value, Some($right_value));

                $node.left.unwrap()
            }};
        }

        macro_rules! assert_node_terminal {
            ($node:expr, $frequency:expr, $left_frequency:expr, $left_value:expr, $right_frequency:expr, $right_value:expr) => {{
                assert_eq!($node.frequency, $frequency);
                assert!($node.value.is_none());
                assert!($node.left.is_some());
                assert!($node.right.is_some());

                let left = $node.left.unwrap();
                assert_eq!(left.frequency, $left_frequency);
                assert_eq!(left.value, Some($left_value));

                let right = $node.right.unwrap();
                assert_eq!(right.frequency, $right_frequency);
                assert_eq!(right.value, Some($right_value));
            }};
        }

        /* Expected tree:

           306
          /   \
        E120  186
             /   \
          79      107
         /  \    /   \
       U37  D42 L42  65
                    /  \
                  C32  33
                      /  \
                     9   M24
                   /  \
                  2Z  7K
        */

        let root = result.unwrap();
        let level1_node_1 = assert_node_left_leaf!(root, 307, 120, 'e');
        let (level2_node1, level2_node2) = assert_node!(level1_node_1, 187);
        assert_node_terminal!(level2_node1, 79, 37, 'u', 42, 'd');
        let level3_node1 = assert_node_left_leaf!(level2_node2, 108, 43, 'l');
        let level4_node1 = assert_node_left_leaf!(level3_node1, 65, 32, 'c');
        let level5_node1 = assert_node_right_leaf!(level4_node1, 33, 24, 'm');
        assert_node_terminal!(level5_node1, 9, 2, 'z', 7, 'k');
    }
    
    #[test]
    fn test_build_encoding_table_opendsa_example() {
        //Arrange
        let mut frequency_table: HashMap<char, u32> = HashMap::new();
        frequency_table.insert('c', 32);
        frequency_table.insert('d', 42);
        frequency_table.insert('e', 120);
        frequency_table.insert('k', 7);
        frequency_table.insert('l', 43);
        frequency_table.insert('m', 24);
        frequency_table.insert('u', 37);
        frequency_table.insert('z', 2);

        //Act
        let tree = build_tree(frequency_table).unwrap();
        let table = build_encoding_table(&tree);

        //Assert
        assert_eq!(table[&'c'], "1110");
        assert_eq!(table[&'d'], "101");
        assert_eq!(table[&'e'], "0");
        assert_eq!(table[&'k'], "111101");
        assert_eq!(table[&'l'], "110");
        assert_eq!(table[&'m'], "11111");
        assert_eq!(table[&'u'], "100");
        assert_eq!(table[&'z'], "111100");
    }
    
    #[test]
    fn test_build_tree_aaabb() {
        //Arrange
        let mut frequency_table: HashMap<char, u32> = HashMap::new();
        frequency_table.insert('a', 3);
        frequency_table.insert('b', 2);

        //Act
        let result = build_tree(frequency_table);

        //Assert
        assert!(result.is_ok());
        let tree = result.unwrap();
        let left = tree.left.unwrap();
        let right = tree.right.unwrap();
        assert_eq!(tree.frequency, 5);
        assert_eq!(tree.value, None);
        assert_eq!(left.frequency, 2);
        assert_eq!(left.value, Some('b'));
        assert_eq!(right.frequency, 3);
        assert_eq!(right.value, Some('a'));
    }
}
