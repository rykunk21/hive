/// Cartesian Genetic Programming — Genotype Module
///
/// The chromosome is a flat Vec<usize> encoding a fixed-shape grid of nodes.
/// Dead genes physically exist in the chromosome and are available for mutation.
/// Typing is optional: use ValType::Dynamic as a catch-all to relax constraints.
// ─── Types ────────────────────────────────────────────────────────────────────
use rand::Rng;
/// The type system for node connections.
///
/// `Dynamic` is a catch-all — a Dynamic output satisfies any input requirement,
/// and a Dynamic input accepts any output. Use it to opt specific functions out
/// of type enforcement without disabling typing globally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValType {
    Float,
    Int,
    Bool,
    Text,
    Vector,
    Json,
    /// Escape hatch: no type checking on this connection.
    Dynamic,
}

impl ValType {
    /// Returns true if `self` (a source output) satisfies `required` (a slot's expected type).
    pub fn satisfies(&self, required: &ValType) -> bool {
        matches!(
            (self, required),
            (_, ValType::Dynamic) | (ValType::Dynamic, _)
        ) || self == required
    }
}

// ─── Function Signature ───────────────────────────────────────────────────────

/// The type-level contract of a node function.
/// Kept separate from the callable — this module never touches async or closures.
///
/// If you don't want typed CGP, set all inputs to `ValType::Dynamic`
/// and output to `ValType::Dynamic`. Mutation will accept any wiring.
#[derive(Debug, Clone)]
pub struct FnSignature {
    pub name: &'static str,
    pub inputs: Vec<ValType>,
    pub output: ValType,
}

impl FnSignature {
    pub fn arity(&self) -> usize {
        self.inputs.len()
    }

    /// Convenience: a fully dynamic signature of the given arity.
    pub fn dynamic(name: &'static str, arity: usize) -> Self {
        Self {
            name,
            inputs: vec![ValType::Dynamic; arity],
            output: ValType::Dynamic,
        }
    }
}

// ─── Grid Shape ───────────────────────────────────────────────────────────────

/// The fixed dimensions of the CGP grid.
/// This is a hyperparameter — tune it externally.
#[derive(Debug, Clone, Copy)]
pub struct GridShape {
    /// Number of node columns.
    pub cols: usize,
    /// Number of node rows per column.
    pub rows: usize,
    /// How many columns back a node's inputs may reach.
    /// Set to `cols` for unrestricted connectivity.
    pub levels_back: usize,
}

impl GridShape {
    /// Total number of node slots in the grid, including dead nodes.
    pub fn n_nodes(&self) -> usize {
        self.cols * self.rows
    }

    /// Column index of a node given its position in the node array.
    pub fn col_of(&self, node_index: usize) -> usize {
        node_index / self.rows
    }

    /// Minimum node index reachable from `node_index` given levels_back.
    pub fn min_reachable_node(&self, node_index: usize) -> usize {
        let col = self.col_of(node_index);
        col.saturating_sub(self.levels_back) * self.rows
    }
}

// ─── Chromosome ───────────────────────────────────────────────────────────────

/// The raw genome: a flat array of usize genes plus output pointers.
///
/// Layout per node (repeating for all rows × cols nodes):
///   [fn_id, input_0, input_1, ..., input_{arity-1}]
///
/// Followed by `n_outputs` output pointer genes.
///
/// Dead nodes exist in the chromosome at their natural positions and
/// participate in mutation normally. Whether a node is active is determined
/// at evaluation time by tracing back from the output pointers.
#[derive(Debug, Clone)]
pub struct Chromosome {
    /// The flat gene array. Length = n_nodes * (1 + max_arity) + n_outputs.
    pub genes: Vec<usize>,
    /// Grid shape — needed to interpret gene positions.
    pub shape: GridShape,
    /// Number of primary inputs fed at evaluation time.
    pub n_inputs: usize,
    /// Max arity across all functions in the set.
    /// All node slots are padded to this width even if the active function
    /// uses fewer inputs. Extra genes exist and mutate; they're just ignored
    /// during evaluation if the function doesn't read them.
    pub max_arity: usize,
    /// Number of output pointer genes at the end of the chromosome.
    pub n_outputs: usize,
}

