#[doc = "Reader of register LTC0_PKN3_14"]
pub type R = crate::R<u32, super::LTC0_PKN3_14>;
#[doc = "Writer for register LTC0_PKN3_14"]
pub type W = crate::W<u32, super::LTC0_PKN3_14>;
#[doc = "Register LTC0_PKN3_14 `reset()`'s with value 0"]
impl crate::ResetValue for super::LTC0_PKN3_14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PKHA_N3`"]
pub type PKHA_N3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKHA_N3`"]
pub struct PKHA_N3_W<'a> {
    w: &'a mut W,
}
impl<'a> PKHA_N3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - N3 VALUE"]
    #[inline(always)]
    pub fn pkha_n3(&self) -> PKHA_N3_R {
        PKHA_N3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - N3 VALUE"]
    #[inline(always)]
    pub fn pkha_n3(&mut self) -> PKHA_N3_W {
        PKHA_N3_W { w: self }
    }
}
