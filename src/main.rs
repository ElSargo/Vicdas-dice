use colored::{
    Color::{self, *},
    *,
};
use rand::{thread_rng, Rng};
use std::{cmp::Ordering, env::args};

// The game goes as follows, the player throws six dice from which they will try to get some points
// The goal is to reach 4000 points in the fewest number of turns
// Points are awarded when the dice match a certain pattern e.g 2,2,2 5,5,5,5 1,2,3,4,5 etc
// If the player chooses a pattern those dice are 'consumed' and they will continue to throw with the remaining dice
// This process repeats until the either 'banks' the points they have accumulated during their turn or they go 'bust' by throwing
// and having no scoring options available
// Their turn ends after this, if they banked the points it goes towards their score but if they went bust they get nothing
// If all dice are consumed they player gets six more dice
// The player must deiced to either peruse more points by throwing and risk losing them or bank their points, potentiality
// missing out on more

fn main() {
    println!(
        "\nWelcome to dice: get to 4000 in the least amount of throws. Exit game with ctrl + c"
    );
    // The first arg is working dir
    let names = args().into_iter().skip(1);

    // No players passed in
    if names.size_hint().0 == 0 {
        println!("Please pass in the names of the players! e.g $ riskydice Jim James Joe");
        return;
    }

    let mut name_rng = thread_rng();
    let name_colors: Vec<Color> = vec![
        Red,
        BrightRed,
        Green,
        BrightGreen,
        Blue,
        BrightBlue,
        Magenta,
        BrightMagenta,
    ];

    let mut players = Vec::with_capacity(names.size_hint().0);
    for name in names {
        players.push(Player {
            name: name.color(name_colors[name_rng.gen_range(0..name_colors.len())]),
            score: 0,
        });
    }

    let mut input_buffer = String::new();

    // Game runs until score >= 4000
    // Each iteration represents one turn for each player
    for turn in 1.. {
        // We can't iterate over the player vec directly bc muh mutability
        'turn_loop: for i in 0..players.len() {
            display_turn_change(&players, i);

            // Now we can borrow mutably
            let player = &mut players[i];
            let score = &mut player.score;
            // Player can accumulate points, but will not count if they go bust before they bank them
            let mut turn_points = 0;
            let mut can_reroll = false;
            let mut throw = DiceThrow::new();

            // Each iteration represents one roll of the dice
            'roll_loop: loop {
                let mut options = throw.scoring_options();

                // Bust check
                if options.len() == 0 {
                    let bust_msg = "Bust!".bright_red().bold();
                    let throw = format!("{:?}", throw.remaining()).bold();
                    let lost = format!{"turn points: {}", format!{"{turn_points}"}.strikethrough().bright_yellow()}.bold();
                    println!("\n{bust_msg} {throw} {lost}");
                    continue 'turn_loop;
                }

                // The player selects the next move each iteration
                // there can be many per roll
                'selection_loop: loop {
                    let max_selection = options.len() + usize::from(can_reroll);
                    {
                        // Idk if this is more readable
                        let score = format!("{score}").bright_green().bold();
                        let turn_points = format!("{turn_points}").bright_yellow().bold();
                        let remaining = format!("{:?}", throw.remaining());
                        let bank_option = "0| Bank".bright_green().bold().underline();
                        let options = display_options(&options);
                        print!(
                            "
Turn: {turn} (score: {score}, turn points: {turn_points})
{remaining}
{bank_option}
{options}"
                        );
                        if can_reroll {
                            let reroll = format!("{max_selection}| re-roll")
                                .purple()
                                .bold()
                                .underline();
                            println!("{reroll}");
                        }
                    };
                    let selection: usize =
                        get_player_selection(&mut input_buffer, 0..=max_selection);
                    // First option is always to bank
                    if selection == 0 {
                        *score += turn_points;
                        println!("Banking {turn_points}, new score: {score}");
                        continue 'turn_loop;
                    }
                    // Last option is always to re-roll
                    if can_reroll && selection == max_selection {
                        can_reroll = false;
                        throw.re_roll();
                        continue 'roll_loop;
                    }
                    // A scoring option was selected
                    let selection = options.remove(selection - 1);
                    can_reroll = true;
                    turn_points += selection.score;
                    if turn_points + *score >= 4000 {
                        on_win(turn);
                    }
                    throw.remove_all(selection.indexes);
                    // Auto re-roll if player has exhausted dice
                    if throw.remaining().len() == 0 {
                        throw = DiceThrow::new();
                        can_reroll = false;
                        println!("{}", "Exhausted dice, re-rolling!".green().bold());
                    }
                    options = throw.scoring_options();
                    continue 'selection_loop;
                }
            }
        }
    }

    /// Read in a number from the cl
    fn get_player_selection(input: &mut String, range: std::ops::RangeInclusive<usize>) -> usize {
        loop {
            input.clear();
            print!("Select option: 0 to {}\n=> ", range.end());
            let _ = std::io::Write::flush(&mut std::io::stdout());
            if let Err(_) = std::io::stdin().read_line(input) {
                println!("Error reading input");
                continue;
            }
            let s = &input[0..1];
            let attempt: Result<usize, <usize as std::str::FromStr>::Err> = s.parse::<usize>();
            match attempt {
                Ok(i) if range.contains(&i) => return i,
                _ => {
                    println!(
                        "{}",
                        format!("Invalid selection: Number between 0 and {}", range.end()).red()
                    );
                    input.clear();
                    continue;
                }
            };
        }
    }

    fn display_options(opts: &Vec<ScoringOption>) -> ColoredString {
        let mut res = String::new().bold();
        for (i, opt) in opts.iter().enumerate() {
            let n = i + 1;
            res = format!("{res}{n}| {opt} \n")
                .to_owned()
                .as_str()
                .bold()
                .cyan();
        }
        res
    }
}

