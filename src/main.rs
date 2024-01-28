/*
    Author: Ashley DeMott
    Project: Yahtzee
    Description: Creating a game of Yahtzee using Rust
*/
use std::io::Write;
use rand::Rng;
use std::collections::HashMap;

// The ability to roll a random value
trait Random {
    fn roll(&mut self);
}

// Allow cloning of Die, used with vec![]
#[allow(unused)]
#[derive(Debug, Clone)]
struct Die {
    num: u8, // The Die's number
    frozen: bool, // If the Die cannot be rolled
}
// Implement the default values for a DIe
impl Default for Die {
    fn default() -> Die {
        // Create a default Die
        return Die {
            num: 0,
            frozen: false,
        };
    }
}
// Implement the functions of Random (can roll a Die)
impl Random for Die {
    fn roll(&mut self) {
        // Randomize the die value if it isn't frozen
        if !self.frozen {
            self.num = rand::thread_rng().gen_range(1..7); // 1 - 6 (inclusive)
        }
    }
}

// Getters, immutable and perform the same for ALL scorecard Sections
trait Section {
    fn is_filled(&self) -> bool;
    fn get_points(&self) -> i32;
    fn get_name(&self) -> &'static str;
    fn print(&self);
}

// The ability to calculate points from a vector of Die
trait Points {
    fn calc_score(&self, dice: &Vec<Die>) -> i32;
    fn set_score(&mut self, score: i32);
}

// All Scores have these attributes and implement Section
struct Score {
    filled: bool, // If the score section has been filled
    points: i32, // The point value of the score section
    name: &'static str, // Name of the score section
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
        // Display points if filled, otherwise empty string
        print!("{0}: {1: <3}", self.name, if self.filled {
            self.points.to_string()
        } else {
            " ".to_string()
        });
    }
}

// Get points for having specific number/value
struct Section1 {
    score: Score, // Has a Score section
    value: u8, // The Die value that counts for points
}
impl Points for Section1 {
    fn calc_score(&self, dice: &Vec<Die>) -> i32 {
        let mut score = 0;

        // For every die,
        for die in dice {
            // Only add points for those of the specified value
            if die.num == self.value {
                score += die.num as i32;
            }
        }
        return score;
    }
    fn set_score(&mut self, score: i32) {
        // Assert that the score hasn't already been set
        assert!(self.score.points == 0);
        assert!(!self.score.filled);

        // Fill with the given score
        self.score.filled = true;
        self.score.points = score;
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
    value: u8,
}
impl Points for Section2 {
    fn calc_score(&self, dice: &Vec<Die>) -> i32 {
        let mut score = 0;

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
        if mode >= self.value {
            score = dice_total;
        }

        // Return the calculated score
        return score;
    }
    fn set_score(&mut self, score: i32) {
        // Assert that the score hasn't already been set
        assert!(self.score.points == 0);
        assert!(!self.score.filled);

        // Fill with the given score
        self.score.filled = true;
        self.score.points = score;
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
    value: u8,
}
impl Points for Section3 {
    fn calc_score(&self, dice: &Vec<Die>) -> i32 {
        // For every die,
        let mut present: Vec<u8> = Vec::new();

        let mut straight = false;

        // Get a true/false for every value
        for die in dice {
            // If not already in the list,
            if !present.contains(&die.num) {
                present.push(die.num);
            }
        }

        // TODO: Remake this without hardcoded straights
        // For a straight of five (12345 or 23456)
        if self.value == 5 {
            // If there isn't a 1,
            if !present.contains(&1) {
                // If there isn't a 6,
                if !present.contains(&6) {
                    return 0; // Doesn't have a 1 or 6, cannot be a straight of 5
                }
                // There is a 6, but not a 1
            } else {
                if present.contains(&6) {
                    return 0; // Can't have both a 1 or 6 in a straight of 5
                }
                // There is a 1, but no 6
            }

            // Both straights contain 2345
            straight =
                present.contains(&2) &
                present.contains(&3) &
                present.contains(&4) &
                present.contains(&5);
        }
        if self.value == 4 {
            // Hardcoded straights
            straight =
                (present.contains(&1) &
                    present.contains(&2) &
                    present.contains(&3) &
                    present.contains(&4)) |
                (present.contains(&2) &
                    present.contains(&3) &
                    present.contains(&4) &
                    present.contains(&5)) |
                (present.contains(&3) &
                    present.contains(&4) &
                    present.contains(&5) &
                    present.contains(&6));
        }
        if self.value == 3 {
            // Hardcoded straights
            straight =
                (present.contains(&1) & present.contains(&2) & present.contains(&3)) |
                (present.contains(&2) & present.contains(&3) & present.contains(&4)) |
                (present.contains(&3) & present.contains(&4) & present.contains(&5)) |
                (present.contains(&4) & present.contains(&5) & present.contains(&6));
        }

        // Return 30, 40, or 50 (depending on straight size) if there is a straight
        return if straight {
            (self.value * 10) as i32
        } else {
            0
        };
    }
    fn set_score(&mut self, score: i32) {
        // Assert that the score hasn't already been set
        assert!(self.score.points == 0);
        assert!(!self.score.filled);

        // Fill with the given score
        self.score.filled = true;
        self.score.points = score;
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

// Combination of the traits Points ans Section, all structs
//  implementing both can be in a collection of &dyn PointSections
trait PointSection: Points + Section {}

// All score sections are under a shared trait
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
fn menu_choice(rolls: u8) -> u8 {
    assert!(rolls >= (0 as u8)); // Assert game in valid state
    assert!(rolls <= MAX_ROLLS);

    // Display the menu, prompt for a choice
    println!("\nMenu:\n[1] Roll Dice\n[2] Freeze Dice\n[3] Pick Score\n[0] Quit\n"); // Display the menu

    // Until the user has picked a valid choice,
    loop {
        let choice = get_int("Pick a menu choice", &0, &MAX_ROLLS);

        // always allow the user to quit
        if choice == 0 {
            return choice;
        } else if
            // If the user is out of rolls, but hasn't chosen to end their turn,
            (rolls == 0) & (choice != MAX_ROLLS)
        {
            // Cannot roll if out of rolls
            println!("Please pick a score section.");
        } else if
            // If the user hasn't rolled yet, but is choosing something else,
            (rolls == MAX_ROLLS) & (choice != 1)
        {
            // Cannot roll if out of rolls
            println!("Please roll first.");
        } else {
            // Valid choice
            return choice;
        }
    }
}

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

// Calculate the total game score
fn get_score(scorecard: &Vec<&mut &mut dyn PointSection>) -> i32 {
    let mut total = 0;
    for section in scorecard {
        total += section.get_points();
    }
    return total;
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
fn reset_turn(rolls: &mut u8, dice: &mut Vec<Die>) {
    assert!(dice.len() == 5); // Assert number of dice is the same
    *rolls = MAX_ROLLS;

    // Reset all the Die (unfreeze and set to 0)
    for die in dice {
        die.frozen = false;
        die.num = 0;
    }
}

// The number of rolls the player starts each round with
static MAX_ROLLS: u8 = 3;

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
        value: 3,
    });
    let mut s2_4: &mut dyn PointSection = &mut (Section2 {
        score: Score { name: "8. 4 of a Kind", points: 0, filled: false },
        value: 4,
    });
    let mut s2_5: &mut dyn PointSection = &mut (Section2 {
        score: Score { name: "9. YAHTZEE", points: 0, filled: false },
        value: 5,
    });

