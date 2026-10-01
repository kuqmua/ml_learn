//! Публичная учебная операция должна быть показана в main своего крейта.
//! Проверяем синтаксическое дерево Rust: комментарии, строки и невызванные функции
//! не считаются демонстрацией. Численное поведение проверяют тесты самих уроков.

type FunctionPath = Vec<String>;

fn imports(
    tree: &syn::UseTree,
    prefix: FunctionPath,
    names: &mut std::collections::BTreeMap<String, FunctionPath>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            let mut prefix = prefix;
            prefix.push(path.ident.to_string());
            imports(&path.tree, prefix, names);
        }
        syn::UseTree::Name(name) => {
            let mut path = prefix;
            if name.ident != "self" {
                path.push(name.ident.to_string());
            }
            if let Some(name) = path.last() {
                names.insert(name.clone(), path);
            }
        }
        syn::UseTree::Rename(rename) => {
            let mut path = prefix;
            if rename.ident != "self" {
                path.push(rename.ident.to_string());
            }
            names.insert(rename.rename.to_string(), path);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                imports(item, prefix.clone(), names);
            }
        }
        // Explicit imports make the owner of a demonstrated operation unambiguous.
        syn::UseTree::Glob(_) => {}
    }
}

struct Calls {
    names: std::collections::BTreeMap<String, FunctionPath>,
    paths: std::collections::BTreeSet<FunctionPath>,
}

impl<'ast> syn::visit::Visit<'ast> for Calls {
    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {
        // A nested helper definition is not a call from main.
    }

    fn visit_item_mod(&mut self, _: &'ast syn::ItemMod) {}

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        imports(&item.tree, Vec::new(), &mut self.names);
    }

    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = &*call.func {
            let mut path: Vec<_> = function
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect();
            if let Some(imported) = path.first().and_then(|name| self.names.get(name)) {
                let mut resolved = imported.clone();
                resolved.extend(path.into_iter().skip(1));
                path = resolved;
            }
            self.paths.insert(path);
        }
        syn::visit::visit_expr_call(self, call);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        // Calls inside assert_eq! and other macros are expressions too.
        let expressions =
            syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
        if let Ok(expressions) = syn::parse::Parser::parse2(expressions, mac.tokens.clone()) {
            for expression in &expressions {
                syn::visit::Visit::visit_expr(self, expression);
            }
        }
    }
}

fn calls_in_main(source: &str) -> std::collections::BTreeSet<FunctionPath> {
    let file = syn::parse_file(source).expect("main.rs должен быть корректным Rust");
    let mut calls = Calls {
        names: std::collections::BTreeMap::new(),
        paths: std::collections::BTreeSet::new(),
    };
    for item in &file.items {
        if let syn::Item::Use(item) = item {
            imports(&item.tree, Vec::new(), &mut calls.names);
        }
    }
    let main = file
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Fn(function) if function.sig.ident == "main" => Some(function),
            _ => None,
        })
        .expect("у учебного крейта должен быть main");
    syn::visit::Visit::visit_block(&mut calls, &main.block);
    calls.paths
}

fn exported_functions(
    items: &[syn::Item],
    directory: &std::path::Path,
    prefix: &[String],
    result: &mut Vec<FunctionPath>,
) {
    for item in items {
        match item {
            syn::Item::Fn(function) if matches!(function.vis, syn::Visibility::Public(_)) => {
                let mut path = prefix.to_vec();
                path.push(function.sig.ident.to_string());
                result.push(path);
            }
            syn::Item::Mod(module) if matches!(module.vis, syn::Visibility::Public(_)) => {
                let mut path = prefix.to_vec();
                path.push(module.ident.to_string());
                let child = directory.join(module.ident.to_string());
                if let Some((_, items)) = &module.content {
                    exported_functions(items, &child, &path, result);
                } else {
                    let file = if child.with_extension("rs").is_file() {
                        child.with_extension("rs")
                    } else {
                        child.join("mod.rs")
                    };
                    let source = std::fs::read_to_string(&file)
                        .unwrap_or_else(|error| panic!("{}: {error}", file.display()));
                    let parsed =
                        syn::parse_file(&source).expect("модуль должен быть корректным Rust");
                    exported_functions(&parsed.items, &child, &path, result);
                }
            }
            syn::Item::Use(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
                panic!(
                    "Учебные операции должны иметь один крейт-владелец; используйте прямую зависимость вместо pub use"
                );
            }
            _ => {}
        }
    }
}

