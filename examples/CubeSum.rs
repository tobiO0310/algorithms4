use std::{
    cmp::Ordering,
    error::Error,
    fmt::Display,
    io::{self, BufRead, StdinLock, Write, stdout},
};

use algorithms4::PriorityQueue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Node(usize, usize, usize);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}^3 + {}^3", self.0, self.1, self.2)
    }
}

fn get_usize_from_stdin(stdin: &mut StdinLock<'_>) -> Result<usize, String> {
    let mut buf = String::new();
    stdin.read_line(&mut buf).map_err(|a| a.to_string())?;
    buf.trim().parse::<usize>().map_err(|e| e.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    print!("Please input N: ");
    let _ = stdout().flush();
    let n = get_usize_from_stdin(&mut stdin)?;

    if n.checked_pow(3).is_none_or(|n| n.checked_mul(2).is_none()) {
        panic!("N is too high for your computer to utilize a usize!");
    }

    let mut pq = PriorityQueue::new();

    for i in 0..n {
        pq.insert(Node(i.pow(3), i, 0));
    }

    while !pq.is_empty() {
        let node = pq.pop().unwrap();
        println!("{}", node);
        if node.2 < n {
            let Node(_, i, j) = node;
            pq.insert(Node(i.pow(3) + (j + 1).pow(3), i, j + 1));
        }
    }

    Ok(())
}
