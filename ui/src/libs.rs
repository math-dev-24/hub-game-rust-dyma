use console::Term;

pub fn clear_terminal() {
    let term: Term = Term::stdout();
    term.clear_screen().expect("Failled to clear terminal");
}