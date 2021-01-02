use std::fs::File;
use std::io;
use std::io::Read;
use std::slice;

#[derive(Debug)]
pub struct Chip8 {
    /* === CPU ===*/
    opcode: u16,
    memory: [u8; 4096],

    // registers v0, v1 ... vE
    registers: [u8; 16],

    index_register: u16,
    program_counter: u16,

    // Screen graphics (64 x 32 px)
    gfx: [u8; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: Stack,
    stack_pointer: u16,

    keypad: [u8; 16],

    /* === FLAGS === */
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            opcode: 0x0000,
            memory: [0x00; 4096],
            registers: [0x00; 16],
            index_register: 0x00,
            program_counter: 0x00,
            gfx: [0x00; 64 * 32],
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: Stack::new(),
            stack_pointer: 0x00,
            // keypad: (0x0u8..0xEu8).collect::<[u8; 16]>()
            keypad: [0x00; 16],
            draw_flag: false,
        }
    }
    pub fn initialize(&mut self) {
        self.program_counter = 0x200;

        // Clear display
        // Clear stack
        // Clear registers V0-VF
        // Clear memory

        // THIS "FONTSET" is a TEMPORARY VALUE
        let chip8_fontset = [0x00; 80];

        // Load fontset
        for i in 0..80 {
            self.memory[i] = chip8_fontset[i];
        }

        // Reset timers
    }
    pub fn load_game(&mut self, file_name: &str) -> io::Result<()> {
        let mut f = File::open(file_name)?;
        // TODO: Handle error for file loading

        let buffer_size = f.metadata().unwrap().len() as usize;

        let offset: usize = self.program_counter.into();
        let end = offset + buffer_size;
        let mem_slice = &mut self.memory[offset..end];

        for (mem_byte, file_byte) in mem_slice.iter_mut().zip(f.bytes()) {
            *mem_byte = file_byte.unwrap();
        }

        // println!("First 10 bytes of memory: {:#04x?}", &mem_slice[0..10]);

        Ok(())
    }
    pub fn set_keys(&self) {}
    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        let pc = self.program_counter as usize;
        let opcode = u16::from_be_bytes([self.memory[pc], self.memory[pc + 1]]);
        // println!("opcode: {:#0x?}", opcode);

        // Decode Opcode
        // Get the first half byte (nibble) to determine the opcode
        match opcode & 0xF000 {
            // 6xNN: sets VX to NN
            0x6000 => {
                let bytes = opcode.to_be_bytes();
                let x: usize = (bytes[0] & 0x0F).into();
                let nn = bytes[1];
                self.registers[x] = nn;

                // Move pc 2 bytes to next opcode
                self.program_counter += 2;

                // println!("\nNEXT CPU CYCLE:");
                // println!("bytes: {:#04x?}", bytes);
                // println!("x: {}", x);
                // println!("nn: {:#04x?}", nn);
                // println!("V Registers: {:?}", self.registers);
            }
            _ => eprintln!("Unknown opcode: {:#0x?}", opcode),
        }

        // Execute Opcode

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }
}

#[derive(Debug)]
struct Stack {
    levels: Vec<u16>,
}

impl Stack {
    fn new() -> Self {
        Stack { levels: Vec::new() }
    }
    // Consider initializing with Vec::with_capacity(16) instead
    fn push(&mut self, val: u16) {
        if self.levels.len() < 16 {
            self.levels.push(val);
        } else {
            eprintln!("CPU Stack is full!");
        }
    }
}
