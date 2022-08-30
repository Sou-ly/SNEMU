const WRAM_BANK         : u8 = 0x7E;
const WRAM_BANK_END     : u8 = 0x7F;
const MIRROR_1_BANK     : u8 = 0x00;
const MIRROR_1_BANK_END : u8 = 0x3F;
const MIRROR_2_BANK     : u8 = 0x80;
const MIRROR_2_BANK_END : u8 = 0xBF;
const LOW_RAM           : u16 = 0x0000;
const LOW_RAM_END       : u16 = 0x1fff;
const WRAM_SIZE         : usize = 0x20000;

pub struct Bus {
    wram : [u8; WRAM_SIZE],
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            wram : [0; WRAM_SIZE],
        }
    }

    pub fn read(&mut self, bank: u8, address: u16) -> Option<u8> {
        match bank {

            WRAM_BANK..=WRAM_BANK_END => 
                return Some(self.wram[address as usize]),

            MIRROR_1_BANK..=MIRROR_1_BANK_END | MIRROR_2_BANK..=MIRROR_2_BANK_END => {
                match address {

                    LOW_RAM..=LOW_RAM_END =>
                        return Some(self.wram[((bank as usize) << 16 | address as usize) & 0x0001_1111]),                        

                    _ => None
                }
            }
            __ => None
        }
    }

    pub fn write(&mut self, bank: u8, address: u16, data: u8) {
        match bank {

            WRAM_BANK..=WRAM_BANK_END =>
                self.wram[address as usize] = data,

            MIRROR_1_BANK..=MIRROR_1_BANK_END | MIRROR_2_BANK..=MIRROR_2_BANK_END => {
                match address {

                    LOW_RAM..=LOW_RAM_END =>
                        self.wram[((bank as usize) << 16 | address as usize) & 0x0001_1111] = data,                  

                    _ => return
                }
            }

            _ => return
        }
    }
}