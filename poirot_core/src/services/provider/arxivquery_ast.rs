// Complete arXiv query AST + parser + serializer.
//
// Supported user syntax:
// - Terms: `ti:"quantum"`, `au:"John Doe"`, or unprefixed terms like `quantum` (defaults to `all:`)
// - Operators: `&&` (AND), `||` (OR), `!&&` (ANDNOT)
// - Parentheses: `( ... )`
// - Implicit AND: whitespace between two terms/groups means AND
//
// Output:
// - arXiv query syntax using `+AND+`, `+OR+`, `+ANDNOT+`
pub mod ast {

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Prefix {
        Au,
        Ti,
        Abs,
        Co,
        Jr,
        Cat,
        Rn,
        #[default]
        All,
    }

    impl Prefix {
        pub fn as_str(self) -> &'static str {
            match self {
                Prefix::Au => "au:",
                Prefix::Ti => "ti:",
                Prefix::Abs => "abs:",
                Prefix::Co => "co:",
                Prefix::Jr => "jr:",
                Prefix::Cat => "cat:",
                Prefix::Rn => "rn:",
                Prefix::All => "all:",
            }
        }

        pub fn parse(s: &str) -> Self {
            match s {
                "au" => Prefix::Au,
                "ti" => Prefix::Ti,
                "abs" => Prefix::Abs,
                "co" => Prefix::Co,
                "jr" => Prefix::Jr,
                "cat" => Prefix::Cat,
                "rn" => Prefix::Rn,
                "all" => Prefix::All,
                _ => Prefix::All, // default to All for unknown prefixes
            }
        }
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
    pub struct Term {
        pub prefix: Prefix,
        pub value: String,
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
    pub enum Expr {
        #[default]
        Empty,
        Term(Term),
        And(Vec<Expr>),               // and can be in any order
        Or(Vec<Expr>),                // or can be in any order
        AndNot(Box<Expr>, Box<Expr>), // a !&& b
    }

    impl Expr {
        pub fn term(prefix: Prefix, value: impl Into<String>) -> Self {
            let v = value.into();
            // add empty value
            if v.is_empty() {
                Expr::Empty
            } else {
                Expr::Term(Term { prefix, value: v })
            }
        }

        pub fn and(lhs: Expr, rhs: Expr) -> Expr {
            match (lhs, rhs) {
                // Extend two end node in a single larger and node
                (Expr::And(mut a), Expr::And(b)) => {
                    a.extend(b);
                    Expr::And(a)
                }
                (Expr::And(mut a), b) => {
                    a.push(b);
                    Expr::And(a)
                }
                (a, Expr::And(mut b)) => {
                    let mut out = Vec::with_capacity(b.len() + 1);
                    out.push(a);
                    out.append(&mut b);
                    Expr::And(out)
                }
                (a, b) => Expr::And(vec![a, b]),
            }
        }

        pub fn or(lhs: Expr, rhs: Expr) -> Expr {
            match (lhs, rhs) {
                (Expr::Or(mut a), Expr::Or(b)) => {
                    a.extend(b);
                    Expr::Or(a)
                }
                (Expr::Or(mut a), b) => {
                    a.push(b);
                    Expr::Or(a)
                }
                (a, Expr::Or(mut b)) => {
                    let mut out = Vec::with_capacity(b.len() + 1);
                    out.push(a);
                    out.append(&mut b);
                    Expr::Or(out)
                }
                (a, b) => Expr::Or(vec![a, b]),
            }
        }
    }
}

pub mod visit {
    use super::ast::{Expr, Term};

    pub trait Visitor {
        type Output;

        fn visit_term(&mut self, term: &Term) -> Self::Output;
        fn visit_expr(&mut self, expr: &Expr) -> Self::Output;
    }
}

pub mod parse {
    use super::ast::{Expr, Prefix, Term};
    use crate::services::provider::constants::arxiv_categories;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ParseError {
        pub message: String,
        pub byte_index: usize,
    }

