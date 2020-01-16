pub fn csv_to_vec(input: String) -> Vec<String> {
    input.split(",").map(str::trim).map(str::to_string).collect()
}