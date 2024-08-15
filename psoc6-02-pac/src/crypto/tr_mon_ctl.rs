#[doc = "Register `TR_MON_CTL` reader"]
pub type R = crate::R<TrMonCtlSpec>;
#[doc = "Register `TR_MON_CTL` writer"]
pub type W = crate::W<TrMonCtlSpec>;
#[doc = "Field `BITSTREAM_SEL` reader - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
pub type BitstreamSelR = crate::FieldReader;
#[doc = "Field `BITSTREAM_SEL` writer - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
pub type BitstreamSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
    #[inline(always)]
    pub fn bitstream_sel(&self) -> BitstreamSelR {
        BitstreamSelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selection of the bitstream: '0': DAS bitstream. '1': RED bitstream. '2': TR bitstream. '3': Undefined."]
    #[inline(always)]
    #[must_use]
    pub fn bitstream_sel(&mut self) -> BitstreamSelW<TrMonCtlSpec> {
        BitstreamSelW::new(self, 0)
    }
}
#[doc = "True random monitor control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonCtlSpec;
impl crate::RegisterSpec for TrMonCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_ctl::R`](R) reader structure"]
impl crate::Readable for TrMonCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_mon_ctl::W`](W) writer structure"]
impl crate::Writable for TrMonCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_MON_CTL to value 0x02"]
impl crate::Resettable for TrMonCtlSpec {
    const RESET_VALUE: u32 = 0x02;
}
