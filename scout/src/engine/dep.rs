use rhai::{ASTNode, Engine as RhaiEngine};

pub fn get_defines_and_uses(engine: &RhaiEngine, script: &str) -> (Vec<String>, Vec<String>) {
    let ast = match engine.compile(script) {
        Ok(ast) => ast,
        Err(_) => return (vec![], vec![]),
    };

    let mut defines = Vec::new();
    let mut uses = Vec::new();
    ast.walk(&mut |nodes| {
        if let Some(node) = nodes.last() {
            match node {
                ASTNode::Stmt(rhai::Stmt::Var(x, ..)) => {
                    defines.push(x.0.name.to_string());
                }

                ASTNode::Stmt(rhai::Stmt::Assignment(x, ..)) => {

                    // reassignment uses the variable, never defines it
                    if let rhai::Expr::Variable(v, ..) = &x.1.lhs {
                        let name = v.1.to_string();
                        if !defines.contains(&name) {
                            uses.push(name);
                        }
                    }
                }

                ASTNode::Expr(rhai::Expr::Variable(x, ..)) => {
                    let name = x.1.to_string();
                    if !defines.contains(&name) {
                        uses.push(name);
                    }
                }
                _ => {}
            }
        }
        true
    });

    uses.dedup();
    (defines, uses)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_engine() -> RhaiEngine {
        RhaiEngine::new()
    }

    #[test]
    fn returns_empty_on_syntax_error() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "!!!");
        assert!(defs.is_empty());
        assert!(uses.is_empty());
    }

    #[test]
    fn returns_empty_on_empty_script() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "");
        assert!(defs.is_empty());
        assert!(uses.is_empty());
    }

    #[test]
    fn simple_define() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "let x = 1;");
        assert_eq!(defs, vec!["x"]);
        assert!(uses.is_empty());
    }

    #[test]
    fn simple_use() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "x;");
        assert!(defs.is_empty());
        assert_eq!(uses, vec!["x"]);
    }

    #[test]
    fn define_then_use() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "let x = 1; x + 2;");
        assert_eq!(defs, vec!["x"]);
        assert!(uses.is_empty());
    }

    #[test]
    fn use_in_initializer() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "let x = a;");
        assert_eq!(defs, vec!["x"]);
        assert_eq!(uses, vec!["a"]);
    }

    #[test]
    fn define_then_use_in_initializer() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "let x = 1; let y = x + 2;");
        assert_eq!(defs, vec!["x", "y"]);
        assert!(uses.is_empty());
    }

    #[test]
    fn assignment_without_define_is_use() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "x = 1;");
        assert!(defs.is_empty());
        assert_eq!(uses, vec!["x"]);
    }

    #[test]
    fn assignment_after_define_is_filtered() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "let x = 1; x = 2;");
        assert_eq!(defs, vec!["x"]);
        assert!(uses.is_empty());
    }

    #[test]
    fn multiple_uses_sorted_check() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "x + y;");
        assert!(defs.is_empty());
        let mut uses = uses;
        uses.sort();
        assert_eq!(uses, vec!["x", "y"]);
    }

    #[test]
    fn deduplicates_adjacent_uses() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "x + x;");
        assert!(defs.is_empty());
        assert_eq!(uses, vec!["x"]);
    }

    #[test]
    fn use_before_define_counts_as_use() {
        let engine = make_engine();
        let (defs, uses) = get_defines_and_uses(&engine, "x; let x = 1;");
        assert_eq!(defs, vec!["x"]);
        assert_eq!(uses, vec!["x"]);
    }
}
