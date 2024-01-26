/*
    Author: Ashley DeMott
    Project: Yahtzee
    Description: Creating a game of Yahtzee using Rust
*/

use std::io::Write;
use std::collections::HashMap;
use rand::Rng;

// The ability to roll a random value
trait Random {
    fn roll(&mut self);
}

// Allow cloning of this, used with vec![]
#[allow(unused)]
#[derive(Debug, Clone)]
struct Die {
    num: u8, // The Die's number
    frozen: bool, // If the Die cannot be rolled
}

impl Default for Die {
    fn default() -> Die {
        return Die {
            num: 0,
            frozen: false,
        };
    }
}

impl Random for Die {
    fn roll(&mut self) {
        if !self.frozen {
            self.num = rand::thread_rng().gen_range(1..6);
        }
    }
}

// TODO: Rename traits/structs, currenlty just testing functionality

// The ability to calculate # points from criteria (specific to Section types (# of, # in a row, etc))
trait Points {
    fn fill(&mut self);
    fn calc_score(&mut self, dice: &Vec<Die>);
}

// Getters, immutable and perform the same for ALL scorecard Sections
trait Section {
    fn is_filled(&self) -> bool;
    fn get_points(&self) -> i32;
    fn get_name(&self) -> &'static str;
    fn print(&self);
}

trait PointSection: Points + Section {}

// All Scores have these attributes
struct Score {
    filled: bool,
    points: i32,
    name: &'static str, // Strings are fun, size not known
}
impl Section for Score {
    fn is_filled(&self) -> bool {
        return self.filled;
    }
    fn get_points(&self) -> i32 {
        return self.points;
    }
    fn get_name(&self) -> &'static str {
        return self.name;
    }
    fn print(&self) {
        print!("{0}: {1: <3}", self.name, if self.filled {
            self.points.to_string()
        } else {
            " ".to_string()
        });
    }
}

// Get points for having specific number/value
struct Section1 {
    score: Score, // Has a combo score value and filled status (ALL Sections have this, easier to edit in one place, right?)
    value: u8,
}
impl Points for Section1 {
    fn fill(&mut self) {
        self.score.filled = true;
    }
    fn calc_score(&mut self, dice: &Vec<Die>) {
        assert!(self.score.points == 0);
        self.score.filled = true;
        self.score.points = 50;

        /*
        // For every die,
        for die in dice {
            // Only add points for those of the specified value
            if die.num == self.value {
                self.score.points += die.num as i32;
            }
        }*/
    }
}
// To access score's values at the top level..
impl Section for Section1 {
    fn get_points(&self) -> i32 {
        return self.score.get_points();
    }
    fn is_filled(&self) -> bool {
        return self.score.is_filled();
    }
    fn get_name(&self) -> &'static str {
        return self.score.get_name();
    }
    fn print(&self) {
        self.score.print();
    }
}

// Get points for having # of a kind, YAHTZEE = 5 of a kind
struct Section2 {
    score: Score,
    num: u8,
}
impl Points for Section2 {
    fn fill(&mut self) {
        self.score.filled = true;
    }
    fn calc_score(&mut self, dice: &Vec<Die>) {
        assert!(self.score.points == 0);
        self.score.filled = true;

        // Create a hashmap (key: die num, value: # in game_dice)
        let mut counts: HashMap<u8, u8> = HashMap::new();
        let mut dice_total: i32 = 0; // The total value of game_dice

        for die in dice {
            // Find if the number is there, otherwise create a new key/value pair
            let c = counts.entry(die.num).or_insert(0);
            *c += 1; // Add one to the count

            // Add to the total value of the dice
            dice_total += die.num as i32;
        }

        // Find the mode from the hashmap (or 0, if not found)
        let mode = counts.values().cloned().max().unwrap_or(0);

        // If enough of a single type, points = dice total [Hasbro Yahtzee rules]
        if mode > self.num {
            self.score.points = dice_total;
        }
        // Otherwise, filled, but with a value of 0 (default)
    }
}
impl Section for Section2 {
    fn get_points(&self) -> i32 {
        return self.score.get_points();
    }
    fn is_filled(&self) -> bool {
        return self.score.is_filled();
    }
    fn get_name(&self) -> &'static str {
        return self.score.get_name();
    }
    fn print(&self) {
        self.score.print();
    }
}

