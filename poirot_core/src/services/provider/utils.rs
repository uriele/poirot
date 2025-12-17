
use super::constants::{arxiv_text};
use std::sync::OnceLock;
use fancy_regex::Regex;
use crate::domain::errors::QueryError;

// TODO : implement proper parsing of arXiv query body
// I want to find && as +AND+all: and || as +OR+all: all the other text should be wrapped in \" "
// it can have prefixes like ti:, au:, abs:, if no prefix is specified, all: is assumed
pub fn parse_arxiv_query_body(query_body: &str) -> Result<String,QueryError> {
    // Grammar (testing):
    // -`&&` becomes +AND+
    // - Terms may have prefixes: ti:, au:, abs:, all:
    // - If no prefix is present, all: is assumed
    // - Term values are wrapped in quotes: prefix:"value"
    //
    // Notes:
    // - This does not implement parentheses or NOT.
    // - Whitespace is treated as part of a term value unless it separates operators.

    // Validate that the format of the query is correct: balanced double quotes
    fn validate_balanced_double_quotes(s: &str) -> Result<(),QueryError> {
        // Remove escaped quotes \" then count remaining ".
        static RE_ESCAPED: OnceLock<Regex> = OnceLock::new();
        let re_escaped = RE_ESCAPED.get_or_init(|| Regex::new(r#"\\"|\\\""#).expect("regex"));
        // NOTE: this is conservative; it treats \" as escaped quote and also strips \\ sequences.
        let cleaned = re_escaped.replace_all(s, "").to_string();
        if cleaned.chars().filter(|&c| c == '"').count() % 2 != 0 {
            return Err(QueryError::WrongFormatting("Unbalanced double quotes in query".to_string()));
        }
        Ok(())
    }

    fn validate_formatted_prefixes(s: &str) -> Result<(),QueryError> {
        // Prefix should be followed by text or non empty quoted text
        // Invalid:
        // - prefix followed by operator: "ti: && foo"
        // - prefix at end of string: "au:" "au:   "
        // - prefix followed by closing parenthesis: "ti: )"
        // Note: the absence of prefix is also "all:" so we need to implement it some way maybe before passing it to
        // the validator
        static RE_DANGLING_PREFIX: OnceLock<Regex> = OnceLock::new();
        let re_dangling = RE_DANGLING_PREFIX.get_or_init(|| {
            Regex::new(r#"(?x)
            (?<!\w)(?:ti:|au:|abs:|all:)
            \s*(
                $
                | \)
                | !&& | && | \|\|
            )   "#).expect("regex")
        });
        if re_dangling.is_match(s).unwrap_or(false) {
            return Err(QueryError::WrongFormatting("Dangling prefix in query".to_string()));
        }
        Ok(())
    }


    fn validate_bad_operators(s: &str) -> Result<(),QueryError> {
        // Invalid sequences:
        // - two operators in a row: && &&, || ||
        // - operator at start or end of string

        static RE_BAD_OPS: OnceLock<Regex> = OnceLock::new();
        let re_bad = RE_BAD_OPS.get_or_init(|| {
            Regex::new(r#"(?x)
                (?: ^\s*(?:!&&|&&|\|\|) )                      # operator at start
              | (?: (?:!&&|&&|\|\|)\s*$ )                      # operator at end
              | (?: (?:!&&|&&|\|\|)\s*(?:!&&|&&|\|\|) )        # two operators in a row
              | (?: \(\s*(?:!&&|&&|\|\|) )                     # '(' then operator
              | (?: (?:!&&|&&|\|\|)\s*\) )                     # operator then ')'
            "#).expect("regex")
        });
        
        if re_bad.is_match(s).unwrap_or(false) {
            return Err(QueryError::WrongFormatting(
                "Operator missing term (at start/end, next to another operator, or adjacent to parentheses)".to_string(),
            ));
        }
        Ok(())
    }


    let _ = validate_balanced_double_quotes(query_body)?;
    let _ = validate_formatted_prefixes(query_body)?;
    let _ = validate_bad_operators(query_body)?;



    fn normalize_prefix(p:&str) -> &'static str {
        match p {
            "ti:" => arxiv_text::TITLE,
            "au:" => arxiv_text::AUTHOR,
            "abs:" => arxiv_text::ABSTRACT,
            "all:" => arxiv_text::ALL,
            _ => arxiv_text::ALL,
        }
    }

    
    #[derive(Copy, Clone, Debug)]
    enum Op {
        And,
        Or,
        AndNot,
    }

    fn op_str(op: Op) -> &'static str {
        match op {
            Op::And => arxiv_text::AND,
            Op::Or => arxiv_text::OR,
            Op::AndNot => arxiv_text::ANDNOT,   
        }
    }

    // This is used to allow the quotes inside values to be escaped
    fn escape_quotes(s: &str) -> String {
        s.replace('"', "\\\"")
    }

    fn normalize_ws(s: &str) -> String {
        static RE_WS: OnceLock<Regex> = OnceLock::new();
        let re = RE_WS.get_or_init(|| Regex::new(r"\s+").expect("regex"));
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        re.replace_all(trimmed, " ").to_string()
    }

    fn strip_wrapping_quotes(s: &str) -> &str {
        let t = s.trim();
        if t.len() < 2 {
            return t;
        }

        let bytes = t.as_bytes();
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];

        // I do not care about ' ' it is not part of my parser
        let is_pair = first == b'"' && last == b'"';
        if !is_pair {
            return t;
        }
        // Safe because we only slice at UTF-8 boundaries for ASCII quotes.
        &t[1..t.len() - 1]
    }


    // MAIN LOGIC 
    static RE_TOKENS: OnceLock<Regex> = OnceLock::new();
    let re = RE_TOKENS.get_or_init(|| {
        Regex::new(
            r#"(?x)
                (!&&|&&|\|\|)                 # 1 operator
              | (?<!\w)(ti:|au:|abs:|all:)      # 2 prefix
              | ("(?:\\.|[^"\\])*")           # 3 double-quoted string
              | ([^\s]+)                      # 4 bare token
            "#,
        )
        .expect("regex")
    });

    

    let mut out = String::new();
    let mut pending_op: Option<Op> = None;
    let mut current_prefix: Option<&'static str> = None;
    let mut value_parts: Vec<&str> = Vec::new();


    #[allow(unused_mut)]
    let mut flush_value = |out: &mut String,
                           pending_op: &mut Option<Op>,
                           current_prefix: &mut Option<&'static str>,
                           parts: &mut Vec<&str>| {
        if parts.is_empty() {
            return;
        }

        let joined = parts.join(" ");
        parts.clear();

        let p = current_prefix.take().unwrap_or(arxiv_text::ALL);
        let v = strip_wrapping_quotes(&joined);
        let v = normalize_ws(v);
        if v.is_empty() {
            return;
        }
        let v = escape_quotes(&v);

        if !out.is_empty() {
            out.push_str(op_str(pending_op.take().unwrap_or(Op::And)));
        } else {
            pending_op.take();
        }

        out.push_str(p);
        out.push('"');
        out.push_str(&v);
        out.push('"');
    };
    
    
    for m in re.find_iter(query_body) {
        let m = m.map_err(|e| QueryError::UnexpectedError(format!("Regex tokenization error: {e}")))?;
        let tok = m.as_str();

        match tok {
            "!&&" => {
                flush_value(&mut out, &mut pending_op, &mut current_prefix, &mut value_parts);
                pending_op = Some(Op::AndNot);
            }
            "&&" => {
                flush_value(&mut out, &mut pending_op, &mut current_prefix, &mut value_parts);
                pending_op = Some(Op::And);
            }
            "||" => {
                flush_value(&mut out, &mut pending_op, &mut current_prefix, &mut value_parts);
                pending_op = Some(Op::Or);
            }
            "ti:" | "au:" | "abs:" | "all:" => {
                // A prefix starts a new term; flush any accumulated value first.
                flush_value(&mut out, &mut pending_op, &mut current_prefix, &mut value_parts);
                current_prefix = Some(normalize_prefix(tok));
            }
            _ => {
                // value token (quoted or bare)
                value_parts.push(tok);
            }
        }
    }

    flush_value(&mut out, &mut pending_op, &mut current_prefix, &mut value_parts);

    Ok(out)
}



