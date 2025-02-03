use std::thread::current;

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    // On Chip-8 memory, 000 - 1FF (000 to 512) is meant for the interpreter which we leave as empty for now.
    // Each address is actually only 8 bits
    // The program counter can address 16 bits at a time (so 2 memory locations at a time)
    let mut memory: [u8; 4096] = [0; 4096]; // Initialize the array with zeros

    // 16 one by general registers
    let mut registers: [[u8; 1]; 16] = [[0; 1]; 16];

    // Font Data
    let sprite_data: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0,     // 0
        0x20, 0x60, 0x20, 0x20, 0x70,     // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0,     // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0,     // 3
        0x90, 0x90, 0xF0, 0x10, 0x10,     // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0,     // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0,     // 6
        0xF0, 0x10, 0x20, 0x40, 0x40,     // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0,     // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0,     // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90,     // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0,     // B
        0xF0, 0x80, 0x80, 0x80, 0xF0,     // C
        0xE0, 0x90, 0x90, 0x90, 0xE0,     // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0,     // E
        0xF0, 0x80, 0xF0, 0x80, 0x80      // F
    ];
    // Store font data in memory
    memory[0..80].copy_from_slice(&sprite_data);

    let mut current_mem_addr = 512;
    let mut index_register: u16 = 0;    // Index register is used to point at different memory locations
    let mut stack: [u16; 2] = [0; 2];

    loop {
        //Fetch instruction
        let mut program_counter = fetch_instruction(memory, current_mem_addr);
        
        
        
        clear_background(BLACK);
        
        next_frame().await;
        let maximum_frame_time = 1. / 60.; // Maximum number of seconds per frame to abide the hardware limit of 60 fps (0.166 seconds per frame)
        let frame_time = get_frame_time(); // How many seconds since last frame drawn

        // If current fps is to high, we'll delay it
        if frame_time < maximum_frame_time {
            let delay_frame_time = (maximum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(delay_frame_time as u64))
        }

        current_mem_addr = current_mem_addr + 2;
    }
}

// Fetching as instruction in Chip-8 translates fetching two conssecutive bytes and concatenating them into one 16-bit instruction
fn fetch_instruction(memory: &[u8; 4096], current_mem_addr: usize) -> u16 {
    let current_instruction = (memory[current_mem_addr] as u16) << 8 | (memory[current_mem_addr + 1]) as u16;
    return current_instruction;
}

fn decode_instruction(current_instruction: u16) {
    let first_nibble: u8 = (current_instruction >> 8) as u8 & (1 as u8);
    match first_nibble {
        0 => clear_screen(),
        1 => jump_instruction(),
        6 => set_register(),
        7 => add_to_register(),
        11 => set_index_register(),
        13 => draw_to_screen(),
        _ => error!("Unknown instruction found!")
    }
}

fn clear_screen() {}

fn jump_instruction() {}

fn set_register() {}

fn add_to_register() {}

fn set_index_register() {}

fn draw_to_screen() {}