// Small (3), Large(4), and full/one-of-a-kind(5) straights (num = num in a row needed)
struct Section3 {
    score: Score,
    num: i8,
}
impl Points for Section3 {
    fn fill(&mut self) {
        self.score.filled = true;
    }
    fn calc_score(&mut self, dice: &Vec<Die>) {
        assert!(self.score.points == 0);
        self.score.filled = true;

        // ignores duplicates,
        // self.score =
    }
}
impl Section for Section3 {
    fn get_points(&self) -> i32 {
        return self.score.get_points();
    }
    fn is_filled(&self) -> bool {
        return self.score.is_filled();
    }
    fn get_name(&self) -> &'static str {
        return self.score.get_name();
    }
    fn print(&self) {
        self.score.print();
    }
}

// Under a shared trait, can call methods from both
impl PointSection for Section1 {}
impl PointSection for Section2 {}
impl PointSection for Section3 {}

// Display the current state of the dice
fn display_dice(dice: &Vec<Die>) {
    for die in dice {
        // The box's exterior is different if frozen/unfrozen
        let box_str = if die.frozen { ('<', '>') } else { ('[', ']') };

        // Die numbers from 1 - 6 are shown, 0 is not shown (nothing has been rolled)
        let box_num = if die.num != 0 { die.num.to_string() } else { " ".to_string() };

        // Display the number within the box
        print!("{} {} {} ", box_str.0, box_num, box_str.1);
    }
    println!();
}

// Display the current state of the Scorecard
fn display_scorecard(scorecard: &Vec<&mut &mut dyn PointSection>) {
    let mut col: u8 = 0; // Count the columns printed

    println!(); // newline
    // For every score in the scorecard,
    for score in scorecard {
        score.print();
        col += 1;

        // Limit the number of columns to 5
        if col > 4 {
            println!(); // Start a new line
            col = 0; // Reset the count
        }
    }
    println!(); // End line
}

// Pick a choice from the displayed menu, automatically sets to 'Pick score' if out of rolls
fn menu_choice(rolls: i32) -> u8 {
    assert!(rolls >= 0); // Assert game in valid state
    assert!(rolls <= 3);

    // Display the menu, prompt for a choice
    println!("\nMenu:\n[1] Roll Dice\n[2] Freeze Dice\n[3] Pick Score\n[0] Quit\n"); // Display the menu

    // Until the user has picked a valid choice,
    loop {
        let choice = get_int("Pick a menu choice", &0, &3);

        // always allow the user to quit
        if choice == 0 {
            return choice;
        } else if
            // If the user is out of rolls, but hasn't chosen to end their turn,
            (rolls == 0) & (choice != 3)
        {
            // Cannot roll if out of rolls
            println!("Please pick a score section.");
        } else if
            // If the user hasn't rolled yet, but is chooseing something else,
            (rolls == 3) & (choice != 1)
        {
            // Cannot roll if out of rolls
            println!("Please roll first.");
        } else {
            // Valid choice
            return choice;
        }
    }
}