#[test]
fn every_public_lesson_operation_is_demonstrated_in_its_own_main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let mut missing = Vec::new();
    let mut checked = 0;
    let mut lessons = 0;
    for entry in std::fs::read_dir(root).unwrap() {
        let directory = entry.unwrap().path();
        let name = directory.file_name().unwrap().to_string_lossy();
        if !directory.is_dir()
            || name.len() < 5
            || name.as_bytes()[0] != b'l'
            || !name.as_bytes()[1..4].iter().all(u8::is_ascii_digit)
            || name.as_bytes()[4] != b'-'
        {
            continue;
        }
        lessons += 1;
        let source = directory.join("src");
        let functions = {
            let library = match std::fs::read_to_string(source.join("lib.rs")) {
                Ok(source) => source,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
                Err(error) => panic!("{}: {error}", source.display()),
            };
            let parsed = syn::parse_file(&library).expect("lib.rs должен быть корректным Rust");
            let mut functions = Vec::new();
            let crate_name = name.replace('-', "_");
            exported_functions(&parsed.items, &source, &[crate_name], &mut functions);
            functions
        };
        let calls = {
            let main = std::fs::read_to_string(source.join("main.rs"))
                .expect("у урока должен быть main.rs");
            calls_in_main(&main)
        };
        for function in functions {
            checked += 1;
            if !calls.contains(&function) {
                missing.push(format!("{name}: {}", function.join("::")));
            }
        }
    }
    assert!(
        lessons > 0 && checked > 0,
        "проверка не должна проходить на пустом наборе уроков"
    );
    assert!(
        missing.is_empty(),
        "Публичные функции без демонстрации в main своего крейта:\n{}\nВынесите самостоятельную операцию в отдельную часть с main и тестами.",
        missing.join("\n")
    );
}

#[test]
fn recognizes_qualified_calls_imports_aliases_and_macro_arguments() {
    let calls = calls_in_main(
        r#"
        use lesson::{calculate as operation, nested};
        use lesson as own;
        fn main() {
            operation();
            nested::run();
            own::another();
            assert_eq!(lesson::inside_assert(), 1);
            use lesson::local;
            local();
        }
    "#,
    );
    for path in [
        "lesson::calculate",
        "lesson::nested::run",
        "lesson::another",
        "lesson::inside_assert",
        "lesson::local",
    ] {
        assert!(calls.contains(&path.split("::").map(str::to_owned).collect::<Vec<_>>()));
    }
}

#[test]
fn comments_strings_unused_helpers_and_other_crates_do_not_demonstrate_a_function() {
    let calls = calls_in_main(
        r#"
        fn helper() { lesson::calculate(); }
        #[test] fn test_only() { lesson::calculate(); }
        fn main() {
            // lesson::calculate();
            let _ = "lesson::calculate()";
            fn unused() { lesson::calculate(); }
            other_lesson::calculate();
        }
    "#,
    );
    assert!(!calls.contains(&vec!["lesson".into(), "calculate".into()]));
}

#[test]
fn indirect_library_call_still_needs_its_own_demonstration() {
    let library =
        syn::parse_file("pub fn distance() { squared(); } pub fn squared() {} fn helper() {}")
            .unwrap();
    let mut functions = Vec::new();
    exported_functions(
        &library.items,
        std::path::Path::new("."),
        &["lesson".into()],
        &mut functions,
    );
    let calls = calls_in_main("fn main() { lesson::distance(); }");
    let missing: Vec<_> = functions
        .into_iter()
        .filter(|function| !calls.contains(function))
        .collect();
    assert_eq!(
        missing,
        vec![vec!["lesson".to_string(), "squared".to_string()]]
    );
}
