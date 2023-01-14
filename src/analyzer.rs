pub fn analyze_string(string: &str) -> Vec<String> {
    // String format example: name = value;

    let p: Vec<&str> = string.split(" = ").collect();
    let mut t = "{unknow}";

    if p[1].starts_with("\"") && p[1].ends_with("\"") {
        t = "string"
    } else if p[1] == "true" || p[1] == "false" {
        t = "boolean"
    } else {
        t = "number"
    }

    return vec![p[0].to_string(), p[1].to_string(), t.to_string()];
}

pub fn convert_to_json(string: &str) -> String {
    let mut json = String::from("{");

    let lines: Vec<&str> = string.lines().collect();

    for line in lines {
        let result = analyze_string(line);

        json.push_str(format!("\"{}\": {},", result[0], result[1]).as_str());
    }

    json.remove(json.len() - 1);
    json.push_str("}");

    return json;
}

pub fn convert_to_yaml(string: &str) -> String {
    let mut yaml = String::new();
    
    let lines: Vec<&str> = string.lines().collect();

    for line in lines {
        let result = analyze_string(line);

        yaml.push_str(format!("{}: {}\n", result[0], result[1]).as_str());
    }

    return yaml;
}