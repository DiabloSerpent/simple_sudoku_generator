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
- Change main to fill cells randomly instead of by index
  - could add Sudoku::free_cells and revamp Sudoku::cell_solved
    to use that instead of Sudoku::cell_flags, as it seems there
    won't be any other use case for it.
- Fix/Formalize spacing of definitions
- figure out a more convenient way to manage the type difference b/t
  usize and CellSize
- rename `r` in Sudoku rules to `ret`
- maybe could make it so that modifying a cell adds them to a list
  which other functions use to see if they should act or not
  - would need some way for all functions to process the list individually
- needing to check Cell::has_digit twice while removing a digit is annoying
- change Cell to use boolean vector of some sort?
- more robust debugging
  - ie logging the state of a sudoku to a dedicated file as opposed to stdout
  - kinda depends on rules being separate objects
- turn the sudoku rules into separate objects
- break up logic of rules into smaller chunks
- make it so that Sudoku::RULE_ORDER doesn't need to list the size of the array



Helpful website:
https://www.sudokuwiki.org/Getting_Started
