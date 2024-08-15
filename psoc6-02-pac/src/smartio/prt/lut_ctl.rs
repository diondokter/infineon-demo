#[doc = "Register `LUT_CTL[%s]` reader"]
pub type R = crate::R<LutCtlSpec>;
#[doc = "Register `LUT_CTL[%s]` writer"]
pub type W = crate::W<LutCtlSpec>;
#[doc = "Field `LUT` reader - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
pub type LutR = crate::FieldReader;
#[doc = "Field `LUT` writer - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
pub type LutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LUT_OPC` reader - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg &lt;= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg &lt;= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable &amp; (tr1_in ^ LUT\\[2\\]) &amp; LUT\\[3\\]. clr = enable &amp; (tr0_in ^ LUT\\[0\\]) &amp; LUT\\[1\\]. Asynchronously (no clock required): lut_reg &lt;= if (clr) '0' else if (set) '1'"]
pub type LutOpcR = crate::FieldReader;
#[doc = "Field `LUT_OPC` writer - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg &lt;= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg &lt;= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable &amp; (tr1_in ^ LUT\\[2\\]) &amp; LUT\\[3\\]. clr = enable &amp; (tr0_in ^ LUT\\[0\\]) &amp; LUT\\[1\\]. Asynchronously (no clock required): lut_reg &lt;= if (clr) '0' else if (set) '1'"]
pub type LutOpcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn lut(&self) -> LutR {
        LutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg &lt;= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg &lt;= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable &amp; (tr1_in ^ LUT\\[2\\]) &amp; LUT\\[3\\]. clr = enable &amp; (tr0_in ^ LUT\\[0\\]) &amp; LUT\\[1\\]. Asynchronously (no clock required): lut_reg &lt;= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn lut_opc(&self) -> LutOpcR {
        LutOpcR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    #[must_use]
    pub fn lut(&mut self) -> LutW<LutCtlSpec> {
        LutW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg &lt;= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg &lt;= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable &amp; (tr1_in ^ LUT\\[2\\]) &amp; LUT\\[3\\]. clr = enable &amp; (tr0_in ^ LUT\\[0\\]) &amp; LUT\\[1\\]. Asynchronously (no clock required): lut_reg &lt;= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    #[must_use]
    pub fn lut_opc(&mut self) -> LutOpcW<LutCtlSpec> {
        LutOpcW::new(self, 8)
    }
}
#[doc = "LUT component control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutCtlSpec;
impl crate::RegisterSpec for LutCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_ctl::R`](R) reader structure"]
impl crate::Readable for LutCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_ctl::W`](W) writer structure"]
impl crate::Writable for LutCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUT_CTL[%s]
to value 0"]
impl crate::Resettable for LutCtlSpec {
    const RESET_VALUE: u32 = 0;
}
