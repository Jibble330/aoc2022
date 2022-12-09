fn main() {
    println!("Day 1 Part 1: {}", one1());
    println!("Day 1 Part 2: {}", one2());

    println!();

    println!("Day 2 Part 1: {}", two1());
    println!("Day 2 Part 2: {}", two2());

    println!();

    println!("Day 3 Part 1: {}", three1());
    println!("Day 3 Part 2: {}", three2());

    println!();

    println!("Day 4 Part 1: {}", four1());
    println!("Day 4 Part 2: {}", four2());

    println!();

    println!("Day 5 Part 1: {}", five1());
    println!("Day 5 Part 2: {}", five2());

    println!();

    println!("Day 6 Part 1: {}", six1());
    println!("Day 6 Part 2: {}", six2());

    println!();

    println!("Day 7 Part 1: {}", seven1());
    println!("Day 7 Part 2: {}", seven2());

    println!();

    println!("Day 8 Part 1: {}", eight1());
    println!("Day 8 Part 2: {}", eight2());

}

fn input_to_string(day: u8) -> String {
    std::fs::read_to_string(format!("input/in{}.txt", day)).expect("Unable to read input file")
}

/// Day 1 Part 1
fn one1() -> usize {
    let input = input_to_string(1);

    let mut i = 0;
    let mut elves = vec![0];

    for number in input.lines() {
        if number == "" {
            i += 1;
            elves.push(0);
            continue
        }
        elves[i] += number.parse::<usize>().unwrap()
    }

    *elves.iter().max().unwrap()
}

/// Day 1 Part 2
fn one2() -> usize {
    let input = input_to_string(1);

    let mut i = 0;
    let mut elves = vec![0];
    for number in input.lines() {
        if number == "" {
            i += 1;
            elves.push(0);
            continue;
        }
        elves[i] += number.parse::<usize>().unwrap();
    }


    let first = *elves.iter().max().unwrap();
    elves.remove(elves.iter().position(|x| *x == first).unwrap());
    let second = *elves.iter().max().unwrap();
    elves.remove(elves.iter().position(|x| *x == second).unwrap());
    let third = *elves.iter().max().unwrap();
    elves.remove(elves.iter().position(|x| *x == third).unwrap());

    first+second+third
}

/// Day 2 Part 1
fn two1() -> usize {
    let input = input_to_string(2);
    let mut total = 0;

    for line in input.lines() {
        let (op, me) = (line.split_at(1).0, line.split_at(2).1);
        let opponent = Shape::from(op).unwrap();
        let you = Shape::from(me).unwrap();

        total += you.play(&opponent).score() + you.score();
    }

    total
}

/// Day 2 Part 2
fn two2() -> usize {
    let input = input_to_string(2);
    let mut total = 0;

    for line in input.lines() {
        let (op, me) = (line.split_at(1).0, line.split_at(2).1);
        let opponent = Shape::from(op).unwrap();
        let round = Round::from(me).unwrap();

        let you = opponent.result(&round);

        total += you.score() + round.score();
    }

    total
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from(letter: &str) -> Option<Self> {
        match letter {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            "X" => Some(Shape::Rock),
            "Y" => Some(Shape::Paper),
            "Z" => Some(Shape::Scissors),
            _ => None
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    fn play(&self, opponent: &Shape) -> Round {
        if self == opponent {
            return Round::Draw
        }
        if (*self == Self::Rock && *opponent == Self::Scissors) || (*self == Self::Paper && *opponent == Self::Rock) || (*self == Self::Scissors && *opponent == Self::Paper) {
            Round::Win
        } else {
            Round::Loss
        }
    }

    fn result(&self, end: &Round) -> Shape {
        match end {
            Round::Loss => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper
            },
            Round::Draw => *self,
            Round::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock
            }
        }
    }
}

enum Round {
    Win,
    Draw,
    Loss
}

impl Round {
    fn from(letter: &str) -> Option<Self> {
        match letter {
            "X" => Some(Self::Loss),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0
        }
    }
}

/// Day 3 Part 1
fn three1() -> usize {
    let input = input_to_string(3);
    let mut total = 0;

    'outer: for line in input.lines() {
        let sides = line.split_at(line.len()/2);

        for c in sides.0.chars() {
            if sides.1.contains(c) {
                total += priority(c);
                continue 'outer;
            }
        }
    }

    total
}

