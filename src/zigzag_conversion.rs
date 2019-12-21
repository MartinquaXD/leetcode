//https://leetcode.com/problems/zigzag-conversion/

/*
00       06       12       18
01    05 07    11 13    17
02 04    08 10    14 16
03       09       15

(2 * 4"rows" - 2) * col + row =>
(2 * 4"rows" - 2) * col

00 06 12 18
01 05 07 11 13 17
02 04 08 10 14 16
03 09 15


(2 * 4"rows" - 2) * col + row

0-th = (2 * rows - 2) * i
1-st = last + 1
2-st = last + 2
last = (rows - 1) * i
*/


use std::collections::HashMap;

pub fn convert(s: String, num_rows: i32) -> String {
    let rows_iter = ((0..num_rows).chain((1 .. num_rows - 1).rev())).cycle();

    let mut rows = vec![String::from(""); num_rows as usize];

    s.chars().zip(rows_iter).for_each(|(c, row)|{
        rows[row as usize].push(c);
    });

    rows.into_iter().collect::<String>()
}