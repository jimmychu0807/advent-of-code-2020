use crate::*;

#[test]
fn can_convert_from_a_square() {
	let input = vec!["..", ".#"];
	assert_eq!(Slope::try_from(input), Ok(Slope {
		landscape: vec![vec![0, 0], vec![0, 1]]
	}));
}

#[test]
fn fail_from_unknown_symbol() {
	let input = vec!["abc"];
	assert_eq!(Slope::try_from(input), Err("Unknown character in input"));
}

#[test]
fn slide_single_cell_0() {
	let input = vec!["."];
	let slope = Slope::try_from(input).unwrap();
	assert_eq!(slope.slide(2, 2), 0);
}

#[test]
fn slide_single_cell_1() {
	let input = vec!["#"];
	let slope = Slope::try_from(input).unwrap();
	assert_eq!(slope.slide(2, 2), 1);
}

#[test]
fn slide_small_2() {
	let input = vec!["#..", "..#"];
	let slope = Slope::try_from(input).unwrap();
	assert_eq!(slope.slide(2, 1), 2);
}
