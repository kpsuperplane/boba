use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate serde_json;
extern crate regex;

use regex::Regex;

struct Highlighter {
  last_highlighted: usize,
  patterns: Vec<serde_json::Value>
}

impl Highlighter {
  fn setGrammar(&self, grammar: &serde_json::Value) {
    self.patterns = Vec::new();
    for pattern in grammar["patterns"].as_array().unwrap() {
      if pattern["match"] != serde_json::Value::Null {
        let patternObj = pattern.as_object().unwrap();
        pattern["re"] = json!(Regex::new(pattern["match"].as_str().unwrap()).unwrap());
        &self.patterns.push(pattern);
      }
    }
  }
  fn highlight(&self, mut start: usize, lines: &Vec<&str>) {
    for l in start .. lines.len() {
      let mut line = lines[l].to_string();
      while line != "" {
        for pattern in &self.grammar["patterns"] {
          let borrow = &line.clone();
          if pattern["re"].is_match(borrow) {
            let matched = pattern["re"].find(borrow).unwrap();
            line = (&line[matched.end()..]).to_string();
            println!("{}: {}", pattern["name"], matched.as_str());
            continue;
          }
        }
        break;
        line = line[1..].to_string();
      }
    }
  }
}
fn main() {

  let mut file = File::open("src/grammar/c.json").expect("Unable to open grammar");
  let mut contents = String::new();
  file.read_to_string(&mut contents);
  let grammar: serde_json::Value = serde_json::from_str(&contents).expect("Unable to parse grammar");

  let data = "#include <stdio.h>
int main()
{
    int i, n, t1 = 0, t2 = 1, nextTerm;

    printf(\"Enter the number of terms: \");
    scanf(\"%d\", &n);

    printf(\"Fibonacci Series: \");

    for (i = 1; i <= n; ++i)
    {
        printf(\"%d, \", t1);
        nextTerm = t1 + t2;
        t1 = t2;
        t2 = nextTerm;
    }
    return 0;
}";
  let lines: Vec<&str> = data.split('\n').collect();
  let highlighter = Highlighter {last_highlighted: 0};
  highlighter.setGrammar(&grammar);
  highlighter.highlight(0, &lines);
}