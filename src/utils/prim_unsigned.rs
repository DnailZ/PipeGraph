
use std::u32;

use int::UInt;

pub trait PrimUnsigned : UInt {
    fn lowest_u8(self) -> u8;
    fn lowest_u16(self) -> u16;
    fn lowest_u32(self) -> u32;
    fn lowest_u64(self) -> u64;
    fn lowest_u128(self) -> u128;
}

impl PrimUnsigned for u8 {
    #[inline(always)]
    fn lowest_u8(self) -> u8 {
        self as u8
    }
    
    #[inline(always)]
    fn lowest_u16(self) -> u16 {
        self as u16
    }
    
    #[inline(always)]
    fn lowest_u32(self) -> u32 {
        self as u32
    }
    
    #[inline(always)]
    fn lowest_u64(self) -> u64 {
        self as u64
    }
    
    #[inline(always)]
    fn lowest_u128(self) -> u128 {
        self as u128
    }
}

impl PrimUnsigned for u16 {
    #[inline(always)]
    fn lowest_u8(self) -> u8 {
        self as u8
    }
    
    #[inline(always)]
    fn lowest_u16(self) -> u16 {
        self as u16
    }
    
    #[inline(always)]
    fn lowest_u32(self) -> u32 {
        self as u32
    }
    
    #[inline(always)]
    fn lowest_u64(self) -> u64 {
        self as u64
    }
    
    #[inline(always)]
    fn lowest_u128(self) -> u128 {
        self as u128
    }
}

impl PrimUnsigned for u32 {
    #[inline(always)]
    fn lowest_u8(self) -> u8 {
        self as u8
    }
    
    #[inline(always)]
    fn lowest_u16(self) -> u16 {
        self as u16
    }
    
    #[inline(always)]
    fn lowest_u32(self) -> u32 {
        self as u32
    }
    
    #[inline(always)]
    fn lowest_u64(self) -> u64 {
        self as u64
    }
    
    #[inline(always)]
    fn lowest_u128(self) -> u128 {
        self as u128
    }
}

impl PrimUnsigned for u64 {
    #[inline(always)]
    fn lowest_u8(self) -> u8 {
        self as u8
    }
    
    #[inline(always)]
    fn lowest_u16(self) -> u16 {
        self as u16
    }
    
    #[inline(always)]
    fn lowest_u32(self) -> u32 {
        self as u32
    }
    
    #[inline(always)]
    fn lowest_u64(self) -> u64 {
        self as u64
    }
    
    #[inline(always)]
    fn lowest_u128(self) -> u128 {
        self as u128
    }
}

impl PrimUnsigned for u128 {
    #[inline(always)]
    fn lowest_u8(self) -> u8 {
        self as u8
    }
    
    #[inline(always)]
    fn lowest_u16(self) -> u16 {
        self as u16
    }
    
    #[inline(always)]
    fn lowest_u32(self) -> u32 {
        self as u32
    }
    
    #[inline(always)]
    fn lowest_u64(self) -> u64 {
        self as u64
    }
    
    #[inline(always)]
    fn lowest_u128(self) -> u128 {
        self as u128
    }
}

impl PrimUnsigned for usize {
    #[inline(always)]
    fn lowest_u8(self) -> u8 {
        self as u8
    }
    
    #[inline(always)]
    fn lowest_u16(self) -> u16 {
        self as u16
    }
    
    #[inline(always)]
    fn lowest_u32(self) -> u32 {
        self as u32
    }
    
    #[inline(always)]
    fn lowest_u64(self) -> u64 {
        self as u64
    }
    
    #[inline(always)]
    fn lowest_u128(self) -> u128 {
        self as u128
    }
}