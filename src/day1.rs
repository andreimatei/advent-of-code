use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

// Challenge text: https://adventofcode.com/2022/day/1
//
// You're given a file with a groups of numbers, one per line, with different
// groups separated by an empty line. Part 1 asks for the sum of numbers from
// the group with highest sum; part 2 asks for the sum of the 3 groups with the
// highest sums.

// elf_iterator takes a string comprised of groups of numbers (one per line),
// separated by empty lines, and returns an iterator over the sum of the numbers
// in each group.
//
// For example, for the input:
//  1
//  2
//
//  3
//  4
//  5
//
// it will returns an iterator over {3, 12}.
fn elf_iterator<'a>(contents: &'a str) -> impl Iterator<Item = i32>  +  'a  {
    let groups = contents.split("\n\n");
    groups.map(|g| {
        let lines = g.split("\n");
        let calories = lines.map(|s| s.parse::<i32>().unwrap()).sum();
        return calories;
    })
}

pub fn part1() {
    let contents: String = fs::read_to_string("input/day1.txt").expect("failed to read input file");
    println!("The top elf carries {} calories.", elf_iterator(contents.as_str()).max().unwrap());
}

pub fn part2() {
    let contents: String = fs::read_to_string("input/day1.txt").expect("failed to read input file");
    let calories_per_elf = elf_iterator(contents.as_str());

    // We'll insert the calories in a min-heap of size <= 3. As soon as there's
    // a 4th element, we remove the smallest one, thus maintaining the largest 3
    // elements in the heap.
    let mut heap = BinaryHeap::new();
    for calories in calories_per_elf {
        heap.push(Reverse(calories));
        if heap.len() > 3 {
            heap.pop();
        }
    }

    // Sum up the heap elements.
    let x : i32 = heap.drain().map(|x| x.0).sum();
    println!("Sum of top 3 elves: {}.", x)
}