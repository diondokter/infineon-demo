#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `IDX` reader - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
pub type IdxR = crate::FieldReader;
#[doc = "Field `IDX` writer - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
pub type IdxW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VALID` reader - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IdxW<StatusSpec> {
        IdxW::new(self, 0)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. STATUS.IDX, DATA0, ..., DATA3 specify the fault. Note: Typically, HW sets this field to '1' (on an activated HW fault source that is 'enabled' by the MASK registers) and SW clears this field to '0' (typically by boot code SW (after a warm system reset, when the fault is handled). In this typical use case scenario, the HW source fault data is simultaneously captured into DATA0, ..., DATA3 when the VALID field is set to '1'. An exceptional SW use case scenario is identified as well. In this scenario, SW sets this field to '1' with a fault source index different to one of the defined HW fault sources. SW update is not restricted by the MASK registers). In both use case scenarios, the following holds: - STATUS.IDX, DATA0, ..., DATA3 can only be written when STATUS.VALID is '0'; the fault structure is not in use yet. Writing STATUS.VALID to '1' effectively locks the fault structure (until SW clears STATUS.VALID to '0'). This restriction requires a SW update to sequentially update the DATA registers followed by an update of the STATUS register. Note: For the exceptional SW use case, sequential updates to the DATA and STATUS registers may be 'interrupted' by a HW fault capture. In this case, the SW DATA register updates are overwritten by the HW update (and the STATUS.IDX field will reflect the HW capture)"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<StatusSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "Fault status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
