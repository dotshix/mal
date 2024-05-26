use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mal.pest"]
struct MalParser;

#[derive(Debug, Clone)]
pub enum MalValue {
    String(String),
    Comment(String),
    NonSpecialSeq(String),
    List(Vec<MalValue>),
    Mal(Vec<MalValue>),
    Other(String),
    EOI,
}

pub fn parse_input(input: &str) -> Result<Vec<MalValue>, Error<Rule>> {
    let pairs = MalParser::parse(Rule::mal, input)?;
    let mut ast = Vec::new();

    for pair in pairs {
        let node = build_ast(pair);
        ast.push(node);
    }

    Ok(ast)
}

fn build_ast(pair: Pair<Rule>) -> MalValue {
    // println!("Processing rule: {:?}", pair.as_rule());
    // println!("Pair content: {:?}", pair.as_str());

    match pair.as_rule() {
        Rule::STRING => {
            let content = pair.into_inner().map(|p| p.as_str()).collect::<Vec<_>>().join("");
            // println!("STRING content: {:?}", content);
            MalValue::String(content)
        },
        Rule::COMMENT => {
            let content = pair.as_str().to_string();
            // println!("COMMENT content: {:?}", content);
            MalValue::Comment(content)
        },
        Rule::NON_SPECIAL_SEQ => {
            let content = pair.as_str().to_string();
            // println!("NON_SPECIAL_SEQ content: {:?}", content);
            MalValue::NonSpecialSeq(content)
        },
        Rule::list => {
            let elements = pair.into_inner()
                .filter(|p| {
                    let is_empty = p.as_str().trim().is_empty();
                    // println!("Checking element in list: {:?}, is_empty: {:?}", p.as_str(), is_empty);
                    !is_empty
                })
                .map(build_ast)
                .collect::<Vec<_>>();
            // println!("List elements: {:?}", elements);
            MalValue::List(elements)
        },
        Rule::expr => {
            let mut inner_pairs = pair.into_inner();
            // println!("Processing expr with inner pairs: {:?}", inner_pairs.clone().collect::<Vec<_>>());
            if let Some(single_pair) = inner_pairs.next() {
                if inner_pairs.clone().count() == 0 {
                    // println!("Single expr: {:?}", single_pair);
                    build_ast(single_pair)
                } else {
                    let content = std::iter::once(single_pair)
                        .chain(inner_pairs)
                        .map(build_ast)
                        .collect::<Vec<_>>();
                    // println!("Multiple expr content: {:?}", content);
                    MalValue::List(content)
                }
            } else {
                // println!("Empty expr");
                MalValue::Other(String::new())
            }
        },
        Rule::mal => {
            let content = pair.into_inner().map(build_ast).collect::<Vec<_>>();
            // println!("Mal content: {:?}", content);
            if content.len() == 1 {
                content.into_iter().next().unwrap()
            } else {
                MalValue::Mal(content)
            }
        },
        Rule::EOI => {
            // println!("EOI encountered");
            MalValue::EOI
        },
        _ => {
            // println!("Unexpected rule encountered: {:?}", pair.as_rule());
            panic!("Unexpected rule: {:?}", pair.as_rule());
        },
    }
}
