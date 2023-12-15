use std::io::Read;

use serde_json as json;

fn json_string(value: Option<&json::Value>) -> Option<String> {
  value.and_then(|v| v.as_str().map(String::from))
}

fn json_u64(value: Option<&json::Value>) -> Option<u64> {
  value.and_then(|v| v.as_u64())
}

fn main() {
  let mut string = String::new();

  if let Err(err) = std::io::stdin().read_to_string(&mut string) {
    eprintln!("unable to read from stdin: {}", err);
    std::process::exit(1);
  }

  let object: json::Value = match json::from_str(&string) {
    Err(err) => {
      eprintln!("unable to parse json: {}", err);
      std::process::exit(1);
    }
    Ok(ok) => ok,
  };
  let array = match object.as_array() {
    Some(array) => array,
    None => {
      eprintln!("expected array of entries");
      std::process::exit(1);
    }
  };

  for entry in array {
    let file_name = match json_string(entry.get("fileName")) {
      Some(val) => val,
      None => {
        eprintln!("expected fileName attribute");
        std::process::exit(1);
      }
    };
    let line_number = match json_u64(entry.get("lineNumber")) {
      Some(val) => val,
      None => {
        eprintln!("expected lineNumber attribute");
        std::process::exit(1);
      }
    };
    let rule_description = match json_string(entry.get("ruleDescription")) {
      Some(val) => val,
      None => {
        eprintln!("expected ruleDescription attribute");
        std::process::exit(1);
      }
    };
    let rules = match entry.get("ruleNames").and_then(|v| v.as_array()) {
      Some(val) => val,
      None => {
        eprintln!("expected ruleNmaes attribute");
        std::process::exit(1);
      }
    };
    let rules: Vec<_> = rules
      .iter()
      .flat_map(|v| v.as_str())
      .map(|v| v.to_owned())
      .collect();
    let rule = rules.join("/");

    eprintln!(
      "::warning file={},line={},title={}:: {}",
      file_name, line_number, rule, rule_description
    );
  }
}
