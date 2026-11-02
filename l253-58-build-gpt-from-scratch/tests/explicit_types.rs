//! Правило учебного кода: типы всех именованных локальных переменных и параметров замыканий явные.
use std::path::{Path, PathBuf};
use syn::{
    Expr, Pat, Token,
    parse::Parser,
    punctuated::Punctuated,
    visit::{self, Visit},
};

#[derive(Default)]
struct Audit {
    problems: Vec<String>,
    locals: usize,
    closures: usize,
}
impl<'ast> Visit<'ast> for Audit {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        self.locals += 1;
        if !matches!(local.pat, Pat::Type(_)) {
            self.problems.push("let без явной аннотации типа".to_owned());
        }
        visit::visit_local(self, local);
    }
    fn visit_expr_closure(&mut self, closure: &'ast syn::ExprClosure) {
        self.closures += 1;
        if !closure.inputs.iter().all(|pattern: &Pat| -> bool { matches!(pattern, Pat::Type(_)) }) {
            self.problems.push("параметр замыкания без явного типа".to_owned());
        }
        visit::visit_expr_closure(self, closure);
    }
    fn visit_type_infer(&mut self, _: &'ast syn::TypeInfer) {
        self.problems.push("тип-заполнитель _, например Vec<_>".to_owned());
    }
    fn visit_expr_for_loop(&mut self, expression: &'ast syn::ExprForLoop) {
        // for _ не вводит переменную. Именованные переменные требуют typed closure.
        if !matches!(*expression.pat, Pat::Wild(_)) {
            self.problems.push("именованная переменная for без аннотации типа".to_owned());
        }
        visit::visit_expr_for_loop(self, expression);
    }
    fn visit_expr_let(&mut self, expression: &'ast syn::ExprLet) {
        self.problems.push("if let/while let с неявными типами привязок".to_owned());
        visit::visit_expr_let(self, expression);
    }
    fn visit_arm(&mut self, arm: &'ast syn::Arm) {
        // Чтобы не оставлять неявные Some(value) в match, разрешаем только ветки без привязок.
        struct Bindings(usize);
        impl<'a> Visit<'a> for Bindings {
            fn visit_pat_ident(&mut self, _: &'a syn::PatIdent) {
                self.0 += 1;
            }
        }
        let mut bindings: Bindings = Bindings(0);
        bindings.visit_pat(&arm.pat);
        if bindings.0 > 0 {
            self.problems.push("привязка match без явного типа".to_owned());
        }
        visit::visit_arm(self, arm);
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        // Проверяем и выражения в assert!, vec!, float_setting! и т. п.
        // Определения macro_rules содержат метапеременные, а не Rust-выражения.
        let _: Option<()> = Punctuated::<Expr, Token![,]>::parse_terminated
            .parse2(mac.tokens.clone())
            .map(|expressions: Punctuated<Expr, Token![,]>| {
                expressions.iter().for_each(|expression: &Expr| {
                    self.visit_expr(expression);
                });
            })
            .ok();
    }
}

fn audit(source: &str) -> Audit {
    let file: syn::File = syn::parse_file(source).expect("Корректный синтаксис Rust");
    let mut result: Audit = Audit::default();
    result.visit_file(&file);
    result
}

#[test]
fn every_lesson_variable_has_an_explicit_type() {
    let root: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let paths: Vec<PathBuf> = ["src", "tests"]
        .into_iter()
        .flat_map(|directory: &str| {
            std::fs::read_dir(root.join(directory))
                .unwrap()
                .map(|entry: std::io::Result<std::fs::DirEntry>| -> PathBuf { entry.unwrap().path() })
        })
        .filter(|path: &PathBuf| -> bool { path.extension().is_some_and(|ext: &std::ffi::OsStr| -> bool { ext == "rs" }) })
        .collect::<Vec<PathBuf>>();
    let mut locals: usize = 0;
    let mut closures: usize = 0;
    paths.iter().for_each(|path: &PathBuf| {
        let source: String = std::fs::read_to_string(path).unwrap();
        let result: Audit = audit(&source);
        assert!(result.problems.is_empty(), "{}: {:?}", path.display(), result.problems);
        locals += result.locals;
        closures += result.closures;
    });
    println!("Проверено {} файлов: {locals} локальных объявлений, {closures} замыканий", paths.len());
    assert!(locals > 100 && closures > 20, "Проверка должна охватывать весь урок");
}

#[test]
fn audit_rejects_implicit_types_including_macro_arguments() {
    [
        "fn f(){ let x=1; }",
        "fn f(){ let x: Vec<_>=vec![1]; }",
        "fn f(){ let x: bool = [1].iter().any(|x| *x>0); }",
        "fn f(){ for x in 0..2 {} }",
        "fn f(){ assert!([1].iter().any(|x| *x>0)); }",
        "fn f(){ match Some(1) { Some(x) => {}, _ => {} } }",
    ]
    .into_iter()
    .for_each(|source: &str| {
        assert!(!audit(source).problems.is_empty(), "{source}");
    });
    let explicit: Audit = audit("fn f(){ let (x,y): (u32,u32)=(1,2); let v: Vec<u32>=vec![x,y]; v.iter().for_each(|x: &u32| {}); }");
    assert!(explicit.problems.is_empty());
}
