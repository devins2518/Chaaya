pub(crate) trait Memory {
    fn map_address(&self, address: u32) -> &[u8];
    fn map_address_mut(&mut self, address: u32) -> &mut [u8];
    fn read_word(&self, address: u32) -> u32;
    fn read_halfword(&self, address: u32) -> u16;
    fn read_byte(&self, address: u32) -> u8;
    fn write(&self, address: u32, value: &[u8]);
}