/*fn create_scorecard() -> Vec<&mut dyn PointSection> {
    // NOTE: Not used due to issues with mutablilty and ownership
    // This worked when PointSections were NOT mutable

    // Create a collection of PointSections, which are mutable
    let mut scorecard: Vec<&mut dyn PointSection> = Vec::new();

    // Totals for a certain number (1 - 6)
    scorecard.push(
        &mut(Section1 { score: Score { name: "Aces", points: 0, filled: false }, value: 1 })
    );
    scorecard.push(
        &mut(Section1 { score: Score { name: "Twos", points: 0, filled: false }, value: 2 })
    );
    scorecard.push(
        &mut(Section1 { score: Score { name: "Threes", points: 0, filled: false }, value: 3 })
    );
    scorecard.push(
        &mut(Section1 { score: Score { name: "Fours", points: 0, filled: false }, value: 4 })
    );
    scorecard.push(
        &mut(Section1 { score: Score { name: "Fives", points: 0, filled: false }, value: 5 })
    );
    scorecard.push(
        &mut(Section1 { score: Score { name: "Sixes", points: 0, filled: false }, value: 6 })
    );

    // 3, 4, or 5 of a kind
    scorecard.push(
        &mut(Section2 { score: Score { name: "3 of a Kind", points: 0, filled: false }, num: 3 })
    );
    scorecard.push(
        &mut(Section2 { score: Score { name: "4 of a Kind", points: 0, filled: false }, num: 4 })
    );
    scorecard.push(
        &mut(Section2 { score: Score { name: "YAHTZEE", points: 0, filled: false }, num: 5 })
    );

    // Straights of 3, 4, or 5 (all different)
    scorecard.push(
        &mut(Section3 { score: Score { name: "Small Straight", points: 0, filled: false }, num: 3 })
    );
    scorecard.push(
        &mut(Section3 { score: Score { name: "Large Straight", points: 0, filled: false }, num: 4 })
    );
    scorecard.push(
        &mut(Section3 { score: Score { name: "Full House", points: 0, filled: false }, num: 5 })
    );

    // Chance
    scorecard.push(
        &mut(Section2 { score: Score { name: "Chance", points: 0, filled: false }, num: 0 })
    ); // # of a kind?

    return scorecard;
}*/

// Checks if there is an empty section in the Scorecard
fn empty_section(scorecard: &Vec<&mut &mut dyn PointSection>) -> bool {
    // For every score section in the scorecard,
    for score in scorecard {
        // Check if there is an empty section
        if !score.is_filled() {
            return true; // Not done with game
        }
    }

    // If none are empty, return false, the game is over
    return false;
}

// Min and max acceptable values (within u8, positive integers)
fn get_int(prompt: &str, min: &u8, max: &u8) -> u8 {
    loop {
        print!("{}: ", prompt);
        std::io::stdout().flush().unwrap(); // Flush the buffer so the print shows

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read input");

        match input.trim().parse::<u8>() {
            Ok(i) => {
                println!();
                if (i <= *max) & (i >= *min) {
                    return i;
                } else {
                    println!("Please enter positive integer between {} and {}", *min, *max);
                }
            }
            Err(..) => {
                println!("Please enter a valid integer");
            }
        }
    }
}

// Reset for the next turn
fn reset_turn(rolls: &mut i32, dice: &mut Vec<Die>) {
    assert!(dice.len() == 5); // Assert number of dice is the same
    *rolls = 3;

    // Reset all the Die (unfreeze and set to 0)
    for die in dice {
        die.frozen = false;
        die.num = 0;
    }
}