/// Day 3 Part 2
fn three2() -> usize {
    let input = input_to_string(3);
    let mut total = 0;

    let lines: Vec<&str> = input.lines().collect();

    'outer: for i in (0..lines.len()).step_by(3) {
        for c in lines[i].chars() {
            
            if lines[i+1].contains(c) && lines[i+2].contains(c) {
                total += priority(c);
                continue 'outer;
            }
        }
        
    }

    total
}

fn priority(c: char) -> usize {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(c).unwrap() + 1
}

/// Day 4 Part 1
fn four1() -> usize {
    let input = input_to_string(4);
    let mut total = 0;

    for line in input.lines() {
        let pair_string = line.split_once(",").unwrap();
        let pair_strings = (pair_string.0.split_once("-").unwrap(), pair_string.1.split_once("-").unwrap());

        let pair = ((pair_strings.0.0.parse::<usize>().unwrap(), pair_strings.0.1.parse::<usize>().unwrap()), (pair_strings.1.0.parse::<usize>().unwrap(), pair_strings.1.1.parse::<usize>().unwrap()));
        if contains(pair.0, pair.1) {
            total += 1;
        }
    }

    total
}

fn contains(r1: (usize, usize), r2: (usize, usize)) -> bool {
    (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1)
}

/// Day 4 Part 2
fn four2() -> usize {
    let input = input_to_string(4);
    let mut total = 0;

    for line in input.lines() {
        let pair_string = line.split_once(",").unwrap();
        let pair_strings = (pair_string.0.split_once("-").unwrap(), pair_string.1.split_once("-").unwrap());

        let pair = ((pair_strings.0.0.parse::<usize>().unwrap(), pair_strings.0.1.parse::<usize>().unwrap()), (pair_strings.1.0.parse::<usize>().unwrap(), pair_strings.1.1.parse::<usize>().unwrap()));
        if overlaps(pair.0, pair.1) {
            total += 1;
        }
    }

    total
}

fn overlaps(r1: (usize, usize), r2: (usize, usize)) -> bool {
    let range1 = r1.0..=r1.1;
    let range2 = r2.0..=r2.1;
    range1.contains(&r2.0) || range1.contains(&r2.1) || range2.contains(&r1.0) || range2.contains(&r1.1)
}

/// Day 5 Part 1
fn five1() -> String {
    let input_str = input_to_string(5);
    let input: Vec<&str> = input_str.lines().collect();

    let space = input.iter().position(|x| *x == "").unwrap();
    let stack_amount = input[space-1].split(" ").filter(|x| *x != "").collect::<Vec<&str>>().len();

    let mut stacks: Vec<Vec<&str>> = Vec::new();
    for _ in 0..stack_amount {
        stacks.push(Vec::new());
    }

    for line in input[..space-1].iter() {
        for i in 0..stack_amount {
            let piece = line.split_at(i*4+1).1.split_at(1).0;
            if piece != " " {
                stacks[i].insert(0, piece);
            }
        }
    }

    for line in input[(space+1)..].iter() {
        let params: Vec<usize> = line.replace("move ", "").replace("from ", "").replace("to ", "").splitn(3, " ").map(|x| x.parse::<usize>().unwrap()).collect();
        let amount = params[0];

        for _ in 0..amount {
            let from = stacks[params[1]-1].pop().unwrap();
            stacks[params[2]-1].push(from);
        }
    }

    tops(&stacks)
}

/// Day 5 Part 2
fn five2() -> String {
    let input_str = input_to_string(5);
    let input: Vec<&str> = input_str.lines().collect();
    let space = input.iter().position(|x| *x == "").unwrap();

    let stack_amount = input[space-1].split(" ").filter(|x| *x != "").collect::<Vec<&str>>().len();
    let mut stacks: Vec<Vec<&str>> = Vec::new();
    for _ in 0..stack_amount {
        stacks.push(Vec::new());
    }

    for line in input[..space-1].iter() {
        for i in 0..stack_amount {
            let piece = line.split_at(i*4+1).1.split_at(1).0;
            if piece != " " {
                stacks[i].insert(0, piece);
            }
        }
    }

    for line in input[(space+1)..].iter() {
        let params: Vec<usize> = line.replace("move ", "").replace("from ", "").replace("to ", "").splitn(3, " ").map(|x| x.parse::<usize>().unwrap()).collect();
        let amount = params[0];

        let from = (*stacks[params[1]-1].split_at(stacks[params[1]-1].len()-amount).1).to_vec(); // hacked together to appease the borrow checker
        stacks[params[1]-1] = stacks[params[1]-1].split_at(stacks[params[1]-1].len()-amount).0.to_vec();
        stacks[params[2]-1].append(&mut from.to_vec());
    }

    tops(&stacks)
}

