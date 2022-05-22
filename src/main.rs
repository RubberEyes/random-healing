use std::collections::BTreeMap;
use std::io;
use std::fs;
use std::path::PathBuf;

use rand::prelude::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;

const HUGE: usize = 20000000;
// const HUGE: usize = 200;

enum CheckResults
{
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure
}

/*
Heal Bonus per DC:
DC 15 = 0
DC 20 = 15
DC 30 = 40
DC 40 = 65
*/

fn main() -> io::Result<()>
{
    let skill = loop 
    {
        println!("Please insert your Medicine Skill Bonus, including penalties and bonuses that are applicable (do not include circumstance bonuses).");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        match input.trim().parse::<i32>()
        {
            Ok(num) => break num,
            Err(_) => 
            {
                println!("That was not a number! Insert the Skill Bonus as a number!");
                continue
            }
        }
    };

    let mut target_dir = String::from("risky_healing_");
    target_dir.push_str(skill.to_string().as_str());

    let mut path_target_dir = PathBuf::new();
    path_target_dir.push(&target_dir.as_str());

    if !is_dir_present(target_dir.as_str()).expect("Failed to read directories for folders")
    {    
        fs::create_dir(&path_target_dir)?;
    }    

    let m = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");

    // Risky Surgery section
    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("risky surgery dc15");
    pb.set_style(sty.clone());

    let risky_surgery_15 = thread::spawn(move|| 
        {
            let risky_surgery_15 = fill_rolls_array_with_bar(skill + 2, 15,risky_surgery, 0, &pb);
            pb.finish_with_message("done with risky surgery dc15");
            risky_surgery_15
        });

    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("risky surgery dc20");
    pb.set_style(sty.clone());

    let risky_surgery_20 = thread::spawn(move||
        {
            let risky_surgery_20 = fill_rolls_array_with_bar(skill + 2, 20,risky_surgery, 15, &pb);
            pb.finish_with_message("done with risky surgery dc20");
            risky_surgery_20
        });

    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("risky surgery dc30");
    pb.set_style(sty.clone());

    let risky_surgery_30 = thread::spawn(move||
        {
            let risky_surgery_30 = fill_rolls_array_with_bar(skill + 2, 30,risky_surgery, 40, &pb);
            pb.finish_with_message("done with risky surgery dc30");
            risky_surgery_30
        });

    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("risky surgery dc40");
    pb.set_style(sty.clone());

    let risky_surgery_40 = thread::spawn(move||
        {
            let risky_surgery_40 = fill_rolls_array_with_bar(skill + 2, 40,risky_surgery, 65, &pb);
            pb.finish_with_message("done with risky surgery dc40");
            risky_surgery_40
        });

    // Treat Wounds section
    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("treat wounds dc15");
    pb.set_style(sty.clone());
    
    let treat_wounds_15 = thread::spawn(move|| 
        {
            let treat_wounds_15  = fill_rolls_array_with_bar(skill, 15, treat_wounds, 0, &pb);
            pb.finish_with_message("done with treat wounds dc15");
            treat_wounds_15
        });

    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("treat wounds dc20");
    pb.set_style(sty.clone());

    let treat_wounds_20 = thread::spawn(move||
        {
            let treat_wounds_20  = fill_rolls_array_with_bar(skill, 20, treat_wounds, 15, &pb);
            pb.finish_with_message("done with treat wounds dc20");
            treat_wounds_20
        });

    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("treat wounds dc30");
    pb.set_style(sty.clone());

    let treat_wounds_30 = thread::spawn(move||
        {    
            let treat_wounds_30  = fill_rolls_array_with_bar(skill, 30, treat_wounds, 40, &pb);
            pb.finish_with_message("done with treat wounds dc30");
            treat_wounds_30
        });
    
    let pb = m.add(ProgressBar::new(HUGE as u64));
    pb.set_message("treat wounds dc40");
    pb.set_style(sty.clone());

    let treat_wounds_40 = thread::spawn(move||
        {    
            let treat_wounds_40  = fill_rolls_array_with_bar(skill, 40, treat_wounds, 65, &pb);
            pb.finish_with_message("done with treat wounds dc40");
            treat_wounds_40
        });
    
    m.join_and_clear().unwrap();
    
    write_collection_to_csv("risky surgery DC15", risky_surgery_15.join().unwrap(), &path_target_dir);
    write_collection_to_csv("risky surgery DC20", risky_surgery_20.join().unwrap(), &path_target_dir);
    write_collection_to_csv("risky surgery DC30", risky_surgery_30.join().unwrap(), &path_target_dir);
    write_collection_to_csv("risky surgery DC40", risky_surgery_40.join().unwrap(), &path_target_dir);
    
    write_collection_to_csv("treat wounds DC15", treat_wounds_15.join().unwrap(), &path_target_dir);
    write_collection_to_csv("treat wounds DC20", treat_wounds_20.join().unwrap(), &path_target_dir);
    write_collection_to_csv("treat wounds DC30", treat_wounds_30.join().unwrap(), &path_target_dir);
    write_collection_to_csv("treat wounds DC40", treat_wounds_40.join().unwrap(), &path_target_dir);

    Ok(())
}

