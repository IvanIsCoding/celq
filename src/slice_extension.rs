use cel::Value;
use cel::extractors::This;
use std::sync::Arc;

/// Returns a slice of a list, Python-style. We support negative indices and indices
/// that are beyond the end of the list.
pub fn slice(This(this): This<Arc<Vec<Value>>>, start: i64, end: i64) -> Arc<Vec<Value>> {
    let len = this.len() as i64;

    // Normalize negative indices
    let norm_start = if start < 0 {
        (len + start).max(0)
    } else {
        start.min(len)
    } as usize;

    let norm_end = if end < 0 {
        (len + end).max(0)
    } else {
        end.min(len)
    } as usize;

    // Handle case where start >= end
    if norm_start >= norm_end {
        return Arc::new(Vec::new());
    }

    // Extract the slice
    Arc::new(this[norm_start..norm_end].to_vec())
}
