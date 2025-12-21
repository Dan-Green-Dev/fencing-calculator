use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, prelude::*, stdout};
use std::usize;
use table_extract;
#[derive(Debug,Clone)]
struct Fencer{
    name: String,
    rank: usize,
    club: String,
    seed: usize,
    entries: usize,
    international: bool,
    points: usize,
    old_ranks: HashMap<usize,usize>
}
impl Default for Fencer {
    fn default() -> Fencer {
        Fencer{
            name: "".to_string(),
            rank: 999,
            club: "unaffiliated".to_string(),
            seed: 999,
            entries: 0,
            international: false,
            points: 0,
            old_ranks: HashMap::new()
        }

    }
    
}
#[derive(Debug,Clone)]
struct  Pool{
    number: usize,
    fencers: Vec<Fencer>
}

// todo
// take in parameters to do things other than SMF instead of changing files
// option to add a fencer
// pool wins -> tableau guess
fn main() {
    let mut fencers = read_entries();
    //println!("Hello, world!");

    fencers = get_data(fencers);
    let mut old_years = Vec::new();
    fencers = get_old_data(2024,fencers.clone());
    old_years.push(2024);
    // calc_points(&fencers);

    let pools = generate_pools(fencers.clone());

    // print_pools(pools.clone());
    println!("Welcome to my fencing stalking app!!! Use:\n    'pool' to print out the pools\n    'points' to calculate the NIF and points of each position\n    'help' to print the help screen\n    'exit' to exit the program");
    loop {
        print!(">");
        let _ = stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("BAD STRING");
        if input == "exit\n".to_string(){
            break;
        }
        if input == "pool\n"{
            print_pools(pools.clone(),&old_years);
        }
        if input == "points\n"{
            calc_points(&fencers.clone());
        }
        if input == "catagory\n"{
            change_catagory();
        }
    }
}
fn read_entries()-> Vec<Fencer>{
    let mut entries_file = File::open("src/input.csv").unwrap();
    let mut contents = String::new(); 
    entries_file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    let mut fencers = Vec::new();
    while let Some(l) = lines.next(){
        if l.len() !=0{ // have not reached EOF, which can be blank if pasted weird
            lines.next();
            let name = l;
            let mut club = "";
            if let Some(l) = lines.next(){
                club = l;
            }
            fencers.push(Fencer{name:name.to_string(),club:club.to_string(),..Default::default()});
        }
        else{
            //println!("EOF?")// TODO add additional checks for entry >50 being pasted in adequately 
        }
    }
    fencers
}

fn get_data(mut fencers: Vec<Fencer>)-> Vec<Fencer>{
        //println!("fencers: {:?}", fencers);
    let html_file = fs::read_to_string("src/senior-mens-foil.html").expect("file not read :(");
    
    let table = table_extract::Table::find_first(html_file.as_str()).unwrap(); 
    let mut rank_fencer =Vec::new();
    let mut table_started = false;
    for row in &table{
        let mut i = row.iter().cloned();
        let rank = i.next().unwrap_or("".to_string());
        let name = i.next().unwrap_or("".to_string());
        if rank =="1"{
            table_started = true;
        }
        if table_started{
            i.next();
            i.next();
            i.next();
            let total_points = i.next().unwrap().parse::<usize>().unwrap();
            // println!("total points {}", total_points);
            let _domestic_points = i.next().unwrap().parse::<usize>().unwrap();
            let domestic_entries = i.next().unwrap().parse::<usize>().unwrap();
            let _int_points = i.next().unwrap().parse::<usize>().unwrap();
            let int_entries = i.next().unwrap().parse::<usize>().unwrap();
            let entries =int_entries+domestic_entries;
            let rank_num = rank.parse().expect("NaN");
            rank_fencer.push(Fencer{name:name,rank:rank_num,entries:entries,points: total_points,..Default::default()});
        }

    }
    //println!("{:?}",rank_fencer);
    for mut fencer in &mut fencers{
        
        let mut rank_iter = rank_fencer.iter();
        let fencer_name = fencer.name.split_whitespace();
        let len = fencer_name.count();
        if len>=3{
            let names1 = &fencer.name.split_whitespace();
            for n2 in &mut rank_iter{ // loops through all fencers
                let names2 = n2.name.split_whitespace();
                let mut matches = 0;
                if len == names2.clone().count(){
                    for l in names1.clone(){
                        for r in names2.clone(){
                            if l.to_ascii_lowercase()==r.to_ascii_lowercase(){
                                matches = matches+1;
                            }
                        }
                    }
                }
                if matches==len{
                    fencer.points = n2.points;
                    fencer.entries = n2.entries;
                    fencer.international = n2.international;
                    fencer.rank = n2.rank;       
                }
            }
        }
        let mut rev_iter= fencer.name.split_whitespace().rev();
        let rev_name = rev_iter.next().unwrap().to_string() + " " + rev_iter.next().unwrap();
        for n2 in &mut rank_iter{
            if rev_name.to_lowercase()==n2.name.to_lowercase(){
                //println!("n1 {:?}, n2 {:?}",rev_name.to_lowercase(),n2.name.to_lowercase());
                update_fencer(&mut fencer, n2);
            }
        }
    }
    fencers.sort_by(|a,b|a.rank.cmp(&b.rank));
    let mut count =1;
    for f in &mut fencers{
        //println!("{:?}", f);
        f.seed = count;
        count = count +1;
    }
    return fencers.clone();
}

fn add_to_pool(pool: &mut Pool,f: Fencer){
    pool.fencers.push(f);
}

fn update_fencer(f1: &mut Fencer, f2: &Fencer){
    f1.points = f2.points;
    f1.entries = f2.entries;
    f1.international = f2.international;
    f1.rank = f2.rank;
}