fn medicine_check(medicine_bonus: i32, check_dc: i32) -> CheckResults
{
    let roll = roll_d20();
    let total = roll + medicine_bonus - check_dc;
    let check: CheckResults = match total
    {
        _r if _r <= -10 => CheckResults::CriticalFailure,
        _r @ (-9..=-1)  => CheckResults::Failure,
        _r @ (0..=9)    => CheckResults::Success,
        _r if _r >= 10  => CheckResults::CriticalSuccess,
        _ => panic!()
    };

    let check: CheckResults = match roll
    {
        val if val == 1 => 
        {
            match check
            {
                CheckResults::CriticalSuccess => CheckResults::Success,
                CheckResults::Success => CheckResults::Failure,
                CheckResults::Failure => CheckResults::CriticalFailure,
                CheckResults::CriticalFailure => CheckResults::CriticalFailure,
            }
        },

        val if val == 20 =>
        {
            match check
            {
                CheckResults::CriticalSuccess => CheckResults::CriticalSuccess,
                CheckResults::Success => CheckResults::CriticalSuccess,
                CheckResults::Failure => CheckResults::Success,
                CheckResults::CriticalFailure => CheckResults::Failure,
            }
        },

        _ => check,
    };

    check
}

// random generators
fn roll_d20() -> i32
{
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    rng.gen_range(1..=20)
}

fn roll_d8() -> i32
{
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    rng.gen_range(1..=8)
}

fn risky_surgery(check: CheckResults, heal_bonus: i32) -> i32
{    
    let rolls = match check 
    {
        CheckResults::CriticalSuccess => roll_d8() + roll_d8() + roll_d8() + roll_d8() - roll_d8() + heal_bonus,
        CheckResults::Success => roll_d8() + roll_d8() + roll_d8() + roll_d8() - roll_d8() + heal_bonus,
        CheckResults::Failure => - roll_d8(),
        CheckResults::CriticalFailure => - roll_d8() - roll_d8(),
    };
    rolls
}

fn treat_wounds(check: CheckResults, heal_bonus: i32) -> i32
{
    let rolls = match check 
    {
        CheckResults::CriticalSuccess => roll_d8() + roll_d8() + roll_d8() + roll_d8() + heal_bonus,
        CheckResults::Success => roll_d8() + roll_d8() + heal_bonus,
        CheckResults::Failure => 0,
        CheckResults::CriticalFailure => - roll_d8(),
    };
    rolls
}

/* 
fn fill_rolls_array(medicine_bonus: i32, check_dc: i32, action: fn(CheckResults, i32)->i32, heal_bonus: i32) -> BTreeMap<i32, i32>
{
    let mut ar: BTreeMap<i32, i32> = BTreeMap::new();

    for _i in 0..HUGE 
    {
        let check = medicine_check(medicine_bonus, check_dc);
        let heal = action(check, heal_bonus);
        ar.insert(
            heal,
            ar.get(&heal).unwrap_or(&0) + &1);
    };
    ar
}
*/

fn fill_rolls_array_with_bar(medicine_bonus: i32, check_dc: i32, action: fn(CheckResults, i32)->i32, heal_bonus: i32, bar: &ProgressBar) -> BTreeMap<i32, i32>
{
    let mut ar: BTreeMap<i32, i32> = BTreeMap::new();

    for _i in 0..HUGE 
    {
        bar.inc(1);

        let check = medicine_check(medicine_bonus, check_dc);
        let heal = action(check, heal_bonus);

        ar.insert(
            heal,
            ar.get(&heal).unwrap_or(&0) + &1);
    };
    ar
}

fn write_collection_to_csv(title: &str, col: BTreeMap<i32, i32>, path: &PathBuf)
{
    let mut text:String = String::from(title);
    text.push_str(",times");

    col.into_iter().for_each(|elem| 
    {
        text.push_str("\n");
        text.push_str(elem.0.to_string().as_str());
        text.push_str(",");
        text.push_str(elem.1.to_string().as_str());
    });

    let mut path = path.to_owned();
    path.push(title);
    let path = path.with_extension("csv");

    println!("{}", &path.display());

    fs::write(path, text).expect("Unable to write file");
}

fn is_dir_present(dir_name: &str) -> io::Result<bool>
{
    let mut wanted_path = PathBuf::new();
    wanted_path.push(".");
    wanted_path.push(dir_name);

    println!("Searching for {}...", wanted_path.display());

    for entry in fs::read_dir(".")?
    {
        let path = entry?.path();
        if path == wanted_path 
        {
            return Ok(true)
        }
    }
    Ok(false)
}