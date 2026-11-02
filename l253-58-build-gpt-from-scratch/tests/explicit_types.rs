//! Правило учебного кода: типы всех именованных локальных переменных и параметров замыканий явные.
use std::path::{Path, PathBuf};
use syn::{
    Expr, Pat, Token,
    parse::Parser,
    punctuated::Punctuated,
    visit::{self, Visit},
};

// Смысл имени требует чтения кода; эта проверка дополнительно запрещает
// однобуквенные привязки и прежние неоднозначные сокращения. Поля сторонних
// struct literal (например ParamsAdamW.lr) не являются определениями наших полей.
fn is_unclear_name(identifier: &str) -> bool {
    identifier.len() == 1 || matches!(identifier, "lr" | "eps" | "rng" | "vocab" | "params" | "ln1" | "ln2" | "ff")
}

#[derive(Default)]
struct ExplicitTypeAudit {
    diagnostics: Vec<String>,
    local_binding_count: usize,
    closure_count: usize,
}
impl<'ast> Visit<'ast> for ExplicitTypeAudit {
    fn visit_pat_ident(&mut self, binding: &'ast syn::PatIdent) {
        let binding_name: String = binding.ident.to_string();
        if is_unclear_name(&binding_name) {
            self.diagnostics.push(format!("Неясное имя переменной или аргумента: {binding_name}"));
        }
        visit::visit_pat_ident(self, binding);
    }
    fn visit_field(&mut self, named_field: &'ast syn::Field) {
        if named_field
            .ident
            .as_ref()
            .is_some_and(|field_name: &syn::Ident| -> bool { is_unclear_name(&field_name.to_string()) })
        {
            self.diagnostics.push("Неясное сокращённое имя поля".to_owned());
        }
        visit::visit_field(self, named_field);
    }

    fn visit_local(&mut self, local_binding: &'ast syn::Local) {
        self.local_binding_count += 1;
        if !matches!(local_binding.pat, Pat::Type(_)) {
            self.diagnostics.push("let без явной аннотации типа".to_owned());
        }
        visit::visit_local(self, local_binding);
    }
    fn visit_expr_closure(&mut self, closure_expression: &'ast syn::ExprClosure) {
        self.closure_count += 1;
        if !closure_expression
            .inputs
            .iter()
            .all(|binding_pattern: &Pat| -> bool { matches!(binding_pattern, Pat::Type(_)) })
        {
            self.diagnostics.push("параметр замыкания без явного типа".to_owned());
        }
        visit::visit_expr_closure(self, closure_expression);
    }
    fn visit_type_infer(&mut self, _: &'ast syn::TypeInfer) {
        self.diagnostics.push("тип-заполнитель _, например Vec<_>".to_owned());
    }
    fn visit_expr_for_loop(&mut self, expression: &'ast syn::ExprForLoop) {
        // for _ не вводит переменную. Именованные переменные требуют typed closure.
        if !matches!(*expression.pat, Pat::Wild(_)) {
            self.diagnostics.push("именованная переменная for без аннотации типа".to_owned());
        }
        visit::visit_expr_for_loop(self, expression);
    }
    fn visit_expr_let(&mut self, expression: &'ast syn::ExprLet) {
        self.diagnostics.push("if let/while let с неявными типами привязок".to_owned());
        visit::visit_expr_let(self, expression);
    }
    fn visit_arm(&mut self, match_arm: &'ast syn::Arm) {
        // Чтобы не оставлять неявные Some(value) в match, разрешаем только ветки без привязок.
        struct PatternBindingCounter {
            binding_count: usize,
        }
        impl<'a> Visit<'a> for PatternBindingCounter {
            fn visit_pat_ident(&mut self, _: &'a syn::PatIdent) {
                self.binding_count += 1;
            }
        }
        let mut pattern_binding_counter: PatternBindingCounter = PatternBindingCounter { binding_count: 0 };
        pattern_binding_counter.visit_pat(&match_arm.pat);
        if pattern_binding_counter.binding_count > 0 {
            self.diagnostics.push("привязка match без явного типа".to_owned());
        }
        visit::visit_arm(self, match_arm);
    }
    fn visit_macro(&mut self, macro_invocation: &'ast syn::Macro) {
        // Проверяем и выражения в assert!, vec!, float_setting! и т. п.
        // Определения macro_rules содержат метапеременные, а не Rust-выражения.
        let _: Option<()> = Punctuated::<Expr, Token![,]>::parse_terminated
            .parse2(macro_invocation.tokens.clone())
            .map(|macro_argument_expressions: Punctuated<Expr, Token![,]>| {
                macro_argument_expressions.iter().for_each(|expression: &Expr| {
                    self.visit_expr(expression);
                });
            })
            .ok();
    }
}

