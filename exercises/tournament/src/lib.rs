use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::cmp::Ordering;

#[derive(Debug)]
struct Team {
    name: String,
    win: u32,
    draw: u32,
    lost: u32,
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            win: 0,
            draw: 0,
            lost: 0,
        }
    }

    fn win(&mut self) {
        self.win += 1;
    }

    fn draw(&mut self) {
        self.draw += 1;
    }
    fn lost(&mut self) {
        self.lost += 1;
    }

    fn matches(&self) -> u32 {
        self.win + self.draw + self.lost
    }

    fn point(&self) -> u32 {
        self.win * 3 + self.draw
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            self.name,
            self.matches(),
            self.win,
            self.draw,
            self.lost,
            self.point()
        )
    }
}


pub fn tally(input: &str) -> String {
    let mut teams: BTreeMap<&str, Team> = BTreeMap::new();

    for play in input.split("\n") {
        if play.is_empty() {
            break;
        }
        let ps: Vec<_> = play.split(';').collect();
        match ps[2] {
            "win" => {
                teams.entry(ps[0]).or_insert(Team::new(ps[0])).win();
                teams.entry(ps[1]).or_insert(Team::new(ps[1])).lost();
            }
            "draw" => {
                teams.entry(ps[0]).or_insert(Team::new(ps[0])).draw();
                teams.entry(ps[1]).or_insert(Team::new(ps[1])).draw();
            }
            "loss" => {
                teams.entry(ps[0]).or_insert(Team::new(ps[0])).lost();
                teams.entry(ps[1]).or_insert(Team::new(ps[1])).win();
            }
            _ => (),
        }
    }

    // println!("XXX {:?}", teams.values().collect::<Vec<_>>());

    let mut result = vec![
        "Team                           | MP |  W |  D |  L |  P".to_string(),
    ];
    let mut ts = teams.values().collect::<Vec<_>>();
    ts.sort_by(|x, y| match x.point().cmp(&y.point()).reverse() {
        Ordering::Equal => x.name.cmp(&y.name),
        other => other,
    });
    result.extend(&mut ts.iter().map(|t| format!("{}", t)));
    result.join("\n")
}
