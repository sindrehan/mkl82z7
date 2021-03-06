#[doc = "Reader of register LTC0_PKESZ"]
pub type R = crate::R<u32, super::LTC0_PKESZ>;
#[doc = "Writer for register LTC0_PKESZ"]
pub type W = crate::W<u32, super::LTC0_PKESZ>;
#[doc = "Register LTC0_PKESZ `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKESZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKESZ`"]
pub type PKESZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKESZ`"]
pub struct PKESZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PKESZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PKHA E Size. This is the size of the numeric value, in bytes, contained within the PKHA E Register."]
    #[inline(always)]
    pub fn pkesz(&self) -> PKESZ_R {
        PKESZ_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PKHA E Size. This is the size of the numeric value, in bytes, contained within the PKHA E Register."]
    #[inline(always)]
    pub fn pkesz(&mut self) -> PKESZ_W {
        PKESZ_W { w: self }
    }
}