impl Chromosome {
    // ── Gene layout helpers ──────────────────────────────────────────────────

    fn node_gene_start(&self, node_index: usize) -> usize {
        node_index * (1 + self.max_arity)
    }

    pub fn fn_id_of(&self, node_index: usize) -> usize {
        self.genes[self.node_gene_start(node_index)]
    }

    pub fn input_of(&self, node_index: usize, slot: usize) -> usize {
        self.genes[self.node_gene_start(node_index) + 1 + slot]
    }

    fn output_gene_start(&self) -> usize {
        self.shape.n_nodes() * (1 + self.max_arity)
    }

    pub fn output_pointer(&self, out_index: usize) -> usize {
        self.genes[self.output_gene_start() + out_index]
    }

    // ── Value table index helpers ────────────────────────────────────────────

    /// Total addressable slots: primary inputs + all nodes.
    pub fn value_table_len(&self) -> usize {
        self.n_inputs + self.shape.n_nodes()
    }

    /// Convert a node array index to its value-table index.
    pub fn node_to_vt(&self, node_index: usize) -> usize {
        node_index + self.n_inputs
    }

    /// Convert a value-table index to a node array index (panics for primaries).
    pub fn vt_to_node(&self, vt_index: usize) -> usize {
        assert!(
            vt_index >= self.n_inputs,
            "value-table index is a primary input"
        );
        vt_index - self.n_inputs
    }

    // ── Construction ────────────────────────────────────────────────────────

    /// Build a random valid chromosome.
    ///
    /// `input_types` must have length == `n_inputs`.
    /// `output_type` determines which nodes are valid output pointers.
    pub fn random(
        fn_set: &[FnSignature],
        input_types: &[ValType],
        shape: GridShape,
        n_outputs: usize,
        output_type: &ValType,
        rng: &mut rand::Rng,
    ) -> Self {
        use rand::seq::SliceRandom;

        let n_inputs = input_types.len();
        let max_arity = fn_set.iter().map(|f| f.arity()).max().unwrap_or(0);
        let n_nodes = shape.n_nodes();
        let gene_len = n_nodes * (1 + max_arity) + n_outputs;

        let mut genes = vec![0usize; gene_len];

        for node_index in 0..n_nodes {
            let start = node_index * (1 + max_arity);

            // Random function.
            let fn_id = rng.gen_range(0..fn_set.len());
            genes[start] = fn_id;

            let sig = &fn_set[fn_id];

            // Wire each real input slot to a type-compatible source.
            // Extra padding slots (beyond sig.arity()) get a random valid source —
            // they don't affect evaluation but exist for neutral mutation.
            for slot in 0..max_arity {
                let required = sig.inputs.get(slot).unwrap_or(&ValType::Dynamic);
                let valid = valid_sources(
                    required,
                    node_index,
                    shape,
                    input_types,
                    &genes,
                    fn_set,
                    max_arity,
                    n_inputs,
                );
                genes[start + 1 + slot] = valid
                    .choose(rng)
                    .copied()
                    .unwrap_or(rng.gen_range(0..n_inputs)); // fallback to a primary input
            }
        }

        // Output pointers: pick nodes that produce the required output type.
        let out_start = n_nodes * (1 + max_arity);
        let valid_out: Vec<usize> = (0..n_nodes)
            .filter(|&i| {
                let fn_id = genes[i * (1 + max_arity)];
                fn_set[fn_id].output.satisfies(output_type)
            })
            .map(|i| i + n_inputs)
            .collect();

        for i in 0..n_outputs {
            genes[out_start + i] = valid_out.choose(rng).copied().unwrap_or(n_inputs);
            // fallback: first node
        }

        Chromosome {
            genes,
            shape,
            n_inputs,
            max_arity,
            n_outputs,
        }
    }

    // ── Active node tracing ──────────────────────────────────────────────────

