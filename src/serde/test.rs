use serde_test::assert_tokens;
use serde_test::Token;

use crate::panicking::vec::map::VecMap;
use crate::panicking::vec::set::VecSet;
use crate::InfallibleLinearMap;
use crate::InfallibleLinearSet;
#[test]
fn vec_map_many() {
    let mut map: VecMap<char, i32> = VecMap::new();
    map.insert('b', 20);
    map.insert('a', 10);
    map.insert('c', 30);

    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(3) },
            Token::Char('b'),
            Token::I32(20),
            Token::Char('a'),
            Token::I32(10),
            Token::Char('c'),
            Token::I32(30),
            Token::MapEnd,
        ],
    );
}

#[test]
fn vec_map_empty() {
    let map: VecMap<char, i32> = VecMap::new();

    assert_tokens(&map, &[Token::Map { len: Some(0) }, Token::MapEnd]);
}

#[test]
fn vec_set_many() {
    let mut set: VecSet<char> = VecSet::new();

    set.insert('b');
    set.insert('a');
    set.insert('c');

    assert_tokens(
        &set,
        &[
            Token::Seq { len: Some(3) },
            Token::Char('b'),
            Token::Char('a'),
            Token::Char('c'),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn vec_set_empty() {
    let set: VecSet<char> = VecSet::new();

    assert_tokens(&set, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);
}
