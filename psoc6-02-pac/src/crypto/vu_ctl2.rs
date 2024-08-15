#[doc = "Register `VU_CTL2` reader"]
pub type R = crate::R<VuCtl2Spec>;
#[doc = "Register `VU_CTL2` writer"]
pub type W = crate::W<VuCtl2Spec>;
#[doc = "Field `MASK` reader - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 8:14 - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - Specifies the size of the vector operand memory region. Legal values: '0b0000000': 32 KB memory region (VU_VTL1.ADDR\\[14:8\\]
ignored). '0b1000000': 16 KB memory region (VU_VTL1.ADDR\\[13:8\\]
ignored). '0b1100000': 8 KB memory region (VU_VTL1.ADDR\\[12:8\\]
ignored). '0b1110000': 4 KB memory region (VU_VTL1.ADDR\\[11:8\\]
ignored). '0b1111000': 2 KB memory region (VU_VTL1.ADDR\\[10:8\\]
ignored). '0b1111100': 1 KB memory region (VU_VTL1.ADDR\\[9:8\\]
ignored). '0b1111110': 512 B memory region (VU_VTL1.ADDR\\[8\\]
ignored). '0b1111111': 256 B memory region. Note: the default specifies a 256 B memory region."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<VuCtl2Spec> {
        MaskW::new(self, 8)
    }
}
#[doc = "Vector unit control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vu_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VuCtl2Spec;
impl crate::RegisterSpec for VuCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vu_ctl2::R`](R) reader structure"]
impl crate::Readable for VuCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`vu_ctl2::W`](W) writer structure"]
impl crate::Writable for VuCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VU_CTL2 to value 0x7f00"]
impl crate::Resettable for VuCtl2Spec {
    const RESET_VALUE: u32 = 0x7f00;
}