    // Straights of 3, 4, or 5 (all different)
    let mut s3_3: &mut dyn PointSection = &mut (Section3 {
        score: Score { name: "10. Small Straight", points: 0, filled: false },
        value: 3,
    });
    let mut s3_4: &mut dyn PointSection = &mut (Section3 {
        score: Score { name: "11. Large Straight", points: 0, filled: false },
        value: 4,
    });
    let mut s3_5: &mut dyn PointSection = &mut (Section3 {
        score: Score { name: "12. Full House", points: 0, filled: false },
        value: 5,
    });

    // Chance (counts up all, as a points for '0 of a kind' Section)
    let mut chance: &mut dyn PointSection = &mut (Section2 {
        score: Score { name: "13. Chance", points: 0, filled: false },
        value: 0,
    });

    // Keep all PointSections in a vector
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
    let mut total_score = 0; // Total points from all scorecard sections
    let mut rolls = MAX_ROLLS; // The number of rolls the player has left

    // While the scorecard is not full,
    while empty_section(&scorecard) {
        // Display the dice, scoreboard, and total score
        display_dice(&game_dice);
        display_scorecard(&scorecard);
        println!("Total Score: {total_score}");

        // Assert game is in a valid state
        assert!(rolls >= (0 as u8));
        assert!(rolls <= MAX_ROLLS);
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

                // Pick a die to freeze, 0 to cancel
                let choice = get_int(
                    "Which die should be frozen/unfrozen?",
                    &0,
                    &(game_dice.len() as u8)
                );

                // If a Die has been chosen,
                if choice != 0 {
                    // Get the Die's index from the user's choice
                    let freeze_i: usize = usize::from(choice - 1);

                    // Invert the Die's frozen state
                    game_dice[freeze_i].frozen = !game_dice[freeze_i].frozen;
                }
            }

            // 3. Pick point section
            3 => {
                let mut pick = false;
                while !pick {
                    display_scorecard(&scorecard); // Display scorecard sections
                    let section_i: usize = usize::from(
                        get_int("Pick a section", &1, &(scorecard.len() as u8)) - 1
                    );

                    // If the section has not been filled,
                    if !scorecard[section_i].is_filled() {
                        pick = true;

                        // Calculate and set the score
                        let score = scorecard[section_i].calc_score(&game_dice);
                        scorecard[section_i].set_score(score);

                        // Recalculate the total score and reset for next turn
                        total_score = get_score(&scorecard);
                        reset_turn(&mut rolls, &mut game_dice);
                    } else {
                        // Tell the user it is already filled
                        println!("That section is already filled.");
                    }
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

    // If the game was finished,
    if !empty_section(&scorecard) {
        // Display final score
        println!("Game over! Total score: {}", total_score);
    }
}
