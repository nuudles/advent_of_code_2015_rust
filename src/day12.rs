use serde_json::Value;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Value {
    serde_json::from_str(input).expect("Invalid JSON")
}

fn value_sum(value: &Value, ignore_red: bool) -> i64 {
    match value {
        Value::Array(array) => array.iter().map(|v| value_sum(v, ignore_red)).sum(),
        Value::Object(object) => {
            if !ignore_red || !object.values().any(|v| 
                if let Value::String(string) = v { string == "red" } else { false }
            ) {
                object
                    .values()
                    .map(|v| value_sum(v, ignore_red))
                    .sum()
            } else { 
                0 
            }
        },
        Value::Number(number) => number.as_i64().unwrap_or(0),
        _ => 0
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &Value) -> i64 {
    value_sum(input, false)
}

#[aoc(day12, part2)]
pub fn part2(input: &Value) -> i64 {
    value_sum(input, true)
}
