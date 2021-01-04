mod chip8;
use chip8::Chip8;
use std::thread;
use std::time::Duration;

const CLOCK_SPEED_HZ: u64 = 60;

fn main() {
    // Set up render system and register input callbacks
    // setupGraphics();
    // setupInput();

    // Initialize the Chip8 system and load the game into the memory
    let mut chip8 = Chip8::new();

    chip8.initialize();
    chip8.load_game("roms/Kaleidoscope [Joseph Weisbecker, 1978].ch8");

    let mut i = 0;
    loop {
        // Emulate one cycle
        i += 1;
        println!("\nCYCLE {}:", i);
        chip8.emulate_cycle();
        println!("{:?}", chip8);

        // If the draw flag is set, update the screen
        // if chip8.draw_flag {
        //     drawGraphics();
        // }

        // Store key press state (Press and Release)
        if i % 10 == 0 {
            chip8.set_keys(0x01, true); // Hold down 0x05 key (temp)
        } else if i % 7 == 0 {
            chip8.set_keys(0x07, true);
        } else if i % 2 == 0 {
            chip8.set_keys(0x07, false);
            chip8.set_keys(0x01, false);
        }

        // Lock cycle loop to 60hz
        thread::sleep(Duration::from_millis(1000 / CLOCK_SPEED_HZ))
    }
}