fn tops(stacks: &Vec<Vec<&str>>) -> String {
    let mut all = "".to_string();
    for stack in stacks {
        all += stack.last().unwrap();
    }

    all
}

/// Day 6 Part 1
fn six1() -> usize {
    let input_chars: Vec<char> = input_to_string(6).chars().collect();
    let marker = 4;

    for c in 0..input_chars.len()-marker {
        let mut set: Vec<char> = Vec::new();
        for i in 0..marker {
            set.push(input_chars[c+i]);
        }
        if is_unique(&set) {
            return c+marker
        }
    }

    0
}

fn six2() -> usize {
    let input_chars: Vec<char> = input_to_string(6).chars().collect();
    let marker = 14;

    for c in 0..input_chars.len()-marker {
        let mut set: Vec<char> = Vec::new();
        for i in 0..marker {
            set.push(input_chars[c+i]);
        }
        if is_unique(&set) {
            return c+marker
        }
    }

    0
}

fn is_unique(chars: &Vec<char>) -> bool {
    for (i, c) in chars.iter().enumerate() {
        for (check_i, check_c) in chars.iter().enumerate() {
            if c == check_c && i != check_i {
                return false
            }
        }
    }

    true
}

/// Day 7 Part 1
fn seven1() -> usize {
    let input = input_to_string(7);
    
    let mut index: Vec<usize> = Vec::new();
    let mut tree = Tree::new("/");
    for line in input.lines().skip(1) {
        if line.starts_with("$ cd ") {
            let name = line.replace("$ cd ", "");
            if name == ".." {
                index.pop();
            } else {
                tree.index(&mut index, &name);
            }
        } else if line.starts_with("dir ") {
            tree.add(Item::Dir(Tree::new(&line.replace("dir ", ""))), &index);
        } else if !line.starts_with("$") {
            let size_str = line.split_once(" ").unwrap().0;
            let size: usize = size_str.parse().unwrap();

            tree.add(Item::File(File::new(size)), &index);
        }
    }    

    tree.sized_below(100_000)
}

/// Day 7 Part 2
fn seven2() -> usize {
    let input = input_to_string(7);
    
    let mut index: Vec<usize> = Vec::new();
    let mut tree = Tree::new("/");
    for line in input.lines().skip(1) {
        if line.starts_with("$ cd ") {
            let name = line.replace("$ cd ", "");
            if name == ".." {
                index.pop();
            } else {
                tree.index(&mut index, &name);
            }
        } else if line.starts_with("dir ") {
            tree.add(Item::Dir(Tree::new(&line.replace("dir ", ""))), &index);
        } else if !line.starts_with("$") {
            let size_str = line.split_once(" ").unwrap().0;
            let size: usize = size_str.parse().unwrap();

            tree.add(Item::File(File::new(size)), &index);
        }
    }    

    tree.min(tree.size()-(70_000_000-30_000_000))
}

enum Item {
    Dir(Tree),
    File(File)
}

struct Tree {
    name: String,
    files: Vec<File>,
    children: Vec<Tree>
}

impl Tree {
    fn new(name: &str) -> Self {
        Self{name: name.to_string(), files: Vec::new(), children: Vec::new()}
    }

    fn index(&mut self, index: &mut Vec<usize>, name: &str) {
        let mut current = self;

        for i in index.iter() {
            current = &mut current.children[*i];
        }

        index.push(current.children.iter().position(|x| x.name == name).unwrap());
    }

    fn add(&mut self, item: Item, index: &Vec<usize>) {
        let mut current = self;
        for i in index {
            current = &mut current.children[*i];
        }

        match item {
            Item::Dir(d) => current.children.push(d),
            Item::File(f) => current.files.push(f)
        }
    }

    fn size(&self) -> usize {
        let mut total = 0;

        for f in self.files.iter() {
            total += f.size;
        }
        for d in self.children.iter() {
            total += d.size();
        }

        total
    }

    fn sized_below(&self, max: usize) -> usize {
        let mut total = 0;

        for d in self.children.iter() {
            let size = d.size();
            if size <= max {
                total += size;
            }
            total += d.sized_below(max);
        }
        
        total
    }

    fn all_sizes(&self) -> Vec<usize> {
        let mut all: Vec<usize> = Vec::new();

        for d in self.children.iter() {
            all.push(d.size());
            all.append(&mut d.all_sizes());
        }

        all
    }
    
