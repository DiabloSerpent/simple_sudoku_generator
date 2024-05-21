use crate::Sudoku;
//use crate::index_manip::*;

impl Sudoku {
    pub fn intersection_removal(&mut self) -> bool {
        // Remove digits that are outside of an intersection.

        /* Algorithm:

            search by row, then col:
                create array of 9 vectors, corresponding to each digit

                continue if unsolved count in section < 2

                search by cell in section:
                    continue if cell is solved

                    search by digit in cell:
                        if count of digit in this section <= 3
                            && count of digit in box_of(cell) > 2:
                                array[digit].push(cell)

                for vector in array:
                    let d = current digit

                    // if vector isnt empty, it should have >1 element
                    if vector is empty:
                        continue

                    let box = box_of(vector[0])
                    if vector[1..].iter().position(|&x| box_of(x) != box) == None {
                        continue;
                    }

                    for cell in of_box(box):
                        if cell has digit:
                            r = true;
                            cell.remove_digit(d)


                //if the only 2 or 3 cells that hold a particular digit are in the same box,
                //then clear out that digit from the rest of the cells in that box

            search by box:
                if the only 2 or 3 cells that hold a particular digit are in the same row/col,
                then clear out that digit from the rest of the cells in that row/col
        */

        false // TODO
    }
}