fn print_pools(pools: Vec<Pool>, old_years: &Vec<usize>){
    
    for p in pools{
        println!("POOL {}",p.number+1);
        for f in p.fencers{
            let mut ppe =0;
            if f.entries>0{
                ppe = f.points/f.entries;
            }
            let mut output = format!("");
            if f.international{
                output = output + &format!("INTERNATIONAL FENCER");

            }
            output = output +& format!(" Name: {} Rank: {} Seed: {} Club: {}, Points per entry:{}",f.name, f.rank,f.seed, f.club,  ppe);
            for y in old_years{
                if let Some(out) =f.old_ranks.get(y){
                    output = output + &format!(" Rank in {}: {}",y,out);
                }
            }
            println!("{}",output);
        }
    }
}

fn generate_pools(fencers: Vec<Fencer>)-> Vec<Pool>{
    println!("Enter how big each pool will be:  ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("BAD STRING");

    let number = fencers.iter().count();
    let num_pools = (number as f64/input.trim().parse::<f64>().unwrap()).ceil() as i32;
    let mut prev_pool = 0;
    let mut pool_direction = 1;

    let mut pools = Vec::<Pool>::new();
    for f in fencers{
        let pool_num;
        // pool size is input
        if prev_pool + pool_direction <= num_pools && prev_pool + pool_direction >0{
            pool_num = prev_pool + pool_direction;
        }
        else{
            pool_direction = pool_direction*-1;
            pool_num = prev_pool;
        }
        // println!("{:?}",pool_num);
        if pools.get((pool_num-1) as usize).is_some(){
            add_to_pool(&mut pools[(pool_num -1) as usize], f);
        }else{
            // println!("ERROR? pool num:{:?} fencer: {:?}",pool_num, f);
            let new_pool = Pool{fencers:vec![],number: (pool_num -1) as usize};
            pools.push(new_pool);
            add_to_pool(&mut pools[(pool_num -1) as usize], f);
        }
        prev_pool = pool_num;
    }
    pools
}
fn calc_points(fencers: &Vec<Fencer>){
        let mut nif =0;
    let mut num:f64 = fencers.iter().count() as f64;
    println!("there are {} fencers", num);

    num = (num *0.83).floor();
    println!("Bottom place eligible for ranking points is: {:?}",num as usize);
    for f in fencers{
        if f.rank<21{
            nif = nif + 10;
        }
        if f.rank<61{
            nif = nif +6;
        }
        if f.rank<151{
            nif = nif +3
        }
        if f.rank<326{
            nif = nif +1;
        }
    }
    println!("The NIF is {:?}",nif);
    let nif =nif as f64;

    if num>8.0{
        println!("1st place will be awarded {} points",(nif*20.0).floor());
    }
    if num>8.0{
        println!("3rd place will be awarded {} points",(nif*16.0).floor());
    }
    if num>8.0{
        println!("8th place will be awarded {} points",(nif*14.1).floor());
    }
    if num >16.0{
        println!("16th place will be awarded {} points",(nif*10.8).floor());
    }
    if num >32.0{
        println!("32nd place will be awarded {} points",(nif*7.5).floor());
    }
    if num >64.0{
        println!("64th place will be awarded {} points",(nif*4.2).floor());
    }
    if num >128.0{
        println!("128th place will be awarded {} points",(nif*0.9).floor());
    }
}


fn get_old_data(year: usize, mut fencers: Vec<Fencer>)->Vec<Fencer>{ // this is a bit of a bad way to do it, should fix highkey
        //println!("fencers: {:?}", fencers);
    let html_file = fs::read_to_string("src/archive/senior-mens-foil-2024.html").expect("file not read :(");
    
    let table = table_extract::Table::find_first(html_file.as_str()).unwrap(); 
    let mut rank_fencer =Vec::new();
    let mut table_started = false;
    for row in &table{
        let mut i = row.iter().cloned();
        let rank = i.next().unwrap_or("".to_string());
        let name = i.next().unwrap_or("".to_string());
        if rank =="1"{
            table_started = true;
        }
        if table_started{
            let rank_num = rank.parse().expect("NaN");
            rank_fencer.push(Fencer{name:name,rank:rank_num,..Default::default()});
        }

    }
    //println!("{:?}",rank_fencer);
    for mut fencer in &mut fencers{
        
        let mut rank_iter = rank_fencer.iter();
        let fencer_name = fencer.name.split_whitespace();
        let len = fencer_name.count();
        if len>=3{
            let names1 = &fencer.name.split_whitespace();
            for n2 in &mut rank_iter{ // loops through all fencers
                let names2 = n2.name.split_whitespace();
                let mut matches = 0;
                if len == names2.clone().count(){
                    for l in names1.clone(){
                        for r in names2.clone(){
                            if l.to_ascii_lowercase()==r.to_ascii_lowercase(){
                                matches = matches+1;
                            }
                        }
                    }
                }
                if matches==len{
                    fencer.old_ranks.insert(year, n2.rank); 
                }
            }
        }
        let mut rev_iter= fencer.name.split_whitespace().rev();
        let rev_name = rev_iter.next().unwrap().to_string() + " " + rev_iter.next().unwrap();
        for n2 in &mut rank_iter{
            if rev_name.to_lowercase()==n2.name.to_lowercase(){
                //println!("n1 {:?}, n2 {:?}",rev_name.to_lowercase(),n2.name.to_lowercase());
                fencer.old_ranks.insert(year, n2.rank);
            }
        }
    }
    fencers.sort_by(|a,b|a.rank.cmp(&b.rank));
    let mut count =1;
    for f in &mut fencers{
        //println!("{:?}", f);
        f.seed = count;
        count = count +1;
    }
    return fencers.clone();
}

fn change_catagory(){
    // take entries filename, catagory of comp
}