fn audit(source_code: &str) -> ExplicitTypeAudit {
    let syntax_tree: syn::File = syn::parse_file(source_code).expect("Корректный синтаксис Rust");
    let mut type_audit: ExplicitTypeAudit = ExplicitTypeAudit::default();
    type_audit.visit_file(&syntax_tree);
    type_audit
}

#[test]
fn every_lesson_binding_has_explicit_type_and_descriptive_name() {
    let lesson_directory: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let rust_source_paths: Vec<PathBuf> = ["src", "tests"]
        .into_iter()
        .flat_map(|subdirectory_name: &str| {
            std::fs::read_dir(lesson_directory.join(subdirectory_name))
                .unwrap()
                .map(|directory_entry_result: std::io::Result<std::fs::DirEntry>| -> PathBuf { directory_entry_result.unwrap().path() })
        })
        .filter(|source_path: &PathBuf| -> bool {
            source_path
                .extension()
                .is_some_and(|file_extension: &std::ffi::OsStr| -> bool { file_extension == "rs" })
        })
        .collect::<Vec<PathBuf>>();
    let mut local_binding_count: usize = 0;
    let mut closure_count: usize = 0;
    rust_source_paths.iter().for_each(|source_path: &PathBuf| {
        let source_code: String = std::fs::read_to_string(source_path).unwrap();
        let type_audit: ExplicitTypeAudit = audit(&source_code);
        assert!(type_audit.diagnostics.is_empty(), "{}: {:?}", source_path.display(), type_audit.diagnostics);
        local_binding_count += type_audit.local_binding_count;
        closure_count += type_audit.closure_count;
    });
    println!(
        "Проверено {} файлов: {local_binding_count} локальных объявлений, {closure_count} замыканий",
        rust_source_paths.len()
    );
    assert!(local_binding_count > 100 && closure_count > 20, "Проверка должна охватывать весь урок");
}

#[test]
fn audit_rejects_implicit_types_including_macro_arguments() {
    [
        "fn example(){ let untyped_value=1; }",
        "fn example(){ let values: Vec<_>=vec![1]; }",
        "fn example(){ let has_positive_value: bool = [1].iter().any(|value| *value>0); }",
        "fn example(){ for value in 0..2 {} }",
        "fn example(){ assert!([1].iter().any(|value| *value>0)); }",
        "fn example(){ match Some(1) { Some(value) => {}, _ => {} } }",
    ]
    .into_iter()
    .for_each(|source_code: &str| {
        assert!(!audit(source_code).diagnostics.is_empty(), "{source_code}");
    });
    let explicitly_typed_audit: ExplicitTypeAudit = audit(
        "fn example(){ let (left_value,right_value): (u32,u32)=(1,2); let values: Vec<u32>=vec![left_value,right_value]; values.iter().for_each(|value: &u32| {}); }",
    );
    assert!(explicitly_typed_audit.diagnostics.is_empty());
}

#[test]
fn audit_rejects_unclear_names_in_variables_arguments_and_fields() {
    ["fn example(){let q: u32=1;}", "fn example(p: u32){}", "struct Example{lr: f64}"]
        .into_iter()
        .for_each(|source_code: &str| {
            assert!(!audit(source_code).diagnostics.is_empty());
        });
    assert!(
        audit("struct OptimizerSettings{learning_rate:f64} fn example(token_id:u32){let next_token_id:u32=token_id+1;}")
            .diagnostics
            .is_empty()
    );
}
