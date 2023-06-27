// percolate simulation by R Neale
// published on https://maths.earth/
// provided "as is" under the MIT license
fn print_array(array:&Vec<Vec<usize>>, height: usize, width: usize){
//function to print out the array to the teminal
    for i in 0..height{
        for j in 0..width{
            print!("{:0>3} ",array[i][j]);
        }
        println!("");
    }
}

fn main() {
 //define array, constants and variables
    const WIDTH: usize = 16;
    const HEIGHT: usize  = 16;
    const N: usize = WIDTH * HEIGHT;
    let mut n = 0;

//define array and populat with zeros
    let mut array = vec![vec![0; WIDTH]; HEIGHT];

//populate array with consecutive numbers where 0 is rock and give it a rock boarder
    for i in 1..HEIGHT-1{
        for j in 1..WIDTH-1{
            if ((i*10)+j)%11>0{
                array[i][j]=(i*10)+j;
            }
        }
    }

//print array
    print_array(&array, HEIGHT, WIDTH);


//perculate array (code to be added to)
    while n < N {
        for i in 0..HEIGHT{
            for j in 0..WIDTH{
                if array[i][j] > 0{
                    array[i][j]+=1;
                }
            }
        }
        n+=1;
    }

//print array
   println!("");
   print_array(&array, HEIGHT, WIDTH);


//end of program
}