pub fn arxiv_category_to_text(category: &arxiv_text::Categories) -> &'static str{
    match category{
        arxiv_text::Categories::ComputerScience(cs_cat) => arxiv_text::computer_science(cs_cat),
        arxiv_text::Categories::Economics(econ_cat) => arxiv_text::economics(econ_cat),
        arxiv_text::Categories::ElectricalEngineeringAndSystemsScience(eess_cat) => arxiv_text::eess(eess_cat),
        arxiv_text::Categories::Mathematics(math_cat) => arxiv_text::mathematics(math_cat),
        arxiv_text::Categories::Physics(phys_cat) => arxiv_text::physics_category(phys_cat),
        arxiv_text::Categories::QuantitativeBiology(qbio_cat) => arxiv_text::quantitative_biology(qbio_cat),
        arxiv_text::Categories::QuantitativeFinance(qfin_cat) => arxiv_text::quantitative_finance(qfin_cat),
        arxiv_text::Categories::Statistics(stat_cat) => arxiv_text::statistics(stat_cat),
    }
}





#[cfg(test)]
mod tests {
    use super::super::constants::arxiv_text::*;
    use super::*;

    #[test]
    fn test_all_arxiv_category() {
        let cs_cat = ComputerScience::AI;
        let econ_cat = Economics::EM;
        let eess_cat = ElectricalEngineeringAndSystemsScience::AS;
        let math_cat = Mathematics::AC;
        let phys_cat = PhysicsCategory::Astrophysics(Astrophysics::CO);
        let qbio_cat = QuantitativeBiology::BM;
        let qfin_cat = QuantitativeFinance::CP;
        let stat_cat = Statistics::AP;

        let categories = vec![
            Categories::ComputerScience(cs_cat),
            Categories::Economics(econ_cat),
            Categories::ElectricalEngineeringAndSystemsScience(eess_cat),
            Categories::Mathematics(math_cat),
            Categories::Physics(phys_cat),
            Categories::QuantitativeBiology(qbio_cat),
            Categories::QuantitativeFinance(qfin_cat),
            Categories::Statistics(stat_cat),
        ];

        let expected_strings = vec![
            "cs.AI",
            "econ.EM",
            "eess.AS",
            "math.AC",
            "astro-ph.CO",
            "q-bio.BM",
            "q-fin.CP",
            "stat.AP",
        ];

        for (category, expected) in categories.iter().zip(expected_strings.iter()) {
            assert_eq!(arxiv_category_to_text(category), *expected);
        }
        
        
    }
}








