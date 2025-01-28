use macroquad::prelude::*;

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    // On Chip-8 memory, 000 - 1FF (000 to 512) is meant for the interpreter which we leave as empty for now.
    // Each address is actually only 12 bits, but we'll allow 16 bits.
    // The program counter can only address 16 bits at a time.
    let mut memory: [u16; 256] = [0; 256]; // Initialize the array with zeros

    // Font Data
    let sprite_data: [u16; 80] = [
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
    // Store that data in memory
    memory[0..80].copy_from_slice(&sprite_data);

    let mut stack: [u16; 16] = [0; 16];


    let mut program_counter: u16 = 0;   // Program counter keeps track of current address for instruction fetched
    let mut index_register: u16 = 0;    // Index register is used to point at different memory locations

    loop {
        clear_background(BLACK);
        next_frame().await;

        let maximum_frame_time = 1. / 60.; // Maximum number of seconds per frame to abide the hardware limit of 60 fps (0.166 seconds per frame)
        let frame_time = get_frame_time(); // How many seconds since last frame drawn

        // If current fps is to high, we'll delay it
        if frame_time < maximum_frame_time {
            let delay_frame_time = (maximum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(delay_frame_time as u64))
        }

    }


}