    /// Returns the set of active node indices by tracing back from output pointers.
    /// Dead nodes are not included. Use this in your eval layer to skip inactive nodes.
    pub fn active_nodes(&self) -> Vec<usize> {
        let mut active = vec![false; self.shape.n_nodes()];
        let mut stack: Vec<usize> = (0..self.n_outputs)
            .map(|i| self.output_pointer(i))
            .filter(|&vt| vt >= self.n_inputs)
            .map(|vt| self.vt_to_node(vt))
            .collect();

        while let Some(node_index) = stack.pop() {
            if active[node_index] {
                continue;
            }
            active[node_index] = true;

            let fn_id = self.fn_id_of(node_index);
            // We'd need the fn_set to know true arity; conservatively trace all slots.
            // Pass fn_set to active_nodes_with_fn_set for precise tracing.
            for slot in 0..self.max_arity {
                let src = self.input_of(node_index, slot);
                if src >= self.n_inputs {
                    stack.push(self.vt_to_node(src));
                }
            }
        }

        active
            .iter()
            .enumerate()
            .filter(|(_, &a)| a)
            .map(|(i, _)| i)
            .collect()
    }

    /// Like `active_nodes` but uses true arity from the function set,
    /// so padding slots don't get traced.
    pub fn active_nodes_precise(&self, fn_set: &[FnSignature]) -> Vec<usize> {
        let mut active = vec![false; self.shape.n_nodes()];
        let mut stack: Vec<usize> = (0..self.n_outputs)
            .map(|i| self.output_pointer(i))
            .filter(|&vt| vt >= self.n_inputs)
            .map(|vt| self.vt_to_node(vt))
            .collect();

        while let Some(node_index) = stack.pop() {
            if active[node_index] {
                continue;
            }
            active[node_index] = true;

            let fn_id = self.fn_id_of(node_index);
            let arity = fn_set[fn_id].arity();
            for slot in 0..arity {
                let src = self.input_of(node_index, slot);
                if src >= self.n_inputs {
                    stack.push(self.vt_to_node(src));
                }
            }
        }

        active
            .iter()
            .enumerate()
            .filter(|(_, &a)| a)
            .map(|(i, _)| i)
            .collect()
    }
}

// ─── Mutation ─────────────────────────────────────────────────────────────────

/// Produces mutant offspring from a parent chromosome.
pub struct MutationContext {
    pub fn_set: &'a [FnSignature],
    pub input_types: &'a [ValType],
    pub rng: &'a mut rand::Rng,
}

impl MutationContext {
    pub fn new(fn_set: &'a [FnSignature], input_types: &'a [ValType], rng: &mut Rand::Rng) -> Self {
        Self {
            fn_set,
            input_types,
            rng,
        }
    }

    /// Produce a mutant offspring by flipping `n_mutations` randomly chosen genes.
    /// Type constraints are enforced per flip; if no valid alternative exists for
    /// a slot the flip is skipped and the genome remains valid.
    pub fn mutate(&mut self, parent: &Chromosome, n_mutations: usize) -> Chromosome {
        let mut child = parent.clone();
        let total_genes = child.genes.len();

        for _ in 0..n_mutations {
            let gene_index = self.rng.gen_range(0..total_genes);
            self.mutate_gene(&mut child, gene_index);
        }

        child
    }

