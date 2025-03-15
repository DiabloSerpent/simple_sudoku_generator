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
- make a function that quantifies the incorrectness of a sudoku
  - ie it measures how much difference there is b/t a solved sudoku and a given sudoku
- make a gui for looking through sudoku history for debugging
  - or maybe just add some character to the middle of the sudoku for easy ctrl+f
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
- remove combo_iter.rs file
- add `Sudoku::add_history_entry_from_solution` method
  - and refactor naked_single to use it
- change debug print of Sudoku to highlight solved cells
- make it so that group_removal will write a HiddenGroup where a human
  would recognize it.
- test moving group_removal above intersection_removal in `Sudoku::RULE_ORDER`
- change Cell to overload bitwise operators instead of using some methods



Helpful website:
https://www.sudokuwiki.org/Getting_Started
