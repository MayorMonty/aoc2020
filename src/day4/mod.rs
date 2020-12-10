

pub fn run() -> io::Result<(usize, usize)> {   
    let lines = input::from_file::<String>(&"src/day4/input.txt", "\n")?;
    let mut trip = TobogganArea::from(items);

    // Part A
    let part_a = num_trees(3, 1, &mut trip.clone());

    // PART B, for multiple slopes
    let trees: Vec<usize> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter().map(|(x, y)| num_trees(x, y, &mut trip.clone())).collect();

    let mut part_b = 1;

    for num in trees {
        part_b *= num;
    }




    Ok((part_a, part_b))
}
