use std::str::Split;

pub fn cut(content: String, field_indices: Vec<usize>, delim: char) -> String {
  let lines = content.split("\n");

  let mut fields: Vec<String> = vec![];
  for line in lines {
    let mut items: Split<char> = line.split(delim);
    
    let mut field_values: Vec<&str> = vec![];

    let mut pos = 0;
    for field_index in &field_indices {
      let elem = items.nth(field_index-pos-1).unwrap_or("-");
      pos = *field_index;
      field_values.push(elem);  
    }
    fields.push(field_values.join(&delim.to_string()));
  }
  let result: String = fields.join("\n");

  return result;
}

#[cfg(test)]
mod cut_tests {
  use crate::cut::cut;

  #[test]
  fn cut_fields() {
    // arrange
    let input = String::from("f1\tf2\n1\t2\n3\t4");
    let fields = vec![2];
    let delim = '\t';

    // act
    let result = cut(input, fields, delim);

    // assert
    assert_eq!(result, "f2\n2\n4")
  }
}