fn on_win(turn: i32) {
    println!(
        "{} {} {}",
        "Game completed in".green(),
        turn,
        "turns!".green()
    );
    // break 'turn_loop;
    std::process::exit(0);
}

fn display_turn_change(players: &Vec<Player>, i: usize) {
    let player = &players[i];
    let name = &player.name;
    let max = players.iter().max().unwrap();
    let target = if player == max {
        format!("{}", "Winning!".bright_green().underline().bold().italic())
    } else {
        format!(
            "{} {}! {} points behind.",
            "Chasing".red().underline().bold().italic(),
            max.name,
            max.score - player.score
        )
    };
    let msg = format!("{name}'s turn! {target}");
    let squigle = "<><><><><><><><><><><><><><><><><><><><><><><>"
        .bold()
        .strikethrough();
    print!("\n{squigle}\n{msg}\n{squigle}");
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Player {
    name: ColoredString,
    score: u32,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score < other.score {
            Ordering::Less
        } else if self.score > other.score {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score < other.score {
            Some(Ordering::Less)
        } else if self.score > other.score {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Debug)]
struct DiceThrow {
    nums: [Option<u32>; 6],
}

#[derive(Debug, Clone)]
enum ScoreType {
    OfAKind,
    Ones,
    Fives,
    InARow,
}

#[derive(Debug)]
struct ScoringOption {
    indexes: Vec<usize>,
    score: u32,
    stype: ScoreType,
}

impl std::fmt::Display for ScoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScoreType::OfAKind => "of a kind".to_owned(),
                ScoreType::Ones => "ones".to_owned(),
                ScoreType::Fives => "fives".to_owned(),
                ScoreType::InARow => "in a row".to_owned(),
            }
        )
    }
}

impl std::fmt::Display for ScoringOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, [{} {}]", self.score, self.indexes.len(), self.stype)
    }
}

type Die = (usize, u32);

impl DiceThrow {
    /// Returns tuples of numbers and their index
    /// The index is required to reliably remove the values
    fn items(&self) -> Vec<Die> {
        return self
            .nums
            .iter()
            .enumerate()
            .filter(|x| x.1.is_some())
            .map(|x| (x.0, x.1.unwrap()))
            .collect();
    }

