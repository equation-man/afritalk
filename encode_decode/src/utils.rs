//! CORRESPONDENCE AND CONSTANTS.
use std::collections::*;
use std::error::Error;
use std::fmt;

pub const DEFAULT_CORRESP: [char; 77] = [
    ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C',
    'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8',
    '9', '0', ',', '?', '"', ';', ':', '(', ')', '-', '_', '[', ']', '{', '}', '.'
]; 

// specifying the number of rows for message matrix
pub const MAT_ROWS: u8 = 4;

// Message matrix default dimension.
pub const DEF_DIM: usize = 4;

pub fn encoding_test_mat() -> Vec<Vec<isize>> {
    vec![
        vec![-1, 0, -1],
        vec![2, 3, 4],
        vec![2, 4, 5],
    ]
}

/// Specifying the rule of correspondence between set of symbols such as letters of 
/// alphabet and punctuation marks from which messages are composed, and a set of integers.
pub fn gen_correspondence(tokens: &[char]) -> BTreeMap<usize, char> {
    let mut correspondence = BTreeMap::new();
    for (i, item) in tokens.iter().enumerate() {
        correspondence.insert(i, *item);
    }
    correspondence
}

/// Generating the numerical equivalence of the message.
pub fn message_numerals(msg: String, map: BTreeMap<usize, char>) -> Vec<usize> {
    let msg_chars: Vec<char> = msg.chars().collect();
    // CREATING THE NUMERICAL VERSION OF THE MESSAGE(MESSAGE VECTOR).
    msg_chars.iter()
        .filter_map(|value| {
            let k_v: Vec<(usize, char)> = map.iter()
                .filter_map(|(key, &val)| if val == *value { return Some((*key, val)); } else { return None; })
                .collect();
            if k_v[0].1 == *value {
                return Some(k_v[0].0);
            } else {
                return None;
            }
        }).collect::<Vec<usize>>()
}

/// Building nxn matrix of the message vector.
/// Working out with 4 to deal with matrices with powers of 2.
pub fn message_vector(msg_numeric: &mut Vec<usize>) -> Vec<usize> {
    // dim specifies the number of rows we want for our matrix
    let msg_matrix_residue = ((4 - (msg_numeric.len() % 4)) as isize).abs() as usize; // Calculating the matrix residue.
    if msg_matrix_residue != 4 {
        let init_residue_vals = vec![0; msg_matrix_residue as usize]; // Init residue vals with zeros.
        msg_numeric.extend(init_residue_vals); // Ext the vector with residue vals. No of elems%4==0 here.
    }
    msg_numeric.to_vec()
}

/// Converting the message vector to matrix.
fn msg_vec_to_mat(msg_vec: &mut Vec<usize>, no_of_cols: u64) -> Vec<Vec<usize>> {
    // let matrix_row_elements = msg_vec.len() / 4; // Calc no of elements per row
    msg_vec.chunks(no_of_cols.try_into().unwrap())
        .collect::<Vec<&[usize]>>()
        .into_iter().map(|item| item.to_vec())
        .collect()
}

