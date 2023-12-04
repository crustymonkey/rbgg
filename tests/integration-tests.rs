use rbgg::{
    bgg1::Client1,
    bgg2::*,
    utils::Params
};
use serde_json::to_string_pretty;

#[test]
fn test_search() {
    // API v1
    let cl = Client1::new_from_defaults();
    let params = Params::from([
        ("exact".into(), "1".into()),
    ]);
    let res = cl.search_b("bruges", Some(params));

    assert!(res.is_ok());
    let data = res.unwrap();
    println!("{}", to_string_pretty(&data).unwrap());
    assert_eq!("Bruges", &data["boardgames"]["boardgame"]["name"]["#text"]);

    // API v2
    let cl = Client2::new_from_defaults();
    let params = Params::from([
        ("exact".into(), "1".into()),
    ]);
    
    let res = cl.search_b("bruges", &vec![Search::BoardGame], Some(params));

    assert!(res.is_ok());

    let data = res.unwrap();
    println!("{}", to_string_pretty(&data).unwrap());
    assert_eq!("Bruges", data["items"]["item"]["name"]["@value"]);
}
