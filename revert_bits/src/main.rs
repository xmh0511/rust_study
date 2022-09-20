trait RevertBitsInByte {
    fn revert_bits(&self) -> Self;
}

impl RevertBitsInByte for u8 {
    fn revert_bits(&self) -> Self {
        let ori = *self;
        let len = 7usize;
        let mut dst = 0;
        let filter = 0b00000001u8;
        for pos in 0..=len {
            let mut bit = ori >> (len - pos);
            bit &= filter;
            bit <<= pos;
            dst |= bit;
        }
        dst
    }
}

macro_rules! impl_revert_bits {
	($($type:ty),+) => {
		$(
			impl RevertBitsInByte for $type{
				fn revert_bits(&self) -> Self{
					let bytes = self.to_ne_bytes();
					const size:usize = std::mem::align_of::<Self>();
					let mut index = 0;
					let mut arr = [0;size];
					for e in bytes{
						let r = e.revert_bits();
						arr[index] = r;
						index+=1;
					};
					Self::from_ne_bytes(arr)
				}
			}
		)+
	};
}

impl_revert_bits!{i32}

fn main() {

}