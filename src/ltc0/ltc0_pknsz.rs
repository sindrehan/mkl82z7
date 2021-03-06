#[doc = "Reader of register LTC0_PKNSZ"]
pub type R = crate::R<u32, super::LTC0_PKNSZ>;
#[doc = "Writer for register LTC0_PKNSZ"]
pub type W = crate::W<u32, super::LTC0_PKNSZ>;
#[doc = "Register LTC0_PKNSZ `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKNSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKNSZ`"]
pub type PKNSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKNSZ`"]
pub struct PKNSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PKNSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - PKHA N Size. This is the size of the numeric value, in bytes, contained within the PKHA N Register."]
    #[inline(always)]
    pub fn pknsz(&self) -> PKNSZ_R {
        PKNSZ_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PKHA N Size. This is the size of the numeric value, in bytes, contained within the PKHA N Register."]
    #[inline(always)]
    pub fn pknsz(&mut self) -> PKNSZ_W {
        PKNSZ_W { w: self }
    }
}