    fn min(&self, min: usize) -> usize {
        let all = self.all_sizes();
        let mut found = *all.iter().find(|x| *x >= &min).unwrap();

        for d in all.iter() {
            if *d >= min {
                found = found.min(*d);
            }
        }

        found
    }
}

struct File {
    size: usize
}

impl File {
    fn new(size: usize) -> Self {
        Self{size}
    }
}

/// Day 8 Part 1
fn eight1() -> usize {
    let input = input_to_string(8);
    let grid = Forest::parse(input);

    let mut total = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.visible(x, y) {
                total += 1;
            }
        }
    }

    total
}

/// Day 8 Part 2
fn eight2() -> usize {
    let input = input_to_string(8);
    let grid = Forest::parse(input);
    let mut max = 0;
    
    for x in 0..grid.width {
        for y in 0..grid.height {
            max = max.max(grid.viewing_distance(x, y));
        }
    }

    max
}

struct Forest {
    inner: Vec<Vec<usize>>,
    width: usize,
    height: usize
}

impl Forest {
    fn new(inner: Vec<Vec<usize>>) -> Self {
        let height = inner.len();
        let width = inner[0].len();
        Self{
            inner,
            height,
            width
        }
    }

    fn parse(grid: String) -> Self {
        let mut full: Vec<Vec<usize>> = Vec::new();

        for line in grid.lines() {
            let mut temp: Vec<usize> = Vec::new();

            for c in line.chars() {
                temp.push(c.to_digit(10).unwrap() as usize);
            }

            full.push(temp);
        }

        Self::new(full)
    }

    fn at(&self, x: usize, y: usize) -> usize {
        self.inner[y][x]
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        let height = self.at(x, y);

        let mut up = true;
        let mut down = true;
        let mut left = true;
        let mut right = true;

        for row in 0..y {
            if self.at(x, row) >= height {
                up = false;
                break;
            }
        }
        for row in y+1..self.height {
            if self.at(x, row) >= height {
                down = false;
                break;
            }
        }

        for column in 0..x {
            if self.at(column, y) >= height {
                left = false;
                break;
            }
        }
        for column in x+1..self.width {
            if self.at(column, y) >= height {
                right = false;
                break;
            }
        }

        up || down || left || right
    }

    fn viewing_distance(&self, x: usize, y: usize) -> usize {
        if x == 0 || y == 0 || x == self.width-1 || y == self.height-1 {
            return 0
        }
        
        let height = self.at(x, y);

        let mut up = 0;
        let mut down = 0;
        let mut left = 0;
        let mut right = 0;

        for row in (0..y).rev() {
            up += 1;
            if self.at(x, row) >= height {
                break
            }
        }
        for row in y+1..self.height {
            down += 1;
            if self.at(x, row) >= height {
                break;
            }
        }
        for column in (0..x).rev() {
            left += 1;
            if self.at(column, y) >= height {
                break;
            }
        }
        for column in x+1..self.width {
            right += 1;
            if self.at(column, y) >= height {
                break;
            }
        }
        
        up*down*left*right
    }
}

/// Day 9 Part 1
fn nine1() -> usize {
    
}

struct Vec2 {
    x: usize,
    y: usize
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Self{x, y}
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn adjacent(&self, other: &Vec2) -> bool {
        (self.x-1..=self.x+1).contains(&other.x) && (self.y-1..=self.y+1).contains(&other.y)
    }

    fn direction(&self, other: &Vec2) -> Direction {
        match *self-*other {
            v if v.x == 0 && v.y == 0 => Direction::Center,
            v if v.x == 0 && v.y > 0 => Direction::Up,
            v if v.x == 0 && v.y < 0 => Direction::Down,
            v if v.y == 0 && v.x > 0 => Direction::Right,
            v if v.y == 0 && v.x < 0 => Direction::Left,
            v if v.y > 0 && v.x > 0 => Direction::UpperRight,
            v if v.y > 0 && v.x < 0 => Direction::UpperLeft,
            v if v.y < 0 && v.x > 0 => Direction::LowerRight,
            v if v.y < 0 && v.x < 0 => Direction::LowerLeft,
            _ => Direction::Center
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output{x: self.x-rhs.x, y: self.y-rhs.y}
    }
}

enum Direction {
    Center,
    Up,
    Down,
    Right,
    Left,
    UpperRight,
    UpperLeft,
    LowerRight,
    LowerLeft
}
