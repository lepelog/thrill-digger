use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::time;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::path::Path;

use crate::rng::*;

use crate::thrill_digger::*;

fn get_loop_value_belongs_to(val: u32, loops: &Vec<HashSet<u32>>) -> usize {
    let mut rng = RngContext::from_state(val);
    loop {
        for (index, lop) in loops.iter().enumerate() {
            if lop.contains(&rng.get_state()) {
                return index;
            }
        }
        rng.next_u32();
    }
}

fn find_longest_journey_to_loop() {
    let mut all_loop_vars: HashSet<u32> = HashSet::new();
    for loop_members in get_all_loops_and_members().iter() {
        all_loop_vars.extend(loop_members.into_iter());
    }
    let mut rng = RngContext::from_state(0);
    let mut max_distance = 0;
    let mut max_distance_seed = 0;
    println!("total loop members: {}", all_loop_vars.len());
    'outloop: for seed in vec![3128500378 as u32].into_iter() {
        if seed % 100 == 0 {
            println!("seed {}, longest distance: {}, long distance seed: {}", seed, max_distance, max_distance_seed);
        }
        rng.set_state(seed);
        let mut distance = 0;
        while !all_loop_vars.contains(&rng.get_state()) {
            rng.next_u32();
            distance += 1;
        }
        if distance > max_distance {
            max_distance = distance;
            max_distance_seed = seed;
        }
    }
    println!("max distance: {}, seed: {}", max_distance, max_distance_seed);
}

fn get_all_loops_and_members() -> Vec<HashSet<u32>> {
    let all_loops = load_loops().unwrap();
    let mut all_loops_with_members = Vec::with_capacity(all_loops.len());
    for (loop_start, period) in all_loops.iter() {
        let mut current_loop_members = HashSet::with_capacity(*period as usize);
        let mut rng = RngContext::from_state(*loop_start);
        current_loop_members.insert(*loop_start);
        for _ in 0..=*period + 1 {
            current_loop_members.insert(rng.next_u32());
        }
        all_loops_with_members.push(current_loop_members);
    }
    return all_loops_with_members;
}

fn load_loops() -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    let path = Path::new("all_loop_counts");
    let buf_read = BufReader::new(File::open(&path)?);
    let mut loops_and_counts = Vec::new();
    for raw_line in buf_read.lines() {
        let line = raw_line?;
        let mut split_line = line.split(":");
        let left = split_line.next().unwrap_or("").trim().parse::<u32>()?;
        let right = split_line.next().unwrap_or("").trim().parse::<u32>()?;
        loops_and_counts.push((left, right));
    }
    Ok(loops_and_counts)
}

fn find_periods_for_all_loops() {
    let path = Path::new("all_loops.txt");
    let mut buf_read = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => BufReader::new(file),
    };
    let mut loop_counts = HashMap::new();
    for num in buf_read.split(b',').map(|s| String::from_utf8(s.unwrap()).unwrap().parse::<u32>().unwrap()) {
        let mut curren_loop_vals: HashSet<u32> = HashSet::new();
        let mut rng = RngContext::from_state(num);
        let mut loop_count = 0;
        while !curren_loop_vals.contains(&rng.get_state()) {
            curren_loop_vals.insert(rng.get_state());
            rng.next_u32();
            loop_count += 1;
        }
        loop_counts.insert(num, loop_count);
    }
    let mut buf_write = match File::create(&Path::new("all_loop_counts")) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => BufWriter::new(file),
    };
    let mut sorted_loop_counts: Vec<(u32, isize)> = loop_counts.into_iter().collect();
    sorted_loop_counts.sort_unstable_by_key(|s| -s.1);
    for (loop_val, loop_size) in sorted_loop_counts.iter() {
        writeln!(buf_write, "{}: {}", loop_val, loop_size);
    }
    buf_write.flush().unwrap();
}

fn get_totally_all_loops() {
    let mut all_vals = vec![false; u32::MAX as usize + 2];
    let mut rng = RngContext::from_state(0);
    let mut curren_loop_vals: HashSet<u32> = HashSet::new();
    let mut all_found_loops = Vec::new();
    'outloop: for seed in 0..=u32::MAX {
        if seed % 1_000_000 == 0 {
            println!("seed {}, found {} loops", seed, all_found_loops.len());
        }
        if all_vals[seed as usize] {
            continue;
        }
        rng.set_state(seed);
        curren_loop_vals.clear();
        curren_loop_vals.shrink_to_fit();
        while !curren_loop_vals.contains(&rng.get_state()) {
            if all_vals[rng.get_state() as usize] {
                continue 'outloop;
            }
            all_vals[rng.get_state() as usize] = true;
            curren_loop_vals.insert(rng.get_state());
            rng.next_u32();
        }
        all_found_loops.push(rng.get_state());
    }
    let path = Path::new("all_loops.txt");
    let mut buf_write = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => BufWriter::new(file),
    };
    for loop_start in all_found_loops.iter() {
        write!(buf_write, "{},", loop_start);
    }
    buf_write.flush().unwrap();
}

