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

#[test]
fn test_boardgame() {
    // API v1
    let cl = Client1::new_from_defaults();
    let params = Params::from([
        ("stats".into(), "1".into()),
    ]);
    let res = cl.boardgame_b(&vec![136888, 133473], Some(params));
    
    assert!(res.is_ok());
    let data = res.unwrap();
    for game in data["boardgames"]["boardgame"].as_array().unwrap() {
        if game["@objectid"] != "136888" && game["@objectid"] != "133473" {
            panic!();
        }
    }
    println!("{}", to_string_pretty(&data["boardgames"]["boardgame"]).unwrap());

    // API v2
    let cl = Client2::new_from_defaults();
    let params = Params::from([
        ("stats".into(), "1".into()),
    ]);
    let res = cl.boardgame_b(&vec![136888, 133473], Some(params));

    assert!(res.is_ok());
    let data = res.unwrap();
    println!("{}", to_string_pretty(&data).unwrap());
    for game in data["items"]["item"].as_array().unwrap() {
        if game["@id"] != "136888" && game["@id"] != "133473" {
            panic!();
        }
    }
    println!("{}", to_string_pretty(&data["items"]["item"]).unwrap());
}
