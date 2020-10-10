# Tic-tac-toe

A tic-tac-toe game in Rust, for fun and/or profit.

## Running

    cargo run

## Testing

    cargo test

## State of development

* The computer always plays X, the player always plays O.
* The computer will seize winning moves and attempt to fork its opponent, while blocking opposing attempts to fork it. Otherwise, it moves at random.
* To change the game setup (player vs. player, computer vs. computer, player as X), change the `ComputerPlayer`/`HumanPlayer` initialization in `lib::run()`.

### To do

* Prompt for game type on launch (player vs. player etc.)
* Add integration tests
* Make the computer smarter, ideally to play a perfect game every time
  * Add a test to validate that the computer never loses against a fuzzer
