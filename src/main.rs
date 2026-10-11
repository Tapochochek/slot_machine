use rand::RngExt;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello to my slot machine");
    println!("Enter your name");

    
    let mut user_name= String::new(); 
    std::io::stdin().read_line(&mut user_name).expect("Not good format for user name");
    let mut balance:u64 = 100;

    loop{
        match print_menu_take_command() {
            1 => {println!("Your balance: {}", balance); continue;},
            2 => generate_slots(),
            3 => {println!("Goodbye"); break;}
            _ => {println!("Incorrect command"); continue;}
        };
    }
    
}
fn print_menu_take_command()-> u32{

    println!("Enter command");
    println!("1)Show balance");
    println!("2)Place a bet");
    println!("3)Exit game");

    let mut command_input = String::new();
    io::stdin().read_line(&mut command_input).expect("Incorrect input");
    let command_input:u32 = command_input.trim().parse().expect("this is not a number");
    command_input
}
fn generate_slots(){

    let char_mass = ['🍋', '🍒', '🍀', '🔔', '💰', '💎', '𝟕'];

    let mut slots_vector = Vec::new();
    for slot_element in (0..15).rev(){
        slots_vector.push(char_mass[rand::rng().random_range(0..6)]);
    }
    let mut stroke_counter = 0;
    println!();
    for slot_element in &slots_vector{
        
        if stroke_counter < 5{
            //thread::sleep(Duration::from_secs_f32(0.2));
            print!("{}",slot_element.to_string());
        }
        else{
            print!("\n{}",slot_element.to_string());
            stroke_counter = 0;
        }
        stroke_counter += 1;
    }
    println!("\n");

    check_combo(&slots_vector);
}
fn check_combo(all_symbols:&[char])->u32{

    let first_stroke = [all_symbols[0],all_symbols[1],all_symbols[2],all_symbols[3],all_symbols[4]];
    let second_stroke = [all_symbols[5],all_symbols[6],all_symbols[7],all_symbols[8],all_symbols[9]];
    let third_stroke = [all_symbols[10], all_symbols[11], all_symbols[12],all_symbols[13],all_symbols[14]];

    let mut first_stroke_buffer = 0;
    let mut second_stroke_buffer = 0;
    let mut third_stroke_buffer = 0;
    
    for index in (0..=4).rev(){

        if index > 0{
            if first_stroke[index] == first_stroke[index-1]{
                first_stroke_buffer += 1;
            }
            else{
                first_stroke_buffer = 0;
            }

            if second_stroke[index] == second_stroke[index-1]{
                second_stroke_buffer+=1;                         
            }
            else{
                second_stroke_buffer = 0;
            }

            if third_stroke[index] == third_stroke[index-1]{
                third_stroke_buffer += 1; 
            }
            else{
                third_stroke_buffer = 0;
            }
        }
        if first_stroke_buffer > 1{
            println!("Горизонтальная линия длинной {}",first_stroke_buffer+1);
        }
        if second_stroke_buffer > 1{
            println!("Горизонтальная линия длинной {}",second_stroke_buffer+1);
        }
        if third_stroke_buffer > 1{
            println!("Горизонтальная линия длинной {}",third_stroke_buffer+1);
        }


        if first_stroke[index] == second_stroke[index] && second_stroke[index] == third_stroke[index]{
            println!("Вертикальная линия");
        }

        if index <= 2{
            if first_stroke[index] == second_stroke[index+1] && second_stroke[index+1] == third_stroke[index+2]{
                println!("Косая");
            }
        }
        if index >= 2 {
            if first_stroke[index] == second_stroke[index-1] && second_stroke[index-1] == third_stroke[index-2]{
                println!("Косая 2");
            }
        }
    }


    return 0;

}