#[derive(Debug)]
enum MatrixError {
    DimensionError(String),
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for MatrixError {}
impl From<String> for MatrixError {
    fn from(error: String) -> Self {
        MatrixError::DimensionError(error)
    }
}

/// PERFORMING MATRIX MULTIPLICATION. ROWS AND COLS ARE OF THE POWER OF 2.
/// Number of cols in matrix A must be equal to number of rows for matrix B.
fn matrix_mul_base(encoding_mat: Vec<Vec<isize>>, msg_mat: Vec<Vec<usize>>) -> Result<Vec<isize>, MatrixError> {
    let a = encoding_mat.to_vec();
    if (a.len() != msg_mat.len()) | (a[0].len() != msg_mat.len()) {
        return Err(MatrixError::DimensionError("Matrices dimensions do not match".to_string()));
    }
    Ok(vec![
        a[0][0]*(msg_mat[0][0] as isize) + a[0][1]*(msg_mat[1][0] as isize),
        a[0][0]*(msg_mat[0][1] as isize) + a[0][1]*(msg_mat[1][1] as isize),
        a[1][0]*(msg_mat[0][0] as isize) + a[1][1]*(msg_mat[1][0] as isize),
        a[1][0]*(msg_mat[0][1] as isize) + a[1][1]*(msg_mat[1][1] as isize),
    ])
}

/// SPLITTING THE MATRIX. CONSIDER WHEN ROWS AND COLS ARE OF THE POWER OF 2. FOR STRASSEN.
fn splitter(matrix: Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>){
    let n = matrix.len();
    (matrix[n % n].to_vec(), matrix[(n % n) + 1].to_vec(),
     matrix[(n % n) + 2].to_vec(), matrix[(n % n) + 3].to_vec())
}

/// DOING A MATRIX TRANSPOSE.
fn transpose(matrix: &mut Vec<usize>, n_cols: u64) -> Vec<usize> {
    let mut min_range_index = 1;
    let mut no_of_cols = n_cols;
    let mut row;
    for l in 2..n_cols {
        for k in 1..no_of_cols {
            row = min_range_index + k*(n_cols-1);
            matrix.swap(row.try_into().unwrap(), min_range_index.try_into().unwrap());
            min_range_index += 1;
        }
        min_range_index += l;
        no_of_cols -= 1;
    }
    matrix.to_vec()
}

pub fn matrix_mul(encoding_mat: Vec<usize>, msg_mat: Vec<usize>) -> Vec<usize> {
    let mut encoded_mat = Vec::with_capacity(msg_mat.len());
    let no_of_cols = 1;
    let mut c = 0;
    for j in 0..(msg_mat.len()/no_of_cols)-1 {
        for i in 0..msg_mat.len() {
            c += encoding_mat[i]*(msg_mat[i] as usize);
            if (i+1) % no_of_cols == 0 {
                encoded_mat.push(c);
                c = 0;
                continue;
            }
        }
    }
    encoded_mat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_split() {
        let matrix_to_split = vec![
            &[3 as usize, 1 as usize, 0 as usize, 2 as usize],
            &[4 as usize, 8 as usize, 5 as usize, 9 as usize],
            &[2 as usize, 1 as usize, 6 as usize, 7 as usize],
            &[0 as usize, 9 as usize, 5 as usize, 2 as usize],
        ];
        //let (a, b, c, d) = splitter(matrix_to_split);
        //let mat_mul = mul_matrix();
        //println!("Matrices are {:?}, {:?}, {:?}, {:?}", a, b, c, d);
    }

    #[test]
    fn test_message_matrix() {
        pub const TEST_MAT_A: [[isize; 2]; 2] = [
            [-1, 0],
            [2, 3],
        ];
        let TEST_MAT_B: Vec<&[usize]> = vec![
            &[1, 1],
            &[3, 2],
        ];
        let encoding_mat = encoding_test_mat();
        let corresp = gen_correspondence(&DEFAULT_CORRESP);
        let msg = "Toda".to_string();
        let mut msg_numerals = message_numerals(msg, corresp);
        let mut msg_mat = message_vector(&mut msg_numerals);
        let mut dummy_encoder_mat = message_vector(&mut msg_numerals);
        transpose(&mut msg_mat, 8);
        let msg_to_vec = msg_vec_to_mat(&mut msg_mat, 8);
        //println!("{}, {}, {} are powers of 2", 2u64.count_ones(), 4u64.count_ones(), 8u8.count_ones());
        println!("The message matrix is {:?}\n", msg_vec_to_mat(&mut dummy_encoder_mat, 8));
        println!("The transpose message matrix is len of msg mat is {}, {:?}\n",msg_to_vec.len(), msg_to_vec);
        let mut mat_mul = matrix_mul(dummy_encoder_mat, msg_mat);
        println!("The matrix multiplication is the len is {} {:?}", mat_mul.len(), msg_vec_to_mat(&mut mat_mul, 8));
        //let mat_mul = matrix_mul_base(&TEST_MAT_A, TEST_MAT_B).unwrap();
        //println!("The mat mul is {:?}", mat_mul);
    }

    #[test]
    fn test_correspondence() {
        let corresp = gen_correspondence(&DEFAULT_CORRESP);
        assert_eq!(corresp.get(&0), Some(' ').as_ref());
        assert_eq!(corresp.get(&1), Some('a').as_ref());
        assert_eq!(corresp.get(&2), Some('b').as_ref());
        assert_eq!(corresp.get(&3), Some('c').as_ref());
        assert_eq!(corresp.get(&4), Some('d').as_ref());
        assert_eq!(corresp.get(&5), Some('e').as_ref());
        assert_eq!(corresp.get(&6), Some('f').as_ref());
        assert_eq!(corresp.get(&7), Some('g').as_ref());
        assert_eq!(corresp.get(&8), Some('h').as_ref());
        assert_eq!(corresp.get(&9), Some('i').as_ref());
        assert_eq!(corresp.get(&10), Some('j').as_ref());
        assert_eq!(corresp.get(&11), Some('k').as_ref());
        assert_eq!(corresp.get(&12), Some('l').as_ref());
        assert_eq!(corresp.get(&13), Some('m').as_ref());
        assert_eq!(corresp.get(&14), Some('n').as_ref());
        assert_eq!(corresp.get(&15), Some('o').as_ref());
        assert_eq!(corresp.get(&16), Some('p').as_ref());
        assert_eq!(corresp.get(&17), Some('q').as_ref());
        assert_eq!(corresp.get(&18), Some('r').as_ref());
        assert_eq!(corresp.get(&19), Some('s').as_ref());
        assert_eq!(corresp.get(&20), Some('t').as_ref());
        assert_eq!(corresp.get(&21), Some('u').as_ref());
        assert_eq!(corresp.get(&22), Some('v').as_ref());
        assert_eq!(corresp.get(&23), Some('w').as_ref());
        assert_eq!(corresp.get(&24), Some('x').as_ref());
        assert_eq!(corresp.get(&25), Some('y').as_ref());
        assert_eq!(corresp.get(&26), Some('z').as_ref());
        assert_eq!(corresp.get(&27), Some('A').as_ref());
        assert_eq!(corresp.get(&28), Some('B').as_ref());
        assert_eq!(corresp.get(&29), Some('C').as_ref());
        assert_eq!(corresp.get(&30), Some('D').as_ref());
        assert_eq!(corresp.get(&31), Some('E').as_ref());
        assert_eq!(corresp.get(&32), Some('F').as_ref());
        assert_eq!(corresp.get(&33), Some('G').as_ref());
        assert_eq!(corresp.get(&34), Some('H').as_ref());
        assert_eq!(corresp.get(&35), Some('I').as_ref());
        assert_eq!(corresp.get(&36), Some('J').as_ref());
        assert_eq!(corresp.get(&37), Some('K').as_ref());
        assert_eq!(corresp.get(&38), Some('L').as_ref());
        assert_eq!(corresp.get(&39), Some('M').as_ref());
        assert_eq!(corresp.get(&40), Some('N').as_ref());
        assert_eq!(corresp.get(&41), Some('O').as_ref());
        assert_eq!(corresp.get(&42), Some('P').as_ref());
        assert_eq!(corresp.get(&43), Some('Q').as_ref());
        assert_eq!(corresp.get(&44), Some('R').as_ref());
        assert_eq!(corresp.get(&45), Some('S').as_ref());
        assert_eq!(corresp.get(&46), Some('T').as_ref());
        assert_eq!(corresp.get(&47), Some('U').as_ref());
        assert_eq!(corresp.get(&48), Some('V').as_ref());
        assert_eq!(corresp.get(&49), Some('W').as_ref());
        assert_eq!(corresp.get(&50), Some('X').as_ref());
        assert_eq!(corresp.get(&51), Some('Y').as_ref());
        assert_eq!(corresp.get(&52), Some('Z').as_ref());
        assert_eq!(corresp.get(&53), Some('1').as_ref());
        assert_eq!(corresp.get(&54), Some('2').as_ref());
        assert_eq!(corresp.get(&55), Some('3').as_ref());
        assert_eq!(corresp.get(&56), Some('4').as_ref());
        assert_eq!(corresp.get(&57), Some('5').as_ref());
        assert_eq!(corresp.get(&58), Some('6').as_ref());
        assert_eq!(corresp.get(&59), Some('7').as_ref());
        assert_eq!(corresp.get(&60), Some('8').as_ref());
        assert_eq!(corresp.get(&61), Some('9').as_ref());
        assert_eq!(corresp.get(&62), Some('0').as_ref());
        assert_eq!(corresp.get(&63), Some(',').as_ref());
        assert_eq!(corresp.get(&64), Some('?').as_ref());
        assert_eq!(corresp.get(&65), Some('"').as_ref());
        assert_eq!(corresp.get(&66), Some(';').as_ref());
        assert_eq!(corresp.get(&67), Some(':').as_ref());
        assert_eq!(corresp.get(&68), Some('(').as_ref());
        assert_eq!(corresp.get(&69), Some(')').as_ref());
        assert_eq!(corresp.get(&70), Some('-').as_ref());
        assert_eq!(corresp.get(&71), Some('_').as_ref());
        assert_eq!(corresp.get(&72), Some('[').as_ref());
        assert_eq!(corresp.get(&73), Some(']').as_ref());
        assert_eq!(corresp.get(&74), Some('{').as_ref());
        assert_eq!(corresp.get(&75), Some('}').as_ref());
        assert_eq!(corresp.get(&76), Some('.').as_ref());
    }
}
