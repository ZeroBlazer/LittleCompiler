use pest;
use Rule;

pub fn validate_rustlin(pairs: pest::iterators::Pairs<Rule, pest::inputs::StrInput<'_>>) -> Result<(), String> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                println!(">>>>>>>>>>>>> {:?}", pair);
            }
            _ => {
                println!("{:?}", pair);
            }
        }
    }

    Ok(())
}