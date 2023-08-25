
fn unique(values: Vec<i32>) -> Vec<i32> {
    let mut values = values;
    values.sort_unstable();
    values.dedup();

    values
}