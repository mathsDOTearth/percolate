// percolate simulation by R Neale
// published on https://maths.earth/
// provided "as is" under the MIT license
use std::collections::HashMap;

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

    for i in 0..height{
        for j in 0..width{
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

   
    fn main() {
     //define array, constants and variables
        const WIDTH: usize = 24;
        const HEIGHT: usize  = 16;
        const N: usize = WIDTH * HEIGHT;
        let mut n = 0;
        let mut change:usize = 0;
        let mut loops:usize = 0;
   
    //define array and populat with zeros
        let mut array = vec![vec![0; WIDTH]; HEIGHT];
   
    //populate array with consecutive numbers where 0 is rock and give it a rock boarder
        for i in 1..HEIGHT-1{
            for j in 1..WIDTH-1{
                if ((i*HEIGHT)+j)%5>0{
                    array[i][j]=(i*HEIGHT)+j;
                }
            }
        }
   
    //print array
        print_array(&array, HEIGHT, WIDTH);
   
   
    //perculate array
        while n < N {
            for i in 1..HEIGHT-1{
                for j in 1..WIDTH-1{
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
                n = N;
            }
        }
   
//print array
       println!("");
       print_array(&array, HEIGHT, WIDTH);
       println!("");

//count the clusters
       let clusters = count_clusters(&array, HEIGHT, WIDTH);
       let sorted_clusters = sort_clusters(&clusters);
       for (cluster, size) in sorted_clusters.iter() {
           println!("Cluster {:0>3} size: {}", cluster, size);
       }
    println!("");
    println!("Total clusters: {}", clusters.len());
    println!("");
    println!("Percolation completed in: {} loops (Max loops {})", loops, N);
    //end of program
    }
   