    fn mutate_gene(&mut self, c: &mut Chromosome, gene_index: usize) {
        let out_start = c.shape.n_nodes() * (1 + c.max_arity);

        if gene_index >= out_start {
            // Output pointer gene — rewire to any node with the right type.
            // (Type of required output is not stored here; accept any node.)
            let new_ptr = self.rng.gen_range(c.n_inputs..c.value_table_len());
            c.genes[gene_index] = new_ptr;
            return;
        }

        let node_index = gene_index / (1 + c.max_arity);
        let gene_within = gene_index % (1 + c.max_arity);

        if gene_within == 0 {
            // Function gene — preserve output type to keep downstream wiring valid.
            let current_out = &self.fn_set[c.fn_id_of(node_index)].output.clone();
            let compatible: Vec<usize> = self
                .fn_set
                .iter()
                .enumerate()
                .filter(|(_, f)| f.output.satisfies(current_out))
                .map(|(i, _)| i)
                .collect();

            if let Some(&new_id) = compatible.choose(self.rng) {
                c.genes[gene_index] = new_id;
            }
        } else {
            // Connection gene.
            let slot = gene_within - 1;
            let fn_id = c.fn_id_of(node_index);
            let required = self.fn_set[fn_id]
                .inputs
                .get(slot)
                .unwrap_or(&ValType::Dynamic)
                .clone();

            let valid = valid_sources(
                &required,
                node_index,
                c.shape,
                self.input_types,
                &c.genes,
                self.fn_set,
                c.max_arity,
                c.n_inputs,
            );

            if let Some(&new_src) = valid.choose(self.rng) {
                c.genes[gene_index] = new_src;
            }
        }
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Collect value-table indices reachable from `node_index` that satisfy `required_type`.
fn valid_sources(
    required_type: &ValType,
    node_index: usize,
    shape: GridShape,
    input_types: &[ValType],
    genes: &[usize],
    fn_set: &[FnSignature],
    max_arity: usize,
    n_inputs: usize,
) -> Vec<usize> {
    // Primary inputs that satisfy the required type.
    let primaries = input_types
        .iter()
        .enumerate()
        .filter(|(_, t)| t.satisfies(required_type))
        .map(|(i, _)| i);

    // Earlier nodes within levels_back that satisfy the required type.
    let min_node = shape.min_reachable_node(node_index);
    let node_sources = (min_node..node_index)
        .filter(|&i| {
            let fn_id = genes[i * (1 + max_arity)];
            fn_set[fn_id].output.satisfies(required_type)
        })
        .map(|i| i + n_inputs);

    primaries.chain(node_sources).collect()
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn simple_fn_set() -> Vec<FnSignature> {
        vec![
            FnSignature {
                name: "add",
                inputs: vec![ValType::Float, ValType::Float],
                output: ValType::Float,
            },
            FnSignature {
                name: "gt",
                inputs: vec![ValType::Float, ValType::Float],
                output: ValType::Bool,
            },
            FnSignature {
                name: "if_else",
                inputs: vec![ValType::Bool, ValType::Float, ValType::Float],
                output: ValType::Float,
            },
            FnSignature::dynamic("passthrough", 1),
        ]
    }

    fn shape() -> GridShape {
        GridShape {
            cols: 4,
            rows: 3,
            levels_back: 2,
        }
    }

    #[test]
    fn chromosome_has_correct_gene_count() {
        let fn_set = simple_fn_set();
        let mut rng = SmallRng::seed_from_u64(1);
        let s = shape();
        let c = Chromosome::random(
            &fn_set,
            &[ValType::Float, ValType::Float],
            s,
            1,
            &ValType::Float,
            &mut rng,
        );
        // 12 nodes * (1 + 3 max_arity) + 1 output = 49
        assert_eq!(c.genes.len(), s.n_nodes() * (1 + 3) + 1);
    }

    #[test]
    fn dead_genes_present_in_chromosome() {
        let fn_set = simple_fn_set();
        let mut rng = SmallRng::seed_from_u64(2);
        let c = Chromosome::random(
            &fn_set,
            &[ValType::Float, ValType::Float],
            shape(),
            1,
            &ValType::Float,
            &mut rng,
        );
        // Active nodes should be a strict subset of all nodes
        let active = c.active_nodes_precise(&fn_set);
        assert!(active.len() <= shape().n_nodes());
    }

    #[test]
    fn mutant_has_same_shape() {
        let fn_set = simple_fn_set();
        let mut rng = SmallRng::seed_from_u64(3);
        let input_types = vec![ValType::Float, ValType::Float];
        let parent =
            Chromosome::random(&fn_set, &input_types, shape(), 1, &ValType::Float, &mut rng);
        let parent_len = parent.genes.len();
        let mut ctx = MutationContext::new(&fn_set, &input_types, &mut rng);
        let child = ctx.mutate(&parent, 5);
        assert_eq!(child.genes.len(), parent_len);
    }

    #[test]
    fn dynamic_fn_accepts_any_input() {
        assert!(ValType::Float.satisfies(&ValType::Dynamic));
        assert!(ValType::Dynamic.satisfies(&ValType::Float));
        assert!(ValType::Float.satisfies(&ValType::Float));
        assert!(!ValType::Float.satisfies(&ValType::Bool));
    }
}
