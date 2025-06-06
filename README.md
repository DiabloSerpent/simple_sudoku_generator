TODO:
- Change crate::cell::DIGIT so that the range 0..9 represents digits 1-9 in
  a cell.
  The specific implementation details of the Cell struct shouldn't really
  be known outside of its module, and it causes oddities in code outside of
  the module.
- add a variable to Sudoku struct that, per section, per digit,
  lists the cells that belong to each digit
  - this could remove the need for the Sudoku::section_digit_sum variable
  - maybe use the Cell struct?
- figure out what information exactly is needed from index_manip
  to see what changes can be made to make it more convenient to use
- add method to Sudoku struct that initializes from an array of cells
- maybe could make it so that modifying a cell adds them to a list
  which other functions use to see if they should act or not
  - would need some way for all functions to process the list individually
  - use bit flag array per cell/section that is updated after a cell is
    modified, using `register_change` method.
  - could remove the need for the `solved_cell_checked` array.
- make a function that quantifies the incorrectness of a sudoku
  - ie it measures how much difference there is b/t a solved sudoku and a given sudoku
- make a gui for looking through sudoku history for debugging
  - or maybe just add some character to the middle of the sudoku for easy ctrl+f
- split up section_digit_sum
- maybe change find_group code to also look for hidden groups?
    - not necessary, but possible, might be a lil quicker (and more fun)
    - mb have that functionality packed into SubSection?
- make it so that group_removal will write a HiddenGroup where a human
  would recognize it.
- change Cell to overload bitwise operators instead of using some methods
- make rng more random? idk.
- make group_removal remove more digits if the found group can be applied to
  a different section or if it uncovers a group that can remove digits in another section
- make intersection_removal emit different history entries based on each digit
  contained in the intersection
- make `ROW_INDICIES`, `COL_INDICES`, and `BOX_INDICES` vars?
- make more helper methods in Cell struct to make code less repetitive
  - and maybe break up code blocks for readability?
- reorganize larger rules so that the entry point is near the top of the file



Helpful website:
https://www.sudokuwiki.org/Getting_Started
