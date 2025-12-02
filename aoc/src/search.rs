/// Computes the Z-array of a given string slice `s`.
///
/// The Z-array `z` is defined such that `z[i]` is the length of the longest substring
/// starting at index `i` that matches the prefix of `s`.
///
/// # Parameters
/// - `s`: A byte slice (`&[u8]`) representing the string to process.
///
/// # Returns
/// A vector `Vec<usize>` where each element `z[i]` represents the length of the
/// longest prefix of `s` that matches the substring starting at position `i`.
///
/// # Example
/// ```
/// let s = b"aaabaaa";
/// let z = z_algorithm(s);
/// assert_eq!(z, vec![0,2,0,1,3,0,0]);
/// ```
///
/// # Explanation
/// The algorithm maintains a window `[l, r]` representing a substring that matches
/// the prefix. For each position `i`:
/// 1. If `i` is inside the window, we use previously computed Z-values to avoid redundant comparisons.
/// 2. Otherwise, we start matching from scratch.
/// 3. We then expand the window `[l, r]` if the match at `i` goes further than the current `r`.
///
/// This implementation runs in O(n) time and O(n) space.
///
/// # Applications
/// - Pattern matching (finding occurrences of a pattern in a string)
/// - Detecting repeated substrings
/// - Computing string periodicity
/// - Solving problems like "is a string composed of repeated blocks?" in linear time
pub fn z_algorithm(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);

    for i in 1..n {
        if i <= r {
            z[i] = (r - i + 1).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }

    z
}