fn hits_after_niter() {
    let mut all_hits = vec![false; u32::MAX as usize + 2];
    let mut rng = RngContext::from_state(0);
    for seed in 0..=u32::MAX {
    // for seed in 0..=3_000_000 {
        if seed % 1_000_000 == 0 {
            println!("seed {}", seed);
        }
        rng.set_state(seed);
        for _ in 0..400 {
            rng.next_u32();
        }
        all_hits[rng.get_state() as usize] = true;
    }
    let total_hit_count: usize = all_hits.iter().fold(0, |acc, x| match x {
        false => acc,
        true => acc +1,
    });
    // 2000: 2995956
    // 3000: 2994389
    // 4000: 2992524
    println!("total hit: {}", total_hit_count);
    let path = Path::new("all_hit_seeds.txt");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    for (index, was_hit) in all_hits.iter().enumerate() {
        if *was_hit {
            write!(file, "{},", index);
        }
    }
}

fn test_for_board() {
    // let expected = vec![
    //     HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified,
    //     HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified,
    //     HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified,
    //     HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified,
    //     HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified,
    // ];
    let expected = vec![
        HoleContent::Rupoor, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified,
        HoleContent::Bomb, HoleContent::Bomb, HoleContent::Rupoor, HoleContent::Bomb, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified,
        HoleContent::Bomb, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Bomb, HoleContent::Rupoor, HoleContent::Unspecified, HoleContent::Bomb,
        HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Bomb, HoleContent::Unspecified,
        HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Unspecified, HoleContent::Rupoor, HoleContent::Rupoor, HoleContent::Rupoor, HoleContent::Rupoor,
    ];
    let start_time = time::SystemTime::now();
    let iterations = 10000000;
    let mut rng = RngContext::from_state(3450336793);
    for i in 0..iterations {
        if i % 100000 == 0 {
            println!("checking loop {}, seed {}",i, rng.get_state());
        }
        let hole_minigame = HoleMinigame::generate(&mut RngContext::from_state(rng.get_state()), HoleMinigameDifficulty::Expert);
        if hole_minigame.check_equals(&expected) {
            println!("found matching seed: {}", i);
            break;
        }
        rng.next_u32();
    }
    let time_taken = time::SystemTime::now().duration_since(start_time).unwrap();
    println!("Took {}ms for {} iterations, thats {} for one", time_taken.as_millis(), iterations, (time_taken.as_millis() as f32/iterations as f32));

}
fn alt_main() {
    let mut rng = RngContext::from_state(0x945B4001);
    // HoleMinigame::generate(&mut rng,HoleMinigameDifficulty::Intermediate).draw();
    let mut last_state : u32 = 3249224983;
    let first_state = last_state;
    let mut holePossibilities = Vec::new();
    let mut iter_count = 0;
    loop {
        iter_count+=1;
        rng.set_state(last_state);
        last_state = rng.next_u32();
        let test = HoleMinigame::generate(&mut rng, HoleMinigameDifficulty::Beginner);
        // test.draw();
        // println!();
        holePossibilities.push(test);
        if last_state == first_state {
            break;
        }
    }
    // filter for already placed
    holePossibilities = holePossibilities.into_iter()
        .filter(|h| {
            return h.get_holes()[10] == HoleContent::GreenRupee && h.get_holes()[17] == HoleContent::BlueRupee;
        }).collect();
    let mut bomb_totals = [0; 20];
    for game in holePossibilities.iter() {
        for i in 0..20 {
            bomb_totals[i] += if game.get_holes()[i] == HoleContent::Bomb { 1 } else { 0 };
        }
    }
    let bomb_percents: Vec<f32> = bomb_totals.iter().map(|v| (*v as f32)/(holePossibilities.len() as f32)).collect();
    for y in 0..4 {
        for x in 0..5 {
            print!("{:01.2}  ", bomb_percents[y * 5 + x]);
        }
        println!();
    }
    println!("total count {}", holePossibilities.len());
    for (i, percent) in bomb_totals.iter().map(|v| (*v as f32)/(holePossibilities.len()) as f32).enumerate() {
        println!("{}: {}", i, percent);
    }
}