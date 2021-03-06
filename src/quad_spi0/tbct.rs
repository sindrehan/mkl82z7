#[doc = "Reader of register TBCT"]
pub type R = crate::R<u32, super::TBCT>;
#[doc = "Writer for register TBCT"]
pub type W = crate::W<u32, super::TBCT>;
#[doc = "Register TBCT `reset()`'s with value 0"]
impl crate::ResetValue for super::TBCT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WMRK`"]
pub type WMRK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WMRK`"]
pub struct WMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> WMRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Determines the watermark for the TX Buffer"]
    #[inline(always)]
    pub fn wmrk(&self) -> WMRK_R {
        WMRK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Determines the watermark for the TX Buffer"]
    #[inline(always)]
    pub fn wmrk(&mut self) -> WMRK_W {
        WMRK_W { w: self }
    }
}
