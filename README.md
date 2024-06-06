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
- optimize Sudoku::group_removal
  - make it so that the iteration work is done in separate method
    (for readability).
    - need to figure out how to make it not slow
  - store already discovered groups separately. they can be analyzed only when
    changed, and also don't need to check outside of group for new subgroups.
  - according to testing, the group_removal takes ~140 ms.
    The rest of the program takes ~7 ms, ~4 ms of which is intersection_removal.
    This means that group_removal is taking ~95% of the total runtime.