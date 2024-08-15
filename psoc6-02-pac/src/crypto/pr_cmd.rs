#[doc = "Register `PR_CMD` reader"]
pub type R = crate::R<PrCmdSpec>;
#[doc = "Register `PR_CMD` writer"]
pub type W = crate::W<PrCmdSpec>;
#[doc = "Field `START` reader - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pseudo random command. On a generated number, HW sets this field to '0' and sets INTR.PR_DATA_AVAILABLE to '1."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<PrCmdSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "Pseudo random command\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrCmdSpec;
impl crate::RegisterSpec for PrCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_cmd::R`](R) reader structure"]
impl crate::Readable for PrCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`pr_cmd::W`](W) writer structure"]
impl crate::Writable for PrCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_CMD to value 0"]
impl crate::Resettable for PrCmdSpec {
    const RESET_VALUE: u32 = 0;
}
