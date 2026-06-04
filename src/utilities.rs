
/// Returns an iterator following the geometric series,
/// with <em>a</em> being the initial value and <em>r</em> being the common ratio
pub(crate) fn geometric_iter(a: usize, r: usize) -> impl Iterator<Item=usize> {
    (0..).map(move |i| a * r.pow(i))
}

/// Returns an iterator following the arithmetic series,
/// with <em>a</em> being the initial value and <em>r</em> being the common difference
pub(crate) fn arithmetic_iter(a: usize, d: usize) -> impl Iterator<Item=usize> {
    (0..).map(move |i| a + d * i)
}