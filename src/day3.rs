#[derive(Clone, Copy)]
struct Slope {
	right: usize,
	down: usize,
}

impl Slope {
	fn new(right: usize, down: usize) -> Self {
		Slope { right: right, down: down }
	}
}

fn count_trees(slope: &Slope) -> usize {
	let mut trees = 0;
	let mut x = 0;
	let mut y = 0;
	while y < INPUT_LINE_COUNT - slope.down {
		if INPUT.as_bytes()[y * (INPUT_LINE_LENGTH + 1) + x] == b'#' {
			trees += 1
		}
		y += slope.down;
		x += slope.right;
		if x >= INPUT_LINE_LENGTH {
			x -= INPUT_LINE_LENGTH;
		}
	}
	trees
}

pub fn run() -> (usize, usize) {
	let trees =
		[Slope::new(1, 1), Slope::new(3, 1), Slope::new(5, 1),
			Slope::new(7, 1), Slope::new(1, 2)]
		.map(|s| count_trees(&s));
	(trees[1], trees.iter().product())
}

const INPUT_LINE_COUNT: usize = 323;
const INPUT_LINE_LENGTH: usize = 31;
const INPUT: &'static str = r".......#................#......
...#.#.....#.##.....#..#.......
..#..#.#......#.#.#............
....#...#...##.....#..#.....#..
....#.......#.##......#...#..#.
...............#.#.#.....#..#..
...##...#...#..##.###...##.....
##..#.#...##.....#.#..........#
.#....#..#..#......#....#....#.
...........................#...
..........#.......#..#.....#.#.
..#.......###..#.#.......#.#...
....#..#....#....#..........#..
..##..#.......#.#...#..........
.....#.......#.....#....#......
..........##..#................
....##.#..###...#..##.....#.#..
..#..#.#.#...#......#...#.....#
....#.#....#...####.##.........
..#.........##...##.#..#..#....
.#......#...#..#..##.#.........
.#....#.......#..##..##..#.#.#.
...........#....#......#.......
..#....#....#...............#..
..#.....#....###.##.....#.#..#.
#..........#.#......#.#....#...
....###...#.#.....#....#.####.#
........#......#...#...#..##..#
...##..............##.#.......#
#..........#...........#.#....#
#...#....#..####..#............
###....#........#..............
...#.##....................#.##
...#..#.....#.....##...#....#..
.......###.#...#.........#.....
.#..#.....#.#..#.....#.........
#................#.............
...#......#.#.....##.#.#....#..
...#..#.#..#.....#...#....#....
.......#......#........#.....#.
.#.##..##.....#.#......#.#.#...
#...............#.....#....#...
.....#...........#..##.........
.....#..#........##..#..#.....#
..###.#.#.......#.#...........#
##....##....#.#....##...#.##.##
..................##.#.#.....#.
.#...........###...#...........
.#.#....#......#....###.#......
.......#.##...#...#..#.#.......
..#.....#.#....#..#............
.....#..#..#....#..#.........#.
..##.#......#.....#...#.#..#.#.
.........#......#....##.......#
#........#..#.#......#...#.#..#
...#....#.#..#....##.......###.
..#...#......#.##..........#...
........#..#..#...#.......#....
.##.#..#...#..#........#.#.####
#..#..#..........#....##...#...
....#...#........##........#...
.#......#.......#..#..#........
#...#.#......#....#............
#........#..##.#...##..........
...#..##.....#......##.#..#.#..
.#.#.....#.....#.####.#..##....
..........###....#.##...#......
.......#.......#..#.#.#.##.#..#
..#.#....#......#.#...#.......#
.#...#....#......#...#.........
.#....#..#....#.##.#....#..##..
...#..#.#..................#...
.##..#.............##.........#
...#.#.#................#.....#
...###..###..................#.
........##.##..#.#...#.....#...
.##...##...#...#....#...#......
#..#....#..#..#.#....#..####...
.#...............##....##.#....
#..#................#...#..#...
.#....#.....#..#.#........#....
...............##.#..##..##....
.#......#........#....#.#...#.#
.#.....#...##.#........#.##.#.#
..###............#..#.#....#...
..#.....#.........#....#..#.#..
.##.....#.#..........#.#....##.
...#...#....#..#......#.#.#..#.
#.....#...#....##...#.......##.
.......#.#.........##..........
............##.#.##...#.......#
.....#........##...#........#..
.#........#.#.#.#....#.........
#....#..#....#.#..#...#.#......
....##...........#...#...##.#.#
......#...##.###.....#.........
............#..##....##......#.
......##....#...#.#....#......#
#..#..#..#.#.#.........#...##.#
...#.........#...#.........##.#
#.#.....#.......#.##..#..#.....
##................#......#....#
....#..#.......#....##.....#...
.....#..#...#...#......#.#....#
..#....#.....#.........#.....#.
..#..#..........#.....#........
.......#..##.#......#.#........
.............##.....#....#.....
...#....#..#.#.#...............
........#....##..#...#........#
..##...............#.....#....#
........##.#.##.#......#..#....
..#.##.......#..........##..#..
.#..............#.#.##.........
.#.......#....#....#.#.#.......
.#.##.......#....#......###.#..
.......#...#............##.....
........#.#..........##..#.....
...###..#......#.....##..#..#..
...........##......#....#......
..............#....#..#..#.#..#
....#...#......#.##...#........
.#.............#..#......###.#.
#...#..#.#..............##..#.#
....................#.........#
..##..#......#.###.....#...#.#.
.#....#.#........#...#........#
..#....#.....#..............#..
##..........#..#..#...#........
...........#..##...#.......#...
........##.............#.......
#....#........#..#.#.###..#....
...........##..........##......
#......#.....##.#.##......##...
..#......#.........#.......#..#
......#.#....##..##.#...#.#...#
......#..................##....
...#....#.#...#.#.......##.....
#.#...##...##........#...##....
..#.......#.#.#...#............
.......#......#..#...#.........
#...#..#...........##..........
......#....#.........#.#....#..
#......#........#...#..##....#.
.....#.......##..#.#......#..#.
...........#......#...#......#.
#.#.##.....#....#.....##......#
.....##..#.#.#.###........#.#..
...#...#.#......#......#.......
......###....#..##...#.#.##....
#.....#.....#..................
...#...#......#...............#
..#............##..#.....#.....
.#....#...#...#...#...#..#.....
.##......#.........#.###.#.....
#.#.##.......##...#........##.#
.##.#.#......#.....#...#.....#.
....####.##.......#..##..##.#..
#.#.......#..##....###..#...#..
..#..#....#...#.#.#.#...#......
##.........#.##................
........#.....................#
..#...........#..#..##.#..#.#..
#...#...................#.###..
##..#............#.........#..#
...............##...#...##....#
#.#.....#..#.......#......#....
.#...#......#............#.....
#.......#...#..#....#.......#..
...#....#.##.#....#....#.#.....
...#..#..............#..#.#..#.
.........#.....#.#...#..#....#.
..#..#..#...##.....##.#.....#..
.#.#..........#........#.......
...............#........#.#.#..
.#......#.....#..............#.
........#.#..............#.#...
.......#.#....#..#.#.#..#.#.##.
...##..#...#.#..#...........#..
#...###.#.....#..#........#....
.#...##...##...##.#.....###....
.........#......#.#..##.#.#....
#....#.#..#...#.#.#....#..#..#.
.#.#...#......###.....#........
#.....#.#.......#..#.#...#.....
.................#.#....#..##..
#...........#....###..#......#.
##.#..#....#.#.#.#.............
#.....#..#...#........#........
..#..#......#..#.##.#..........
...#....#..#..........#.#.##.##
#........#...#.......#..##.#...
.#.#..#....#.#....#......#.....
##.......##.#........#...#..##.
##.##.....#.......#####.#....#.
..#..###.#.#..#....###..#.##..#
#.........#.............#.#...#
..#...##.#..................#..
.....#.#....#.#..#.#........#.#
......#.......#.#..##.#.#..#...
..#......#.#..##......#..#....#
..##..#..#.##.#..#....#...##...
###....#...##....##.........#..
#........##.........#......#..#
...#.........#......#.##.......
.....#.#.#....#......#.........
..#...........#....#......#.#..
##........#...##.....######....
....#..#..##.......#..#..#.....
..#....#..##....#......##....#.
...##....#........##......#....
.#.#...###...#......#..........
#....#..#.##.........#...#.....
......#..#.........#.##.....#..
...#............##....#......#.
...#.....##.....#........#.#..#
......#.#..#......#.....#..##..
#.#.........##..........#......
..###.....#..#....##..........#
.............##..#....#..##....
....#.#....##..#......#...#....
....###.....#..#.......#.......
............#..#...............
......#........#..#......#.....
.#........#.......#.##.......#.
..#.........#..#.#.....##....#.
...#.......#.......#.......##.#
#......##.#.....#......##.#..#.
#..........#.................#.
....#..##...........#.....#.#..
#.###...#............#.#....#.#
....#......#.#..###....##..#...
....#...#..........##..........
..#.#............#...#...###...
......#...#......#..#.#........
.#.......#..#...........##...#.
##...#...##....##.#..#..#.#....
.......#........#............##
.#......#...#.#................
#.#........#.#....#..#.##......
.......#.#...#....##.......##..
........#.#.#.........##..##...
..##...............#.#.###.#...
......#.#....#..#......##.....#
###.........#.....#.#.....##...
.#.#....#.....#.#.##..#.......#
..#..#.#......#...##..##.#..#..
...#........#..#....#..........
#...#.#...#..##....##..........
.........#........#.##....#..#.
..#...#.#.......##..........##.
###...........##.#......#.#..#.
...#....#...#..#..#......#.....
.....##.......###.#....###..##.
...#...#..........#.#......#...
....#.....##...##..#.#........#
.....#...#..#.....##...##....#.
................##.#.##....##.#
.#..#..#....#.....#....#..#...#
.....###.....#.................
#...#..##..#.........#.........
.....#..#................#.....
.#..#...#......#..#............
...#...#.#....#....##...#...##.
..........#....#.#..#.#.....#..
....#...###.##...#..#..#......#
#...#.......#..........#..#....
.#............#..##.......#...#
....#..#...#............#..#.#.
.#....#.......#..#.#......#....
...#...#............#...#.....#
....#.#.#..##.#.....#...#.#....
......#.#.#......#..#...#.....#
......##.....#.............#...
..#...#..#.#....#..............
.#.#..#....#.#..##....###.##...
..#...........#....#.###.#....#
.....#.........#.#.............
...#.#.....#......###......##..
...#...#.....#.................
...#..#...##.....##.........#..
..#...#..#..##..#...#........#.
##..#.#.##.#....#...........#..
.......#....##....#...##..#..#.
#.......##.#...##...##..#.....#
....#.#...............#......#.
....#.#...#.....#....#......#..
.#.........#.#....###........#.
.#.#.....#.....#.#.#....#.#....
............#...........#.#..##
#...#......#..#......#.#.......
...#.#.#.....#..#...#..##......
...#.#..#...#....#.........#.#.
........#..#......##.....#...#.
...#..#..............#..#......
.........#.......#...#......#..
.#......#.....#.....#......#...
......#.......#....#...#.#.....
.#.....#.##..#........#...#....
#.....##..##....#.#.......#..#.
.#..#...#..#.......#...........
..#..#...#.....##....#.....#...
#.#..............#....#..#.....
.........##...#......#.##...##.
.###...#.#...#.....#.........#.
.....#..........##...#..#....##
.#..#......#....##.#...#.......
.............###.#.#..#.#.#...#
.......#...##..#..#.....###....
##.......#...........#....#.#..
##......#...#.#................
.#.####..##.#...............#..
..#...#.#.#..#...#........#...#
.##..##.##.....#.......#..#.#..
...................#......#.#..
#.##..#..........#.............
##..#......#....#.#............
.#........#.....##...#.........
.##....#..#..##..........#...#.
#..........##........#..#..#.#.
####.###.#.....#....#..#.#....#
..#...#...#.#.......#....#...#.
......##.###..##.#.###......#.#";