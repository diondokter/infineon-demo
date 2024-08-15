# Infineon Demo

Taken steps:
- Buy board `CY8CPROTO-062-4343W` with the `CY8C6247BZI-D44` chip
- Update the kitpro firmware using https://github.com/Infineon/Firmware-loader
  - `sudo ./fw-loader --update-kp3`
- Created PAC
- Added target to probe-rs: https://github.com/diondokter/probe-rs/tree/infineon

TODO:
- Solve weird interrupt setup problems.
  - For now many interrupts are commented out to support the main M0+ core that only supports 32 interrupts
- Probe-rs hangs when erasing flash

Links:
- Technical manuals: https://documentation.infineon.com/html/psoc6/zrs1651212645947.html
- Product page: https://www.infineon.com/cms/en/product/microcontroller/32-bit-psoc-arm-cortex-microcontroller/psoc-6-32-bit-arm-cortex-m4-mcu/cy8c6247bzi-d44
