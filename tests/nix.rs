use nickel_lang::term::Term;
use nickel_lang_utilities::eval;

fn run(path: &str) {
    let res = eval(format!(
        "let t = import \"{}/tests/nix/{}\" in array.fold (fun x acc => acc && x) true t",
        env!("CARGO_MANIFEST_DIR"),
        path
    ))
    .map(|rt| {
        let term = Term::from(rt);
        assert_eq!(term, Term::Bool(true), "error in test {}", path,);
    })
    .unwrap();
}

#[test]
fn basics_nix() {
    run("basics.nix");
}