    impl std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} at byte {}", self.message, self.byte_index)
        }
    }

    impl std::error::Error for ParseError {}

    pub fn parse_expr(input: &str) -> Result<Expr, ParseError> {
        let mut parser = Parser { input, i: 0 };
        let expr = parser.parse_bp(0)?;
        parser.skip_ws();
        if !parser.eof() {
            return Err(parser.err("unexpected trailing input"));
        }
        if matches!(expr, Expr::Empty) {
            return Err(ParseError {
                message: "empty expression".to_string(),
                byte_index: 0,
            });
        }
        Ok(expr)
    }

    struct Parser<'a> {
        input: &'a str,
        i: usize,
    }

    impl<'a> Parser<'a> {
        fn eof(&self) -> bool {
            self.i >= self.input.len()
        }

        fn err(&self, message: &str) -> ParseError {
            ParseError {
                message: message.to_string(),
                byte_index: self.i,
            }
        }

        fn skip_ws(&mut self) {
            while let Some(ch) = self.peek_char() {
                if ch.is_whitespace() {
                    self.bump_char();
                } else {
                    break;
                }
            }
        }

        fn peek_char(&self) -> Option<char> {
            self.input[self.i..].chars().next()
        }

        fn bump_char(&mut self) -> Option<char> {
            let ch = self.peek_char()?;
            self.i += ch.len_utf8();
            Some(ch)
        }

        fn starts_with(&self, s: &str) -> bool {
            self.input[self.i..].starts_with(s)
        }

        fn consume(&mut self, s: &str) -> bool {
            if self.starts_with(s) {
                self.i += s.len();
                true
            } else {
                false
            }
        }

        /// Returns true if the next token begins a primary expression.
        /// A primary expression is a term or a parenthesized expression.
        fn next_begins_primary(&mut self) -> bool {
            self.skip_ws();
            if self.eof() {
                return false;
            }
            matches!(
                self.peek_char(),
                Some('(')
                    | Some('"')
                    | Some('a'..='z')
                    | Some('A'..='Z')
                    | Some('0'..='9')
                    | Some('_')
            )
        }

        /// Pratt parser for binary operators with precedence climbing.

        // Operators

        // l_bp: left binding power
        // r_bp: right binding power
        /// Binding powers Table:
        /// Operator   | l_bp | r_bp
        /// ---------------|------|------
        ///  ||            |  1   |  2
        /// !&&            |  3   |  4
        ///  &&            |  3   |  4
        /// (implicit AND) |  3   |  4
        /// ---------------|------|------
        ///
        /// Example:
        /// For input: A && B || C => (A && B) || C
        /// For input: A || B && C => A || (B && C)
        ///
        /// Use explicit binding powers to enforce precedence and associativity.
        /// (A || B ) && C  => (A || B) && C
        fn parse_bp(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
            self.skip_ws();

            // Parse left-hand side primary expression,
            // if it starts with "(" parse a sub-expression until ")"
            let mut lhs = self.parse_primary()?;

            loop {
                self.skip_ws();
                if self.eof() || self.starts_with(")") {
                    break;
                }

                let (op, l_bp, r_bp) = if self.starts_with("||") {
                    (Op::Or, 1, 2)
                } else if self.starts_with("!&&") {
                    (Op::AndNot, 3, 4)
                } else if self.starts_with("&&") {
                    (Op::And, 3, 4)
                } else if self.next_begins_primary() {
                    // implicit AND
                    (Op::And, 3, 4)
                } else {
                    break;
                };

                if l_bp < min_bp {
                    break;
                }

                // consume explicit operator tokens (implicit AND consumes nothing)
                match op {
                    Op::Or => {
                        self.consume("||");
                    }
                    Op::And => {
                        self.consume("&&");
                    }
                    Op::AndNot => {
                        self.consume("!&&");
                    }
                }

                let rhs = self.parse_bp(r_bp)?;
                lhs = match op {
                    Op::And => Expr::and(lhs, rhs),
                    Op::Or => Expr::or(lhs, rhs),
                    Op::AndNot => Expr::AndNot(Box::new(lhs), Box::new(rhs)),
                };
            }

            Ok(simplify(lhs))
        }

        fn parse_primary(&mut self) -> Result<Expr, ParseError> {
            self.skip_ws();
            if self.consume("(") {
                let expr = self.parse_bp(0)?;
                self.skip_ws();
                if !self.consume(")") {
                    return Err(self.err("expected ')'"));
                }
                return Ok(expr);
            }

            self.parse_term()
        }

        fn parse_term(&mut self) -> Result<Expr, ParseError> {
            self.skip_ws();
            if self.eof() {
                return Err(self.err("expected term"));
            }

            // Optional prefix: <ident>:
            let (prefix, value) = if let Some((p, v)) = self.try_parse_prefixed_value()? {
                (p, v)
            } else {
                (Prefix::All, self.parse_value()?)
            };

            if value.is_empty() {
                return Err(self.err("missing term value"));
            }
            Ok(Expr::term(prefix, value))
        }

        fn try_parse_prefixed_value(&mut self) -> Result<Option<(Prefix, String)>, ParseError> {
            let start = self.i;
            let ident = self.parse_ident()?;
            if ident.is_empty() || !self.consume(":") {
                self.i = start;
                return Ok(None);
            }
            let prefix = Prefix::parse(&ident.to_ascii_lowercase());
            let value = self.parse_value()?;
            Ok(Some((prefix, value)))
        }

        fn parse_ident(&mut self) -> Result<String, ParseError> {
            self.skip_ws();
            let start = self.i;
            while let Some(ch) = self.peek_char() {
                if ch.is_ascii_alphabetic() {
                    self.bump_char();
                } else {
                    break;
                }
            }
            Ok(self.input[start..self.i].to_string())
        }

        fn parse_value(&mut self) -> Result<String, ParseError> {
            self.skip_ws();
            match self.peek_char() {
                Some('"') => self.parse_quoted_string(),
                _ => self.parse_bare_word(),
            }
        }

        fn parse_quoted_string(&mut self) -> Result<String, ParseError> {
            if !self.consume("\"") {
                return Err(self.err("expected '\"'"));
            }
            let mut out = String::new();
            while !self.eof() {
                match self.bump_char() {
                    Some('"') => return Ok(out),
                    Some('\\') => {
                        // simple escapes: \" and \\
                        match self.bump_char() {
                            Some('"') => out.push('"'),
                            Some('\\') => out.push('\\'),
                            Some(other) => out.push(other),
                            None => break,
                        }
                    }
                    Some(ch) => out.push(ch),
                    None => break,
                }
            }
            Err(self.err("unterminated string literal"))
        }

        fn parse_bare_word(&mut self) -> Result<String, ParseError> {
            self.skip_ws();
            let start = self.i;
            while !self.eof() {
                if self.starts_with("&&") || self.starts_with("||") || self.starts_with("!&&") {
                    break;
                }
                match self.peek_char() {
                    Some(ch) if ch.is_whitespace() || ch == '(' || ch == ')' || ch == '"' => break,
                    Some(_) => {
                        self.bump_char();
                    }
                    None => break,
                }
            }
            Ok(self.input[start..self.i].to_string())
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Op {
        And,
        Or,
        AndNot,
    }

    // This function is used to expand the Or functions and possibly remove the
    // redundant code. For example for the categories.
    fn terms_only(expr: &Expr) -> Option<Vec<Term>> {
        match expr {
            // Get the input in a vector for later
            Expr::Term(t) => Some(vec![t.clone()]),
            Expr::Or(items) => {
                // If the function only contains terms, extract them
                if items.iter().all(|e| matches!(e, Expr::Term(_))) {
                    Some(
                        items
                            .iter()
                            .map(|e| match e {
                                Expr::Term(t) => t.clone(),
                                _ => unreachable!(), // mostly to use this macro, I tell the compiler this branch cannot be reached
                            })
                            .collect(),
                    )
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Check if the expression has uniform prefixex. Why? If I want to find a paper in
    // all physics, excluded Astro-ph, I can write  ALL_PHYSICS !&& ASTRO-PH and the expansion
    // should be able to ONLY REMOVE the paper in ASTRO-PH category. For this I need a uniform prefix
    fn uniform_prefix(terms: &[Term]) -> Option<Prefix> {
        let first = terms.first()?.prefix;
        if terms.iter().all(|t| t.prefix == first) {
            Some(first)
        } else {
            None
        }
    }

    fn try_or_subract(lhs: Expr, rhs: Expr) -> Option<Expr> {
        use std::collections::HashSet;
        // Only keep lhs_expr if it's an OR statement,
        // If ArXiv supported simple not I could simplify the single term
        // but in this case is better to keep my query slighty more verbose
        // for A !&& B
        let Expr::Or(mut lhs_items) = lhs else {
            return None;
        };

        // RHS must be a term or OR of terms only
        let rhs_terms = terms_only(&rhs)?;
        // The all need to have the same prefix for now, complex mixed operatiors
        // are not included
        let rhs_prefix = uniform_prefix(&rhs_terms)?;
        let remove: HashSet<String> = rhs_terms.into_iter().map(|t| t.value).collect();

        let lhs_terms = terms_only(&Expr::Or(lhs_items.clone()))?;

        let lhs_prefix = uniform_prefix(&lhs_terms)?;
        if lhs_prefix != rhs_prefix {
            return None;
        }
        lhs_items.retain(|it| match it {
            Expr::Term(t) => !(t.prefix == lhs_prefix && remove.contains(&t.value)),
            _ => true,
        });
        let new_lhs = dedup_and_sort(Expr::Or(lhs_items));

        if matches!(new_lhs, Expr::Empty) {
            Some(Expr::Empty)
        } else {
            Some(Expr::AndNot(Box::new(new_lhs), Box::new(rhs)))
        }
    }

    //helper abstract the transformation function of expand category to avoid rewriting
    fn expand_category_or(cat: &[&str]) -> Expr {
        Expr::Or(
            cat.iter()
                .map(|c| Expr::term(Prefix::Cat, c.to_string()))
                .collect(),
        )
    }

    fn expand_category_macro(value: &str) -> Expr {
        let v = value.trim().to_ascii_lowercase().replace(" ", "_");
        match v.as_str() {
            "cs" | "computer_science" => expand_category_or(*arxiv_categories::CS),
            "econ" | "economics" => expand_category_or(*arxiv_categories::ECON),
            "eess"
            | "electrical_engineering_and_systems"
            | "electrical_engineering"
            | "systems"
            | "systems_engineering" => expand_category_or(*arxiv_categories::EESS),
            "math" | "mathematics" => expand_category_or(*arxiv_categories::MATH),
            "astro-ph" | "astro" | "astrophysics" => {
                expand_category_or(*arxiv_categories::ASTRO_PH)
            }
            "cond-mat" | "condensed_matter" => expand_category_or(*arxiv_categories::COND_MAT),
            "gr-qc"
            | "general_relativity_and_quantum_cosmology"
            | "general_relativity"
            | "quantum_cosmology" => expand_category_or(*arxiv_categories::GR_QC),
            "hep" | "high_energy_physics" | "high_energy" => {
                expand_category_or(*arxiv_categories::HEP)
            }
            "math-ph" | "mathematical_physics" => expand_category_or(*arxiv_categories::MATH_PH),
            "nlin" | "nonlinear_sciences" | "nonlinear" => {
                expand_category_or(*arxiv_categories::NLIN)
            }
            "nucl" | "nuclear" | "nuclear_physics" => expand_category_or(*arxiv_categories::NUCL),
            "physics" | "physical_sciences" => expand_category_or(*arxiv_categories::PHYSICS),
            "quant-ph" | "quantum_physics" => expand_category_or(*arxiv_categories::QUANT_PH),
            "q-bio" | "quantitative_biology" => expand_category_or(*arxiv_categories::Q_BIO),
            "q-fin" | "quantitative_finance" => expand_category_or(*arxiv_categories::Q_FIN),
            "stat" | "statistics" => expand_category_or(*arxiv_categories::STAT),
            "all_physics" | "all-physics" | "physics_ext" => {
                expand_category_or(*arxiv_categories::PHYSICS_EXT)
            }
            _ => {
                if arxiv_categories::ALL_CATEGORIES
                    .iter()
                    .any(|c| c.eq_ignore_ascii_case(&value))
                {
                    Expr::term(Prefix::Cat, value.to_string())
                } else {
                    Expr::Empty
                }
            }
        }
    }

    fn simplify(expr: Expr) -> Expr {
        match expr {
            Expr::Empty => Expr::Empty,
            // Term returns as is
            Expr::Term(t) => {
                if t.value.is_empty() {
                    return Expr::Empty;
                }
                match t.prefix {
                    Prefix::Cat => expand_category_macro(&t.value),
                    _ => Expr::Term(t),
                }
            }
            Expr::And(items) => {
                let mut flat = Vec::new();
                for item in items.into_iter().map(simplify) {
                    match item {
                        Expr::Empty => {}
                        // Make And nodes flat by extending children
                        Expr::And(more) => flat.extend(more),
                        other => flat.push(other),
                    }
                }
                dedup_and_sort(Expr::And(flat))
            }
            Expr::Or(items) => {
                let mut flat = Vec::new();
                for item in items.into_iter().map(simplify) {
                    match item {
                        Expr::Empty => {}
                        // Make Or nodes flat by extending children
                        Expr::Or(more) => flat.extend(more),
                        other => flat.push(other),
                    }
                }
                dedup_and_sort(Expr::Or(flat))
            }
            Expr::AndNot(a, b) => {
                let left = simplify(*a);
                let right = simplify(*b);

                // Special Cases
                match (&left, &right) {
                    // 0 !&& X  => 0
                    (&Expr::Empty, _) => return Expr::Empty,
                    // X !&& 0  => X
                    (_, &Expr::Empty) => return left,
                    _ => {}
                }

                // Is it reducible?
                if let Some(rewritten) = try_or_subract(left.clone(), right.clone()) {
                    return rewritten;
                }

                Expr::AndNot(Box::new(left), Box::new(right))
            }
        }
    }

    fn dedup_and_sort(expr: Expr) -> Expr {
        use std::collections::HashSet;

        //extract items and whether it's an And or Or
        let (is_and, mut items) = match expr {
            Expr::And(items) => (true, items),
            Expr::Or(items) => (false, items),
            //only simplify And/Or nodes, return others as is
            other => return other,
        };

        //Remove Empty from Expression
        items.retain(|e| !matches!(e, Expr::Empty));

        if items.is_empty() {
            return Expr::Empty;
        };

        items.sort_by(|a, b| canonical_key(a).cmp(&canonical_key(b)));

        let mut seen = HashSet::new();
        items.retain(|item| seen.insert(canonical_key(item)));

        if items.len() == 1 {
            return items.into_iter().next().unwrap();
        }
        if is_and {
            Expr::And(items)
        } else {
            Expr::Or(items)
        }
    }

    fn canonical_key(expr: &Expr) -> String {
        // This intentionally matches the serializer’s structure; good enough for dedup/canonical order.
        super::interpret::to_arxiv_string(expr)
    }
}

pub mod interpret {

    use super::ast::{Expr, Term};
    use super::visit::Visitor;

    pub struct ArxivQueryInterpreter;

    impl Visitor for ArxivQueryInterpreter {
        type Output = String;

        fn visit_term(&mut self, term: &Term) -> Self::Output {
            // arXiv expects quoted terms for safety (spaces, punctuation).
            if term.value.is_empty() {
                return String::new();
            }
            format!(r#"{}"{}""#, term.prefix.as_str(), term.value)
        }

        fn visit_expr(&mut self, expr: &Expr) -> Self::Output {
            match expr {
                Expr::Empty => String::new(),
                Expr::Term(term) => self.visit_term(term),
                Expr::And(items) => join_infix(items, "+AND+", self),
                Expr::Or(items) => join_infix(items, "+OR+", self),
                Expr::AndNot(a, b) => {
                    let left = wrap_if_needed(a, self);
                    let right = wrap_if_needed(b, self);

                    // Make ANDNOT consistent with join_infix "skip empties":
                    // 0 !&& X  => 0 (empty string)
                    if left.is_empty() {
                        return String::new();
                    }
                    // X !&& 0  => X
                    if right.is_empty() {
                        return left;
                    }

                    format!("{}+ANDNOT+{}", left, right)
                }
            }
        }
    }

    fn wrap_if_needed(expr: &Expr, v: &mut impl Visitor<Output = String>) -> String {
        match expr {
            Expr::Term(_) => v.visit_expr(expr),
            Expr::Empty => String::new(),
            _ => {
                let inner = v.visit_expr(expr);
                if inner.is_empty() {
                    String::new()
                } else {
                    format!("({})", inner)
                }
            }
        }
    }

    fn join_infix(items: &[Expr], op: &str, v: &mut impl Visitor<Output = String>) -> String {
        let mut out = String::new();
        let mut first = true;

        for item in items {
            let rendered = wrap_if_needed(item, v);
            // If empty it skips the operation
            if rendered.is_empty() {
                continue;
            }
            // add operation only after first term so
            // if I had a+b -> b
            if !first {
                out.push_str(op);
            }
            // negate first after first use
            first = false;
            out.push_str(&rendered);
        }
        out
    }

    pub fn to_arxiv_string(expr: &Expr) -> String {
        ArxivQueryInterpreter.visit_expr(expr)
    }

    /// If the expression serializes longer than `max_chars`, this tries to split a top-level OR
    /// into multiple OR queries. Otherwise returns a single query.
    pub fn to_arxiv_queries_with_limit(expr: &Expr, max_chars: usize) -> Vec<String> {
        let s = to_arxiv_string(expr);
        if s.len() <= max_chars {
            return vec![s];
        }

        match expr {
            Expr::Or(items) => {
                let mut out = Vec::new();
                let mut chunk: Vec<Expr> = Vec::new();
                for item in items {
                    chunk.push(item.clone());
                    let candidate = to_arxiv_string(&Expr::Or(chunk.clone()));
                    if candidate.len() > max_chars && chunk.len() > 1 {
                        // flush without the last item
                        let last = chunk.pop().unwrap();
                        out.push(to_arxiv_string(&Expr::Or(chunk.clone())));
                        chunk.clear();
                        chunk.push(last);
                    }
                }
                if !chunk.is_empty() {
                    out.push(to_arxiv_string(&Expr::Or(chunk)));
                }
                out
            }
            _ => vec![s],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{interpret, parse};
    use crate::services::provider::constants::arxiv_categories;

    use super::ast::Expr;
    #[test]
    fn parses_and_serializes_basic() {
        let expr = parse::parse_expr(r#"ti:"Quantum Mechanics" && au:"John Doe""#).unwrap();
        let s = interpret::to_arxiv_string(&expr);
        assert!(s.contains(r#"ti:"Quantum Mechanics""#));
        assert!(s.contains(r#"au:"John Doe""#));

        let re = fancy_regex::Regex::new(r#"^(ti:"Quantum Mechanics"\+AND\+au:"John Doe"|au:"John Doe"\+AND\+ti:"Quantum Mechanics")$"#).unwrap();
        assert!(re.is_match(&s).unwrap());
    }

    #[test]
    fn implicit_and_and_parentheses() {
        let expr = parse::parse_expr(r#"(ti:"qft" || ti:"relativity") au:"Einstein""#).unwrap();
        let s = interpret::to_arxiv_string(&expr);

        assert!(s.contains(r#"(ti:"qft"+OR+ti:"relativity")"#));
        assert!(s.contains(r#"au:"Einstein""#));

        // Regex to match 2 parts connected by +AND+
        // the two parts can be in any order
        // so both of these are acceptable:
        // (ti:"qft"+OR+ti:"relativity")+AND+au:"Einstein"
        // au:"Einstein"+AND+(ti:"qft"+OR+ti:"relativity")

        let re = fancy_regex::Regex::new(r#"^(?:\(ti:"qft"\+OR\+ti:"relativity"\)\+AND\+au:"Einstein"|au:"Einstein"\+AND\+\(ti:"qft"\+OR\+ti:"relativity"\))$"#).unwrap();
        assert!(re.is_match(&s).unwrap());
    }

    #[test]
    fn andnot_operator() {
        let expr = parse::parse_expr(r#"ti:"graph" !&& ti:"neural""#).unwrap();
        let s = interpret::to_arxiv_string(&expr);
        assert_eq!(s, r#"ti:"graph"+ANDNOT+ti:"neural""#);
    }

    #[test]
    fn complex_expression() {
        let expr = parse::parse_expr(
            r#"(ti:"quantum" || ti:"mechanics") && (au:"Einstein" !&& au:"Bohr")"#,
        )
        .unwrap();
        let s = interpret::to_arxiv_string(&expr);

        // Expected structure:
        // (ti:"quantum"+OR+ti:"mechanics")+AND+(au:"Einstein"+ANDNOT+au:"Bohr")

        println!("Serialized: {}", s);

        assert!(s.contains(r#"ti:"quantum""#));
        assert!(s.contains(r#"ti:"mechanics""#));
        let re = fancy_regex::Regex::new(
            r#"(?:\(ti:"quantum"\+OR\+ti:"mechanics"\)|\(ti:"mechanics"\+OR\+ti:"quantum"\))"#,
        )
        .unwrap();
        assert!(re.is_match(&s).unwrap());
        assert!(s.contains(r#"(au:"Einstein"+ANDNOT+au:"Bohr")"#));

        let re = fancy_regex::Regex::new(r#"^(?:\(.*\)\+AND\+\(au:"Einstein"\+ANDNOT\+au:"Bohr"\)|\(au:"Einstein"\+ANDNOT\+au:"Bohr"\)\+AND\+\(.*\))$"#).unwrap();

        assert!(re.is_match(&s).unwrap());
    }

    #[test]
    fn test_repetition_deduplication() {
        let expr =
            parse::parse_expr(r#"ti:"quantum" && ti:"quantum" || au:"Einstein" && au:"Einstein""#)
                .unwrap();
        let s = interpret::to_arxiv_string(&expr);

        // Expected structure after deduplication:
        // (ti:"quantum")+OR+(au:"Einstein")
        assert!(s.contains(r#"ti:"quantum""#));
        assert!(s.contains(r#"au:"Einstein""#));

        let re = fancy_regex::Regex::new(
            r#"^(?:ti:"quantum"\+OR\+au:"Einstein"|au:"Einstein"\+OR\+ti:"quantum")$"#,
        )
        .unwrap();
        println!("Serialized: {}", s);
        assert!(re.is_match(&s).unwrap());
    }

    #[test]
    fn test_default_expr() {
        let default_expr = Expr::default();

        assert_eq!(default_expr, Expr::Empty);
    }

    #[test]
    fn test_default_expr_is_empty() {
        let default_expr = Expr::default();
        assert!(matches!(default_expr, Expr::Empty));
    }

    #[test]
    fn text_empty_parses_error() {
        let result = parse::parse_expr(r#" "#);
        assert!(result.is_err(), "Expected error for empty input");
        let result = parse::parse_expr(r#"all:  "#);
        assert!(result.is_err(), "Expected error for empty input");
    }

    #[test]
    fn text_expand_category_macro_only() {
        let expr = parse::parse_expr(r#"cat:"cs""#).unwrap();
        let s = interpret::to_arxiv_string(&expr);

        for c in arxiv_categories::CS.iter() {
            assert!(s.contains(&format!(r#"cat:"{}""#, c)));
        }
    }

    #[test]
    fn cat_macro_andnot_removes_leaf() {
        let expr = parse::parse_expr(r#"cat:"cs" !&& cat:"cs.SY""#).unwrap();
        let s = interpret::to_arxiv_string(&expr);

        for c in arxiv_categories::CS.iter() {
            assert!(
                s.contains(&format!(r#"cat:"{}""#, c)),
                "Expected to contain cat:\"{}\"",
                c
            );
        }
        // removed

        // contains only 1 cs.SY and the only one is +ANDNOT+cat:"cs.SY"
        assert_eq!(
            s.matches(r#"cat:"cs.SY""#).count(),
            1,
            "cat:cs.SY should appear only once. got: {s}"
        );

        assert!(s.split_once("+ANDNOT+").is_some());

        let (left, right) = s.split_once("+ANDNOT+").unwrap();

        assert!(
            !left.contains(r#"cat:"cs.SY""#),
            r#"Expected no cat:"cs.SY" in the LHS"#
        );
        assert!(
            right.contains(r#"cat:"cs.SY""#),
            r#"Expected cat:"cs.SY" in the RHS"#
        );
    }

    #[test]
    fn test_removes_leaf() {
        let expr =
            parse::parse_expr(r#"(ti:"quantum" || ti:"mechanics") !&& ti:"mechanics""#).unwrap();
        let s = interpret::to_arxiv_string(&expr);

        assert!(
            s.contains(r#"ti:"quantum""#),
            "Expected to contain ti:\"quantum\""
        );
        assert!(
            s.contains(r#"ti:"mechanics""#),
            "Expected to contain ti:\"mechanics\""
        );

        // contains only 1 ti:"mechanics" and the only one is +ANDNOT+ti:"mechanics"
        assert_eq!(
            s.matches(r#"ti:"mechanics""#).count(),
            1,
            "ti:\"mechanics\" should appear only once. got: {s}"
        );
        assert!(s.split_once("+ANDNOT+").is_some());
        let (left, right) = s.split_once("+ANDNOT+").unwrap();
        assert!(
            !left.contains(r#"ti:"mechanics""#),
            r#"Expected no ti:"mechanics" in the LHS"#
        );
        assert!(
            right.contains(r#"ti:"mechanics""#),
            r#"Expected ti:"mechanics" in the RHS"#
        );
    }

    #[test]
    fn test_to_arxiv_queries_with_limit() {
        let expr =
            parse::parse_expr(r#"cat:"cs" || cat:"math" || cat:"physics" || cat:"econ""#).unwrap();
        let queries = interpret::to_arxiv_queries_with_limit(&expr, 100);
        assert!(queries.len() > 1, "Expected multiple queries due to limit");

        for query in queries {
            println!("Query: {}", query);
            assert!(
                query.len() <= 100,
                "Each query should respect the character limit"
            );
        }
    }
}
