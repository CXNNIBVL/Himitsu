
/// Safe comparison of Byte buffers
/// * 'a' - First Buffer
/// * 'b' - Second Buffer
#[inline(never)]
pub fn memeq_s(a: &[u8], b: &[u8]) -> bool {

    if a.len() != b.len() { return false; }

    a.iter().zip(b.iter())
    .map(|(x, y)| x ^ y)
    .fold(0, |sum, nx | sum | nx)
    .eq(&0)
}