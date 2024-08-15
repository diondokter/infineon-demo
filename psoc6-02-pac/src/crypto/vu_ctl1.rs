#[doc = "Register `VU_CTL1` reader"]
pub type R = crate::R<VuCtl1Spec>;
#[doc = "Register `VU_CTL1` writer"]
pub type W = crate::W<VuCtl1Spec>;
#[doc = "Field `ADDR24` reader - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
pub type Addr24R = crate::FieldReader<u32>;
#[doc = "Field `ADDR24` writer - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
pub type Addr24W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
    #[inline(always)]
    pub fn addr24(&self) -> Addr24R {
        Addr24R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Specifies the memory address for the vector unit operand memory region. The register-file registers provide 13-bit word offsets within this memory region. Given ADDR\\[31:8\\], VU_VTL2.MASK\\[14:8\\]
and a 13-bit word offset offset\\[14:2\\], a vector operand memory address VU_OPERAND_ADDR\\[31:0\\]
is calculated as follows: - VU_OPERAND_ADDR\\[31:15\\]
= ADDR\\[31:15\\]
- VU_OPERAND_ADDR\\[14:8\\]
= (ADDR\\[14:8\\]
&amp; MASK\\[14:8\\]) | (offset\\[14:8\\]
&amp; ~MASK\\[14:8\\]) - VU_OPERAND_ADDR\\[7:2\\]
= offset\\[7:2\\]
- VU_OPERAND_ADDR\\[1:0\\]
= 0 (always word aligned) The vector unit operand memory region uses either the IP's memory buffer or system memory. For best performance, the IP's memory buffer should be used and ADDR should be set to MEM_BUFF and MASK should specify the IP memory buffer size. If a vector operand memory address is mapped on a memory hole, read accesses return a '0' and write accesses are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn addr24(&mut self) -> Addr24W<VuCtl1Spec> {
        Addr24W::new(self, 8)
    }
}
#[doc = "Vector unit control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vu_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VuCtl1Spec;
impl crate::RegisterSpec for VuCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vu_ctl1::R`](R) reader structure"]
impl crate::Readable for VuCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`vu_ctl1::W`](W) writer structure"]
impl crate::Writable for VuCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VU_CTL1 to value 0"]
impl crate::Resettable for VuCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
