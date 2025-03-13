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
  - use bit flag array per cell/section that is updated after a cell is
    modified, using `register_change` method.
  - could remove the need for the `solved_cell_checked` array.
- change Cell to use boolean vector of some sort?
- more robust debugging
  - ie logging the state of a sudoku to a dedicated file as opposed to stdout
  - currently partially finished, should tie it in w/ refactoring rule logic
- turn the sudoku rules into separate objects
  - might delete
- break up logic of rules into smaller chunks
  - partially done
- make a function that quantifies the incorrectness of a sudoku
  - ie it measures how much difference there is b/t a solved sudoku and a given sudoku
- make a gui for looking through sudoku history for debugging
  - or maybe just add some character to the middle of the sudoku for easy ctrl+f
- update intersection_removal to use Cell::remove_digits
- split up section_digit_sum
- impl clippy suggestions
- update Sudoku structure comment to remove cell_flag
  - and remove CELL_SOLVED const
- refactor find_group to use breadth-first instead of depth-first search
  - maybe change the code to also look for hidden groups?
    - not necessary, but possible, might be a lil quicker (and more fun)
    - I could also just look for hidden pairs
- rename MAX_GROUP_SIZE to MAX_NAKED_GROUP_SIZE? Would be more accurate
- rename Sudoku to SudokuSolver and then create a Sudoku or SudokuBoard type
  alias for greater clarity
- update intersection_removal algorithm
- add `register_change(id: usize)` function to Sudoku to make creating a
  CellChange array easier.



Helpful website:
https://www.sudokuwiki.org/Getting_Started