fn main() {
    // Create all the PointSections for the scorecard
    let mut s1_1: &mut dyn PointSection = &mut (Section1 {
        score: Score { name: "1. Aces", points: 0, filled: false },
        value: 1,
    });
    let mut s1_2: &mut dyn PointSection = &mut (Section1 {
        score: Score { name: "2. Twos", points: 0, filled: false },
        value: 2,
    });
    let mut s1_3: &mut dyn PointSection = &mut (Section1 {
        score: Score { name: "3. Threes", points: 0, filled: false },
        value: 3,
    });
    let mut s1_4: &mut dyn PointSection = &mut (Section1 {
        score: Score { name: "4. Fours", points: 0, filled: false },
        value: 4,
    });
    let mut s1_5: &mut dyn PointSection = &mut (Section1 {
        score: Score { name: "5. Fives", points: 0, filled: false },
        value: 5,
    });
    let mut s1_6: &mut dyn PointSection = &mut (Section1 {
        score: Score { name: "6. Sixes", points: 0, filled: false },
        value: 6,
    });

    // 3, 4, or 5 of a kind
    let mut s2_3: &mut dyn PointSection = &mut (Section2 {
        score: Score { name: "7. 3 of a Kind", points: 0, filled: false },
        num: 3,
    });
    let mut s2_4: &mut dyn PointSection = &mut (Section2 {
        score: Score { name: "8. 4 of a Kind", points: 0, filled: false },
        num: 4,
    });
    let mut s2_5: &mut dyn PointSection = &mut (Section2 {
        score: Score { name: "9. YAHTZEE", points: 0, filled: false },
        num: 5,
    });

    // Straights of 3, 4, or 5 (all different)
    let mut s3_3: &mut dyn PointSection = &mut (Section3 {
        score: Score { name: "10. Small Straight", points: 0, filled: false },
        num: 3,
    });
    let mut s3_4: &mut dyn PointSection = &mut (Section3 {
        score: Score { name: "11. Large Straight", points: 0, filled: false },
        num: 4,
    });
    let mut s3_5: &mut dyn PointSection = &mut (Section3 {
        score: Score { name: "12. Full House", points: 0, filled: false },
        num: 5,
    });

    // Chance (counts up all, as a points for '0 of a kind' Section)
    let mut chance: &mut dyn PointSection = &mut (Section2 {
        score: Score { name: "13. Chance", points: 0, filled: false },
        num: 0,
    });

    // Keep all PointSections in a vector (ToDo, place in an array? No sections are added/removed)
    let mut scorecard: Vec<&mut &mut dyn PointSection> = vec![
        &mut s1_1,
        &mut s1_2,
        &mut s1_3,
        &mut s1_4,
        &mut s1_5,
        &mut s1_6,
        &mut s2_3,
        &mut s2_4,
        &mut s2_5,
        &mut s3_3,
        &mut s3_4,
        &mut s3_5,
        &mut chance
    ];

    // Create a Vector of 5 dice
    let mut game_dice: Vec<Die> = vec![Die::default(); 5];
    let mut total_score = 0; // Add from scorecard (call summation each time, or only when new points added?)
    let mut rolls = 3; // Have a const int for max_rolls?
    let mut pick = false; // If the Player has picked a score section

    // For a turn: display dice, allow player choice
    // For a round: rolls > 0, a score section must be picked (ends turn even if rolls > 0)

    // While the scorecard is not full,
    while empty_section(&scorecard) {
        display_dice(&game_dice);
        display_scorecard(&scorecard);
        println!("Total Score: {}", total_score);

        // Assert game is in a valid state
        assert!(rolls >= 0);
        assert!(rolls <= 3); // TODO: set 3 as a global const variable instead of magic number
        assert!(empty_section(&scorecard));

        // Depending on the user's choice,
        match menu_choice(rolls) {
            // 1. Roll the Dice
            1 => {
                rolls -= 1;

                // For every die in the vector,
                for die in &mut game_dice {
                    die.roll();
                }
            }

            // 2. Freeze/unfreeze a certain Die
            2 => {
                display_dice(&game_dice); // Display dice

                // Pick a die to freeze
                let freeze_i: usize = usize::from(
                    get_int("Which die should be frozen/unfrozen?", &1, &(game_dice.len() as u8)) -
                        1
                );

                // Invert the frozen state
                game_dice[freeze_i].frozen = !game_dice[freeze_i].frozen;
            }

            // 3. Pick point section
            3 => {
                display_scorecard(&scorecard); // Display scorecard sections
                let section_i: usize = usize::from(get_int("Pick a section", &1, &(scorecard.len() as u8)) - 1);

                // If the section has not been filled,
                if !scorecard[section_i].is_filled() {
                    scorecard[section_i].fill();

                    reset_turn(&mut rolls, &mut game_dice);
                } else {
                    // Tell the user it is already filled
                    println!("That section is already filled.");
                }
            }
            // Exit the game
            0 => {
                return;
            }

            // Invalid menu option
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}
