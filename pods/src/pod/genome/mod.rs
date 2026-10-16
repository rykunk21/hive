use rand::{RngExt, SeedableRng, rngs::SmallRng};

/// A single node in the CGP grid.
/// `func` indexes into the pod's function set.
/// `inputs` are value-table indices (primary inputs first, then nodes).
#[derive(Debug, Clone)]
pub struct Node {
    pub func: usize,
    pub inputs: Vec<usize>,
}

/// The genome: a fixed grid of nodes plus output pointers.
#[derive(Debug, Clone)]
pub struct Genome {
    pub nodes: Vec<Node>,
    pub outputs: Vec<usize>, // value-table indices pointing to active outputs
    pub n_inputs: usize,
    pub arity: usize, // inputs per node (uniform for now)
}

impl Genome {
    /// Generate a random genome.
    /// `n_funcs`: how many functions are in the pod's function set.
    pub fn random(
        n_inputs: usize,
        n_nodes: usize,
        n_outputs: usize,
        n_funcs: usize,
        seed: u64,
    ) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let arity = 2;
        let pool_size = n_inputs + n_nodes;

        let nodes = (0..n_nodes)
            .map(|i| Node {
                func: rng.random_range(0..n_funcs),
                // inputs only point backward — no cycles
                inputs: (0..arity)
                    .map(|_| rng.random_range(0..(n_inputs + i).max(1)))
                    .collect(),
            })
            .collect();

        let outputs = (0..n_outputs)
            .map(|_| rng.random_range(n_inputs..pool_size))
            .collect();

        Self {
            nodes,
            outputs,
            n_inputs,
            arity,
        }
    }

    /// Mutate a clone of this genome, flipping `n` random genes.
    pub fn mutate(&self, n: usize, n_funcs: usize, seed: u64) -> Self {
        let mut child = self.clone();
        let mut rng = SmallRng::seed_from_u64(seed);

        for _ in 0..n {
            let node = rng.random_range(0..child.nodes.len());
            if rng.random_bool(0.5) {
                child.nodes[node].func = rng.random_range(0..n_funcs);
            } else {
                let slot = rng.random_range(0..child.arity);
                let max_src = (child.n_inputs + node).max(1);
                child.nodes[node].inputs[slot] = rng.random_range(0..max_src);
            }
        }

        child
    }

    /// Returns node indices reachable from the output pointers.
    pub fn active_nodes(&self) -> Vec<usize> {
        let mut active = vec![false; self.nodes.len()];
        let mut stack: Vec<usize> = self
            .outputs
            .iter()
            .filter(|&&o| o >= self.n_inputs)
            .map(|&o| o - self.n_inputs)
            .collect();

        while let Some(i) = stack.pop() {
            if i >= self.nodes.len() || active[i] {
                continue;
            }
            active[i] = true;
            for &src in &self.nodes[i].inputs {
                if src >= self.n_inputs {
                    stack.push(src - self.n_inputs);
                }
            }
        }

        active
            .into_iter()
            .enumerate()
            .filter(|(_, a)| *a)
            .map(|(i, _)| i)
            .collect()
    }
}
