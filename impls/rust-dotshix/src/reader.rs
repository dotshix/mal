use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "mal.pest"]
struct MalParser;

#[derive(Debug,Clone)]
pub enum MalValue {
    String(String),
    Comment(String),
    NonSpecialSeq(String),
    List(Vec<MalValue>),
    Other(String),
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
    //println!("Processing rule: {:?}", pair.as_rule());
    // Debugging information

    match pair.as_rule() {
        Rule::STRING => {
            let content = pair.into_inner().map(|p| p.as_str()).collect::<Vec<_>>().join("");
            MalValue::String(content)
        },
        Rule::COMMENT => {
            let content = pair.as_str().to_string();
            MalValue::Comment(content)
        },
        Rule::NON_SPECIAL_SEQ => {
            let content = pair.as_str().to_string();
            MalValue::NonSpecialSeq(content)
        },
        Rule::list => {
            let elements = pair.into_inner().map(build_ast).collect::<Vec<_>>();
            MalValue::List(elements)
        },
        Rule::mal => {
            // let content = pair.into_inner().map(build_ast).collect::<Vec<_>>();
            // MalValue::Other(format!("{:?}", content))
            let content = pair.into_inner().map(|p| p.as_str().trim()).collect::<Vec<_>>().join(" ");
            MalValue::Other(content)
        },
        Rule::EOI => MalValue::Other("EOI".to_string()), // Handling End Of Input
        _ => panic!("Unexpected rule: {:?}", pair.as_rule()),
    }
}
