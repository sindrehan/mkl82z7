#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GICLR {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = "Values that can be written to the field `GIWE`"]
pub enum GIWEW {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0,
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    _1,
}
impl GIWEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            GIWEW::_0 => 0,
            GIWEW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GIWEW<'a> {
    w: &'a mut W,
}
impl<'a> _GIWEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GIWEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GIWEW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GIWEW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GIWDW<'a> {
    w: &'a mut W,
}
impl<'a> _GIWDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Global Interrupt Write Enable"]
    #[inline]
    pub fn giwe(&mut self) -> _GIWEW {
        _GIWEW { w: self }
    }
    #[doc = "Bits 16:31 - Global Interrupt Write Data"]
    #[inline]
    pub fn giwd(&mut self) -> _GIWDW {
        _GIWDW { w: self }
    }
}
