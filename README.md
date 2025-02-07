TODO:
- Change crate::cell::DIGIT so that the range 0..9 represents digits 1-9 in
  a cell.
  The specific implementation details of the Cell struct shouldn't really
  be known outside of its module, and it causes oddities in code outside of
  the module.
- add a variable to Sudoku struct that, per section, per digit,
  lists the cells that belong to each digit
  - this could remove the need for the Sudoku::section_digit_sum variable
- figure out what information exactly is needed from index_manip
  to see what changes can be made to make it more convenient to use
- add method to Sudoku struct that initializes from an array of cells
- Fix/Formalize spacing of definitions
- figure out a more convenient way to manage the type difference b/t
  usize and CellSize
  - maybe just make the CellSize larger?
- rename `r` in Sudoku rules to `ret`
- maybe could make it so that modifying a cell adds them to a list
  which other functions use to see if they should act or not
  - would need some way for all functions to process the list individually
- change Cell to use boolean vector of some sort?
- more robust debugging
  - ie logging the state of a sudoku to a dedicated file as opposed to stdout
  - kinda depends on rules being separate objects
- turn the sudoku rules into separate objects
- break up logic of rules into smaller chunks
- make a function that quantifies the incorrectness of a sudoku
  - ie it measures how much difference there is b/t a solved sudoku and a given sudoku
- make a gui for looking through sudoku history for debugging
  - or maybe just add some character to the middle of the sudoku for easy ctrl+f
- Make the randomly solving a cell a rule or more like a rule
  - for easier integration w/ the debugger
  - I don't think adding it as a rule is desirable, although just making it the last rule would be logically correct.
- Make it so that the digit removal methods in Cell return whether or not a change was detected for greater consolidation of code
- update intersection_removal to use Cell::remove_digits
- add optimized run option to sublime text
- split up section_digit_sum



Helpful website:
https://www.sudokuwiki.org/Getting_Started
