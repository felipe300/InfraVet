use dockerfile_parser::Instruction;

#[derive(Debug, Default)]
pub struct AnalysisContext {
    pub has_from: bool,
    pub stages_count: usize,
    pub user_in_current_stage: bool,
    pub has_healthcheck: bool,
}

impl AnalysisContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Actualiza el contexto según la instrucción actual
    pub fn update(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::From(_) => {
                self.has_from = true;
                self.stages_count += 1;
                self.user_in_current_stage = false;
            }
            Instruction::Misc(misc) => {
                let name = misc.instruction.content.as_str();
                if name.eq_ignore_ascii_case("User") {
                    self.user_in_current_stage = true;
                } else if name.eq_ignore_ascii_case("HEALTHCHECK") {
                    self.has_healthcheck = true;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dockerfile_parser::Dockerfile;

    #[test]
    fn test_context_tracking() {
        let content = "FROM ubuntu:22.04\nUSER app\nFROM alpine:latest";
        let dockerfile = Dockerfile::parse(content).unwrap();

        let mut ctx = AnalysisContext::new();
        for ins in &dockerfile.instructions {
            ctx.update(ins);
        }

        assert_eq!(ctx.stages_count, 2);
        assert!(ctx.has_from);
        // En el segundo stage no hay USER, así que debe ser false
        assert!(!ctx.user_in_current_stage);
    }
}
