use super::param;

param!(struct CoreSpecificationVersion(u8));

impl CoreSpecificationVersion {
    pub const VERSION_1_0B: CoreSpecificationVersion = CoreSpecificationVersion(0x00);
    pub const VERSION_1_1: CoreSpecificationVersion = CoreSpecificationVersion(0x01);
    pub const VERSION_1_2: CoreSpecificationVersion = CoreSpecificationVersion(0x02);
    pub const VERSION_2_0_EDR: CoreSpecificationVersion = CoreSpecificationVersion(0x03);
    pub const VERSION_2_1_EDR: CoreSpecificationVersion = CoreSpecificationVersion(0x04);
    pub const VERSION_3_0_HS: CoreSpecificationVersion = CoreSpecificationVersion(0x05);
    pub const VERSION_4_0: CoreSpecificationVersion = CoreSpecificationVersion(0x06);
    pub const VERSION_4_1: CoreSpecificationVersion = CoreSpecificationVersion(0x07);
    pub const VERSION_4_2: CoreSpecificationVersion = CoreSpecificationVersion(0x08);
    pub const VERSION_5_0: CoreSpecificationVersion = CoreSpecificationVersion(0x09);
    pub const VERSION_5_1: CoreSpecificationVersion = CoreSpecificationVersion(0x0A);
    pub const VERSION_5_2: CoreSpecificationVersion = CoreSpecificationVersion(0x0B);
    pub const VERSION_5_3: CoreSpecificationVersion = CoreSpecificationVersion(0x0C);
}
