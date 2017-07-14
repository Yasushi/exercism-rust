extern crate combine;
extern crate itertools;
extern crate permutohedron;

use std::collections::{BTreeSet, HashMap};

use combine::{many1, skip_many, Parser, ParseError};
use combine::char::{char, space, spaces, upper};

use itertools::Itertools;
use permutohedron::heap_recursive;
use permutohedron::control::Control::*;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let expr = parse(puzzle);
    if expr.is_err() {
        return None;
    }
    let ref expr = expr.unwrap();

    let ref letters: Vec<char> = letters(&expr).iter().cloned().collect();

    let mut result = None;
    for digits in (0..10).combinations(letters.len()) {
        //println!("DDD {:?}", digits);
        let r = heap_recursive(&mut digits.clone(), move |pm| {
            let dict: HashMap<char, u8> = letters
                .iter()
                .cloned()
                .zip(pm.iter().map(|&x| x as u8))
                .collect();
            //println!("XXX {:?} {:?}", pm, dict);
            if check(expr, &dict) {
                Break(dict)
            } else {
                Continue
            }
            // if check(expr, &dict) {
            //     println!(
            //         "{:?} {} {:?}",
            //         dict.iter().collect::<BTreeMap<_, _>>(),
            //         expr.0.iter().flat_map(|e| to_num(e, &dict)).sum::<i32>(),
            //         to_num(&expr.1, &dict)
            //     )
            // }
            // Continue
        });
        result = match r {
            Break(x) => Some(x),
            Continue => None,
        };
        if result.is_some() {
            break;
        }
    }

    // println!("{:?}", result);
    result
}

fn letters(expr: &(Vec<Vec<char>>, Vec<char>)) -> BTreeSet<char> {
    let mut letters: BTreeSet<char> = BTreeSet::new();
    for e in &expr.0 {
        letters.extend(e);
    }
    letters.extend(&expr.1);
    letters
}

fn check(expr: &(Vec<Vec<char>>, Vec<char>), dict: &HashMap<char, u8>) -> bool {
    // println!(
    //     "CCC {:?}",
    //     expr.0.iter().map(|e| to_num(e, dict)).collect::<Vec<_>>()
    // );
    let left: i32 = expr.0.iter().flat_map(|e| to_num(e, dict)).sum();
    if let Ok(right) = to_num(&expr.1, dict) {

        // println!("{} == {}", left, right);
        left == right
    } else {
        false
    }
}

fn to_num(expr: &Vec<char>, dict: &HashMap<char, u8>) -> Result<i32, ()> {
    let n = expr.iter().fold(0, |acc, c| {
        acc * 10 + (*dict.get(c).unwrap() as i32)
    });
    if expr.len() == n.to_string().len() {
        Ok(n)
    } else {
        Err(())
    }
}

fn parse(puzzle: &str) -> Result<(Vec<Vec<char>>, Vec<char>), ParseError<&str>> {
    many1::<Vec<_>, _>(upper())
        .and(many1::<Vec<_>, _>(
            skip_many(space())
                .skip(char('+').skip(spaces()))
                .and(many1::<Vec<_>, _>(upper()).skip(spaces()))
                .map(|x| x.1),
        ))
        .map(|x| {
            let mut r = vec![x.0];
            r.extend(x.1.iter().cloned());
            r
        })
        .skip(spaces())
        .skip(char('='))
        .skip(char('='))
        .skip(spaces())
        .and(many1::<Vec<_>, _>(upper()))
        .parse(puzzle)
        .map(|x| x.0)
}
