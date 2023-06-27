use std::collections::{HashMap, HashSet};
use std::env;
extern crate chrono;
use chrono::Local;
use colored::Colorize;

fn print_array(array:&Vec<Vec<usize>>, height: usize, width: usize){
    //function to print out the array to the teminal
        for i in 0..height{
            for j in 0..width{
                print!("{:0>3} ",array[i][j]);
            }
            println!("");
        }
    // end print_array function
    }

// Function to count clusters
fn count_clusters(array:&Vec<Vec<usize>>, height: usize, width: usize) -> HashMap<usize, usize> {
    let mut clusters = HashMap::new();

    for i in 1..height-1{
        for j in 1..width-1{
            if array[i][j] > 0 {
                *clusters.entry(array[i][j]).or_insert(0) += 1;
            }
        }
    }
    clusters
//end count_cluster function
}

// Function to sort clusters
fn sort_clusters(clusters: &HashMap<usize, usize>) -> Vec<(usize, usize)> {
    let mut sorted_clusters: Vec<_> = clusters.iter().collect();
    sorted_clusters.sort_by(|a, b| b.1.cmp(a.1));
    sorted_clusters.into_iter().map(|(&key, &value)| (key, value)).collect()
//end sort_cluster function
}

//Function to check if a cluster percolates
fn check_percolation(array:&Vec<Vec<usize>>, height: usize, width: usize, clusters: &HashMap<usize, usize>) {
    // Check top and bottom rows for cluster ids
    let top_row: HashSet<_> = array[1].iter().cloned().collect();
    let bottom_row: HashSet<_> = array[height - 2].iter().cloned().collect();
   
    // Check left and right columns for cluster ids
    let left_column: HashSet<_> = array.iter().map(|row| row[1]).collect();
    let right_column: HashSet<_> = array.iter().map(|row| row[width - 2]).collect();
   
    for (&cluster_id, _) in clusters.iter() {
        let touches_top = top_row.contains(&cluster_id);
        let touches_bottom = bottom_row.contains(&cluster_id);
        let touches_left = left_column.contains(&cluster_id);
        let touches_right = right_column.contains(&cluster_id);
       
        let percolates_vertically = touches_top && touches_bottom;
        let percolates_horizontally = touches_left && touches_right;
       
        if percolates_vertically && percolates_horizontally {
            println!("Cluster {:0>3} percolates both vertically and horizontally.", cluster_id);
        } else if percolates_vertically {
            println!("Cluster {:0>3} percolates vertically.", cluster_id);
        } else if percolates_horizontally {
            println!("Cluster {:0>3} percolates horizontally.", cluster_id);
        }
    }
//end check__percolation function
}

   
fn main() {
//define array, constants and variables
    let mut width: usize = 24;
    let mut height: usize  = 16;
    let mut n = 0;
    let mut change:usize = 0;
    let mut loops:usize = 0;
    let mut debug_flag: i32 = 0;
    let mut fileoutput_flag: i32 = 0;
    let now = Local::now();
    let date_time_string = now.format("%Y%m%d%H%M%S").to_string();

//get commandline args
    let args: Vec<String> = env::args().collect();
    for mut i in 1..args.len(){
        if args[i]=="help" {
            println!("{} -- <debug> <height 99> <width 99> <help> <fileoutput> <loadfile filename>","percolate".bold());
            println!("{} - print the before and after array to the console","debug".bold());
            println!("{} - numeric value for heigh (y) of array","height <99>".bold());
            println!("{} - numeric value for width (x) of the array","width <99>".bold());
            println!("{} - print this helpful information","help".bold());
            println!("{} - this will make create three files (no data will print to the console):","fileoutput".bold());
            println!("\tpre-percolation map\n\tpost-percolation map\n\trun log");
            println!("{} - this will load a pre-percolation map and run with that data","loadfile <filename>".bold());
            std::process::exit(0);
        }
        if args[i]=="debug" {debug_flag=1;}
        if args[i]=="width" {
            i+=1;
            width=match args[i].trim().parse::<usize>(){
                Ok(value) => {value},
                Err(_) => {
                    eprintln!("Width must be numeric value");
                    std::process::exit(1);
                },
            };
        }
        if args[i]=="height" {
            i+=1;
            height=match args[i].trim().parse::<usize>(){
                Ok(value) => {value},
                Err(_) => {
                    eprintln!("Height must be numeric value");
                    std::process::exit(1);
                },
            };
        }
        if args[i]=="fileoutput" {fileoutput_flag=1;}
        if args[i]=="loadfile"{
            i+=1;
            let loadfile_name = &args[i];
            println!("{}", loadfile_name);
            std::process::exit(0);
        }
    }
   
//define array and populat with zeros
    let nn: usize = width * height;
    let mut array = vec![vec![0; width]; height];
   
//populate array with consecutive numbers where 0 is rock and give it a rock boarder
    for i in 1..height-1{
        for j in 1..width-1{
            if ((i*height)+j)%5>0{
                array[i][j]=(i*height)+j;
            }
        }
    }
   
//print array
    if debug_flag==1 {
        print_array(&array, height, width);
    }
   
//perculate array
    while n < nn {
        for i in 1..height-1{
            for j in 1..width-1{
                if array[i][j] > 0{
                    if array[i][j-1]>array[i][j] {array[i][j]=array[i][j-1];change+=1;}
                    if array[i][j+1]>array[i][j] {array[i][j]=array[i][j+1];change+=1;}
                    if array[i-1][j]>array[i][j] {array[i][j]=array[i-1][j];change+=1;}
                    if array[i+1][j]>array[i][j] {array[i][j]=array[i+1][j];change+=1;}
                }
            }
        }
        loops+=1;
        if change > 0 {  
            n+=1;
            change = 0;
        } else {
            n = nn;
        }
    }
   
//print array
    if debug_flag==1 {
        println!("");
        print_array(&array, height, width);
    }
    println!("");

//count the clusters
    let clusters = count_clusters(&array, height, width);
    let sorted_clusters = sort_clusters(&clusters);
    for (cluster, size) in sorted_clusters.iter() {
        println!("Cluster {:0>3} size: {}", cluster, size);
    }
    println!("");
//check for clusters that percolate
    check_percolation(&array, height, width, &clusters);
    println!("");
//print totals
    println!("Total clusters: {}", clusters.len());
    println!("");
    println!("Percolation completed in: {} loops (Max loops {})", loops, nn);

//end of program
}
   
