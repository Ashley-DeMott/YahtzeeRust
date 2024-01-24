/*
    Author: Ashley DeMott
    Project: Yahtzee
    Description: Creating a game of Yahtzee using Rust
*/

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
            self.num = if self.num == 1 { 0 } else { 1 };
        }
    }
}

// TODO: Rename traits/structs, currenlty just testing functionality

// The ability to calculate # points from criteria (specific to Section types (# of, # in a row, etc))
trait Points {
    fn calc_score(&mut self);
    // Get/set name? (displayed on scorecard) ex: Total of [#], [#] of a kind
}

// Getters, immutable and perform the same for ALL scorecard sections
trait Score {
    fn is_filled(&self) -> bool;
    fn get_points(&self) -> i32;
    fn get_name(&self) -> &'static str;
    fn print(&self);
}

trait ScorePlus: Points + Score {}

// All Sections have these attributes
struct Section {
    filled: bool,
    points: i32,
    name: &'static str // Strings are fun, size not known
}
impl Score for Section {
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
        print!("{0}: {1: <3}", self.name, if self.filled {self.points.to_string()} else {" ".to_string()});
    }
}

// Get points for having specific number/value
struct Section1 {
    score: Section, // Has a combo score value and filled status (ALL Sections have this, easier to edit in one place, right?)
    value: i8,
}
impl Points for Section1 {
    fn calc_score(&mut self) {
        // Pass die values
        // If die.num == self.value
        // self.score += value
    }
}
// HOWEVER, to access score's values at the top level..
impl Score for Section1 {
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
    score: Section,
    num: i8,
}
impl Points for Section2 {
    fn calc_score(&mut self) {
        // Pass die values
        // Get die.num of most common
        // if count > self.num
        // self.score = ALL die.num [Hasbro Yahtzee rules]
    }
}
impl Score for Section2 {
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
    score: Section,
    num: i8,
}
impl Points for Section3 {
    fn calc_score(&mut self) {
        // ignores duplicates,
        // self.score =
    }
}
impl Score for Section3 {
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

// Chance - add up all die numbers, might be able to do #2 with num = 0?
struct Section4 {
    score: Section,
}
impl Points for Section4 {
    fn calc_score(&mut self) {
        //self.score.points = ALL die.num
    }
}
impl Score for Section4 {
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
impl ScorePlus for Section1{}
impl ScorePlus for Section2{}
impl ScorePlus for Section3{}
impl ScorePlus for Section4{}


// Display the dice
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

fn display_scorecard(scorecard: Vec<&dyn ScorePlus>) {
    let mut col: u8 = 0; // Count the columns printed

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

// Unrealted structs, so use trait Points? Score?
fn create_scorecard() -> Vec<&'static dyn ScorePlus> {
    // Create a Scorecard, has vector/collection of Scores
    let mut scorecard: Vec<&dyn ScorePlus> = Vec::new();

    // TODO: Rename to proper struct names
    // Totals for a certain number (1 - 6)
    scorecard.push(&(Section1 { score: Section { name: "Aces", points: 0, filled: false }, value: 1 }));
    scorecard.push(&(Section1 { score: Section { name: "Twos",points: 0, filled: false }, value: 2 }));
    scorecard.push(&(Section1 { score: Section { name: "Threes",points: 0, filled: false }, value: 3 }));
    scorecard.push(&(Section1 { score: Section { name: "Fours",points: 0, filled: false }, value: 4 }));
    scorecard.push(&(Section1 { score: Section { name: "Fives",points: 0, filled: false }, value: 5 }));
    scorecard.push(&(Section1 { score: Section { name: "Sixes",points: 0, filled: false }, value: 6 }));

    // 3, 4, or 5 of a kind
    scorecard.push(&(Section2 { score: Section { name: "3 of a Kind",points: 0, filled: false }, num: 3 }));
    scorecard.push(&(Section2 { score: Section { name: "4 of a Kind",points: 0, filled: false }, num: 4 }));
    scorecard.push(&(Section2 { score: Section { name: "YAHTZEE",points: 0, filled: false }, num: 5 }));

    // Straights of 3, 4, or 5 (all different)
    scorecard.push(&(Section3 { score: Section { name: "Small Straight",points: 0, filled: false }, num: 3 }));
    scorecard.push(&(Section3 { score: Section { name: "Large Straight",points: 0, filled: false }, num: 4 }));
    scorecard.push(&(Section3 { score: Section { name: "Full House",points: 0, filled: false }, num: 5 }));

    // Chance
    scorecard.push(&(Section2 { score: Section { name: "Chance",points: 0, filled: false }, num: 0 })); // # of a kind?

    return scorecard;
}

fn empty_section(scorecard: Vec<&dyn Score>) -> bool {
    // If an empty section is found, return true
    for score in scorecard {
        if score.get_points() == 0 {
            return true;
        }
    }

    // If none are empty, return false, the game is over
    return false;
}

fn get_int(prompt: &str) -> u8 {
    print!("{}: ", prompt);

    // Error handling

    //
    return 1;
}

fn main() {
    let len = 5;
    // Create a Vector of dice
    let mut game_dice: Vec<Die> = vec![Die::default(); len];
    let mut scorecard: Vec<&dyn ScorePlus> = create_scorecard();

    let mut total_score = 0; // Add from scorecard (call summation each time, or only when new points added?)
    let mut rolls = 3; // Have a const int for max_rolls?
    let mut pick = false; // If the Player has picked a score section

    // For a turn: display dice, allow player choice
    // For a round: rolls > 0, a score section must be picked (ends turn even if rolls > 0)
    
    // While the scorecard is not full,
    // while empty_section()

    // While a score section has not been picked,
    //while !pick {
        // TODO: Prompt player for turn choice
        display_dice(&game_dice);
        display_scorecard(scorecard);

        // 1. Roll again
        //    - Check if rolls left
        // For every die in the vector,
        if rolls > 0 {
            rolls -= 1;
            for die in &mut game_dice {
                die.roll();
            }
        }

        // 2. Freeze/unfreeze
        //    - Prompt die # (convert to index), use ternary as toggle

        // Freezing a die
        game_dice[2].frozen = true;

        // 3. Pick point section
        //    - Prompt for number 1 - # point sections
        //    - Check if picked
        // scorecard[input].calc_score(game_dice);

        // TODO: Add game_loop
        // While scorecard not filled, repeat
        // Add option to quit?
    //}
    display_dice(&game_dice);
}