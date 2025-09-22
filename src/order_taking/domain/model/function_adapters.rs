pub fn predicate_to_passthru<T, F>(error_msg: String, predicate: F, value: T) -> Result<T, String>
where
    F: Fn(&T) -> bool,
{
    if predicate(&value) {
        Ok(value)
    } else {
        Err(error_msg)
    }
}
