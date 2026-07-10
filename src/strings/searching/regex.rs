use std::{error::Error, fmt::Display};

use crate::{
    collections::{Bag, Stack},
    graphs::{DirectedGraph, Graph, dfs, dfs_mul},
    strings::char_at,
};

/// A *nondeterministic finite state automaton* (NFA) built from a lite version of Regular Expressions.
///
/// It supports *concatenation*, *closure*, *binary or*, and *parentheses*.
///
/// A better version is, of course, the [regex](https://docs.rs/regex/latest/regex/) crate.
#[must_use = "building the NFA does not evaluate it"]
pub struct RegExpNFA {
    graph: DirectedGraph,
    regexp: String,
}

/// Errors given during creation or running of the [RegExpNFA].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegularExpressionErrors {
    /// The expression is invalid at (at least) position `i`.
    #[allow(missing_docs)]
    InvalidRegularExpression { i: usize },

    /// The text given to [RegExpNFA::recognizes] has invalid character *c* at *i*
    #[allow(missing_docs)]
    InvalidCharacterInText { i: usize, c: char },
}

impl Display for RegularExpressionErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegularExpressionErrors::InvalidCharacterInText { i, c } => write!(
                f,
                "text contains the meta-character {} at position {}",
                c, i
            ),
            RegularExpressionErrors::InvalidRegularExpression { i } => {
                write!(f, "Invalid regular expression at {}", i)
            }
        }
    }
}

impl Error for RegularExpressionErrors {}

impl RegExpNFA {
    /// Builds the NFA from the given Regular Expression.
    ///
    /// Its running time is O(*m*), where *m* is the length of the regular expression.
    pub fn new(regexp: String) -> Result<Self, RegularExpressionErrors> {
        let mut ops = Stack::new();
        let mut graph = DirectedGraph::new(regexp.len() + 1);
        for i in 0..regexp.len() {
            let mut lp = i;
            if char_at(&regexp, i) == b'(' as i16
                || char_at(&regexp, i) == b'|' as i16
            {
                ops.push(i);
            } else if char_at(&regexp, i) == b')' as i16 {
                let or = ops.pop().ok_or(
                    RegularExpressionErrors::InvalidRegularExpression { i },
                )?;

                if char_at(&regexp, i) == b'|' as i16 {
                    lp = ops.pop().ok_or(
                        RegularExpressionErrors::InvalidRegularExpression { i },
                    )?;
                    graph.add_edge(lp, or + 1);
                    graph.add_edge(or, i);
                } else if char_at(&regexp, or) == b'(' as i16 {
                    lp = or;
                } else {
                    return Err(
                        RegularExpressionErrors::InvalidRegularExpression {
                            i: or,
                        },
                    );
                }
            }

            // closure operator (uses 1-character lookahead)
            if i < regexp.len() - 1 && char_at(&regexp, i + 1) == b'*' as i16 {
                graph.add_edge(lp, i + 1);
                graph.add_edge(i + 1, lp);
            }
            let i_c = char_at(&regexp, i);
            if i_c == b'(' as i16 || i_c == b'*' as i16 || i_c == b')' as i16 {
                graph.add_edge(i, i + 1);
            }
        }
        if let Some(i) = ops.pop() {
            Err(RegularExpressionErrors::InvalidRegularExpression { i })
        } else {
            Ok(Self { graph, regexp })
        }
    }

    /// Returns `true` if the text is matched by the regular expression.
    ///
    /// Its running time is O(*m n*), where *m* is the length of the regular expression
    /// and *n* is the amount of characters in the text.
    pub fn recognizes(
        &self,
        txt: &str,
    ) -> Result<bool, RegularExpressionErrors> {
        let m = self.regexp.len();
        let dfs = dfs(&self.graph, 0);
        let mut pc = Bag::new();
        for v in 0..self.graph.vertices() {
            if dfs.marked()[v] {
                pc.insert(v);
            }
        }

        // Compute possible NFA states for txt[i+1]
        for i in 0..txt.len() {
            let t_c = char_at(txt, i);
            if t_c == b'*' as i16
                || t_c == b'|' as i16
                || t_c == b'(' as i16
                || t_c == b')' as i16
            {
                return Err(RegularExpressionErrors::InvalidCharacterInText {
                    i,
                    c: t_c as u8 as char,
                });
            }

            let mut matches = Bag::new();
            for &v in &pc {
                if v == m {
                    continue;
                }
                let r_c = char_at(&self.regexp, v);
                let t_c = char_at(txt, v);
                if r_c == t_c || r_c == b'.' as i16 {
                    matches.insert(v + 1);
                }
            }
            if matches.is_empty() {
                continue;
            }

            let dfs = dfs_mul(&self.graph, matches.into_iter());
            pc = Bag::new();
            for v in 0..self.graph.vertices() {
                if dfs.marked()[v] {
                    pc.insert(v);
                }
            }

            // optimization if no states reachable
            if pc.is_empty() {
                return Ok(false);
            }
        }

        for v in pc {
            if v == m {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
