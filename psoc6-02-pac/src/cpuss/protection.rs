#[doc = "Register `PROTECTION` reader"]
pub type R = crate::R<ProtectionSpec>;
#[doc = "Register `PROTECTION` writer"]
pub type W = crate::W<ProtectionSpec>;
#[doc = "Field `STATE` reader - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type StateR = crate::FieldReader;
#[doc = "Field `STATE` writer - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> StateW<ProtectionSpec> {
        StateW::new(self, 0)
    }
}
#[doc = "Protection status\n\nYou can [`read`](crate::Reg::read) this register and get [`protection::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protection::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProtectionSpec;
impl crate::RegisterSpec for ProtectionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`protection::R`](R) reader structure"]
impl crate::Readable for ProtectionSpec {}
#[doc = "`write(|w| ..)` method takes [`protection::W`](W) writer structure"]
impl crate::Writable for ProtectionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROTECTION to value 0"]
impl crate::Resettable for ProtectionSpec {
    const RESET_VALUE: u32 = 0;
}