    /// Remove the dice at the given indexes
    fn remove_all(&mut self, indexes: Vec<usize>) {
        for i in indexes {
            self.nums[i] = None;
        }
    }

    /// Randomise the remaining dice
    fn re_roll(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.nums.len() {
            if self.nums[i].is_some() {
                self.nums[i] = Some(rng.gen_range(1..7));
            }
        }
    }

    /// Throw six random dice
    fn new() -> DiceThrow {
        let mut rng = rand::thread_rng();
        let mut d = DiceThrow { nums: [None; 6] };
        for i in 0..6 {
            d.nums[i] = Some(rng.gen_range(1..7));
        }
        d
    }

    /// Returns the values of the remaining dice
    fn remaining(&self) -> Vec<u32> {
        self.nums
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<u32>>()
    }

    /// All scoring options avalible for the thrown dice
    fn scoring_options(&self) -> Vec<ScoringOption> {
        let mut options: Vec<ScoringOption> = Vec::new();
        let items = self.items();
        // Ones and fives
        {
            for (num, points, score_type) in [(1, 100, ScoreType::Ones), (5, 50, ScoreType::Fives)]
            {
                let found: Vec<usize> = items.iter().filter(|x| x.1 == num).map(|x| x.0).collect();
                for n in 1..=found.len() {
                    options.push(ScoringOption {
                        indexes: found[0..n].to_vec(),
                        score: points * n as u32,
                        stype: score_type.clone(),
                    })
                }
            }
        }

        // 3,4,5,6 of a kind
        {
            let mut searched_values: std::collections::HashSet<u32> =
                std::collections::HashSet::new();
            for (_, num) in &items {
                // Iterate through the unique numbers but not 5 bc muh rules
                if searched_values.contains(&num) || num == &5 {
                    continue;
                } else {
                    searched_values.insert(*num);
                };

                // Count duplactes
                let same: Vec<usize> = items.iter().filter(|x| x.1 == *num).map(|x| x.0).collect();

                for n in 3..=same.len() {
                    let score = match num {
                        1 => 1000,
                        _ => num * 100,
                    } * 2u32.pow(n as u32 - 3);
                    options.push(ScoringOption {
                        indexes: same[0..n].to_vec(),
                        score,
                        stype: ScoreType::OfAKind,
                    })
                }
            }
        }

        // 5,6 in a row
        'consecutive: {
            let mut values: Vec<u32> = items.iter().map(|x| x.1).collect();
            values.sort();
            let mut indexes = vec![];
            // 2,3,4,5 are in every pattern so they should be checked first
            for num in [2, 3, 4, 5] {
                match values.binary_search(&num) {
                    Ok(i) => indexes.push(i),
                    Err(_) => break 'consecutive,
                }
            }
            // the 1 and 6 values deterine which sequence we got
            let one = values.binary_search(&1);
            let six = values.binary_search(&6);

            // 1,2,3,4,5
            if let Ok(i_one) = one {
                let mut indexes = indexes.clone();
                indexes.push(i_one);
                options.push(ScoringOption {
                    indexes,
                    score: 500,
                    stype: ScoreType::InARow,
                })
            }
            // 2,3,4,5,6
            if let Ok(i_six) = six {
                let mut indexes = indexes.clone();
                indexes.push(i_six);
                options.push(ScoringOption {
                    indexes,
                    score: 750,
                    stype: ScoreType::InARow,
                })
            }
            // 1,2,3,4,5,6
            if let (Ok(i_one), Ok(i_six)) = (one, six) {
                let mut indexes = indexes.clone();
                indexes.extend_from_slice(&[i_one, i_six]);
                options.push(ScoringOption {
                    indexes, // No clone required as it can't be used later
                    score: 1500,
                    stype: ScoreType::InARow,
                })
            }
        }
        options
    }
}
