use itertools::Itertools;

#[derive(Clone)]
pub enum Pattern {
    String(String),
    Edge(String),
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

#[derive(Clone)]
pub struct Filter {
    pairs: Vec<(String, Pattern)>,
}

/*
struct StackEntry<'a> {
    op :
    iter: Box<dyn Iterator<Item = String> + 'a>,
}
 */

fn compile_query_step<'a>(
    db: &'a [(&'a str, &'a str, &'a str)],
    query: Filter,
    zi: Box<dyn Iterator<Item = String> + 'a>,
) -> (Option<Filter>, Box<dyn Iterator<Item = String> + 'a>) {
    let pairs = query.pairs;
    if !pairs.is_empty() {
        let (k, v) = pairs[0].clone();
        let next_dict = if pairs.len() > 1 {
            Some(Filter {
                pairs: pairs[1..].to_vec(),
            })
        } else {
            None
        };
        println!("Filtering on k: {k}");
        match v {
            Pattern::String(value) => (
                next_dict,
                Box::new(zi.filter(move |subject| {
                    db.iter()
                        .any(|t| t.0 == subject && t.1 == k && t.2 == value)
                })),
            ),
            _ => todo!(),
        }
    } else {
        (None, zi)
    }
}

fn compile_query<'a>(
    db: &'a [(&'a str, &'a str, &'a str)],
    zi: Box<dyn Iterator<Item = String> + 'a>,
    query: Filter,
) -> Box<dyn Iterator<Item = String> + 'a> {
    let (mut next_query, mut next_iterator) = compile_query_step(db, query, zi);
    while next_query.is_some() {
        let query = next_query.unwrap();
        (next_query, next_iterator) = compile_query_step(db, query, next_iterator);
    }
    next_iterator
}

fn run_query<'a>(db: &'a [(&'a str, &'a str, &'a str)], query: Filter) -> Vec<String> {
    let zi = Box::new(db.iter().map(|t| t.0.to_string()).dedup());
    compile_query(db, zi, query).collect()
}

fn main() {
    let db = vec![
        ("Joe", "type", "Person"),
        ("Joe", "name", "Joe Bloggs"),
        ("Joe", "friend", "Jim"),
        ("Joe", "dob", "2012-01-01"),
        ("Jim", "type", "Person"),
        ("Jim", "name", "Jim-Bob McGee"),
        ("Jim", "friend", "Jim"),
        ("Jim", "friend", "Joe"),
    ];
    let query = Filter {
        pairs: vec![("friend".to_string(), Pattern::String("Joe".to_string()))],
    };
    let results = run_query(&db, query);
    println!("{results:?}");
}