#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_parse_query_body(){
        let query="ti:Quantum   Mechanics && au:\"Albert Einstein\" || abs: relativity";
        let parsed=parse_arxiv_query_body(query);
        let correct=r#"ti:"Quantum Mechanics"+AND+au:"Albert Einstein"+OR+abs:"relativity""#;
        assert_eq!(parsed.unwrap(),correct.to_string());
    }

    #[test]
    fn test_parse_query_body_no_prefix(){
        let query="Quantum   Mechanics && au: Albert Einstein || relativity";
        let parsed=parse_arxiv_query_body(query);
        let correct=r#"all:"Quantum Mechanics"+AND+au:"Albert Einstein"+OR+all:"relativity""#;
        assert_eq!(parsed.unwrap(),correct.to_string());
    }

    #[test]
    fn test_parse_query_body_mixed(){
        let query="ti:Quantum   Mechanics && \"Albert Einstein\" || abs: relativity";
        let parsed=parse_arxiv_query_body(query);
        let correct=r#"ti:"Quantum Mechanics"+AND+all:"Albert Einstein"+OR+abs:"relativity""#;
        assert_eq!(parsed.unwrap(),correct.to_string());
    }

    #[test]
    fn test_parse_for_empty_body(){
        let query="";
        let parsed=parse_arxiv_query_body(query);
        let correct=r#""#;
        assert_eq!(parsed.unwrap(),correct.to_string());
    }
}