use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut elf_energy = 0;
    let mut sum_vec = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt"){
        for line in lines{
            if let Ok(energy) = line{
                if energy.is_empty(){
                    sum_vec.push(elf_energy);
                    elf_energy = 0;
                    continue
                }
                if let Ok(res) = energy.parse::<i32>(){
                    elf_energy += res
                }
            }
        }
    }
    sum_vec.sort();
    sum_vec.reverse();
    let top3 = &sum_vec[0..3];
    let top3_sum: i32 = top3.iter().sum();
    println!("max energy : {}",sum_vec[0]);
    println!("max 3 energy : {:?}",top3);
    println!("sum of top3 : {}", top3_sum);
}

/// read_lines will read the line by line from buff reader
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
