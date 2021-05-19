pub enum DataType {
    Byte(u8),
    Halfword(u16),
    Word(u32),
}

type Word = u32;
type Halfword = u16;
type Byte = u8;

pub trait Memory {
    fn read_word(&self, address: u32) -> Word;
    fn read_halfword(&self, address: u32) -> Halfword;
    fn read_byte(&self, address: u32) -> Byte;
    fn write(&self, address: u32, value: DataType);
}
