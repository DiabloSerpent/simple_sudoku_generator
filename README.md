TODO:
- Complete Sudoku::naked_group and the pair variants
- add a variable to Sudoku struct that, per section, per digit,
  lists the cells that belong to each digit
  - this could remove the need for the Sudoku::section_digit_sum variable
- figure out what information exactly is needed from index_manip
  to see what changes can be made to make it more convenient to use
- add method to Sudoku struct that initializes from an array of cells
- Change main to fill cells randomly instead of by index
- Fix/Formalize spacing of definitions
- Change crate::cell::DIGIT so that the range 0..9 represents digits 1-9 in
  a cell.
  The specific implementation details of the Cell struct shouldn't really
  be known outside of its module, and it causes oddities in code outside of
  the module.
- Make it so that the debug print statement in main.rs can be ignored by git
  somehow.
  Idk what a convenient solution to it would be tho.