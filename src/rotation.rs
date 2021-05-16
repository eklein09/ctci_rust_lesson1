fn rotate(matrix: &mut [[i32;2];2]) -> bool {
    if matrix.len() == 0 || matrix.len() != matrix[0].len() {
        return false;
    }; // Not a square
    let n = matrix.len();
    
    for layer in 0..(n/2) {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;
            let top = matrix[first][i]; // save top

            // left -> top
            matrix[first][i] = matrix[last-offset][first]; 			

            // bottom -> left
            matrix[last-offset][first] = matrix[last][last - offset]; 

            // right -> bottom
            matrix[last][last - offset] = matrix[i][last]; 

            // top -> right
            matrix[i][last] = top; // right <- saved top
        }
    }
    true
}

fn print_matrix(m: & [[i32;2];2]) {
    println!("Rotated result: {},{},{},{}",m[0][0],m[0][1],m[1][0],m[1][1]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rotate_matrix() {
        let mut matrix = [[1,2],[3,4]];
        super::print_matrix(&matrix);
        super::rotate(&mut matrix);
        super::print_matrix(&matrix);
    }
}