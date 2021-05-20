// use std::fmt::Display;
//
// fn rotate<T>(matrix: &mut [&mut [T]]) -> bool {
//     if matrix.len() == 0 || matrix.len() != matrix[0].len() {
//         return false;
//     }; // Not a square
//     let n = matrix.len();
//
//     for layer in 0..(n/2) {
//         let first = layer;
//         let last = n - 1 - layer;
//         for i in first..last {
//             let offset = i - first;
//             let top = &matrix[first][i]; // save top
//
//             // left -> top
//             matrix[first][i] = matrix[last-offset][first];
//
//             // bottom -> left
//             matrix[last-offset][first] = matrix[last][last - offset];
//
//             // right -> bottom
//             matrix[last][last - offset] = matrix[i][last];
//
//             // top -> right
//             matrix[i][last] = top; // right <- saved top
//         }
//     }
//     true
// }
//
// fn print_matrix<T: Display>(m: & std::vec::Vec<std::vec::Vec<T>>) {
//     println!("Rotated result: {},{},{},{}",m[0][0],m[0][1],m[1][0],m[1][1]);
// }
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_rotate_matrix() {
//         let mut matrix = vec![vec![1,2],vec![3,4]];
//         // super::print_matrix(&matrix);
//         super::rotate(&mut matrix[..]);
//         // super::print_matrix(&matrix);
//         assert_eq!(matrix[0][0],3);
//         assert_eq!(matrix[0][1],1);
//         assert_eq!(matrix[1][0],2);
//         assert_eq!(matrix[1][1],4);
//     }
// }