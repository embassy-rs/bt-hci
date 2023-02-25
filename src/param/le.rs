use super::param;

param!(struct AddrKind(u8));

impl AddrKind {
    pub const PUBLIC: AddrKind = AddrKind(0);
    pub const RANDOM: AddrKind = AddrKind(1);
    pub const RESOLVABLE_PRIVATE_OR_PUBLIC: AddrKind = AddrKind(2);
    pub const RESOLVABLE_PRIVATE_OR_RANDOM: AddrKind = AddrKind(3);
    pub const ANONYMOUS_ADV: AddrKind = AddrKind(0xff);
}

param! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    bitfield AdvChannelMap[1] {
        (0, is_channel_37_enabled, enable_channel_37);
        (1, is_channel_38_enabled, enable_channel_38);
        (2, is_channel_39_enabled, enable_channel_39);
    }
}

impl AdvChannelMap {
    pub const ALL: AdvChannelMap = AdvChannelMap(0x07);
    pub const CHANNEL_37: AdvChannelMap = AdvChannelMap(0x01);
    pub const CHANNEL_38: AdvChannelMap = AdvChannelMap(0x02);
    pub const CHANNEL_39: AdvChannelMap = AdvChannelMap(0x04);
}

impl Default for AdvChannelMap {
    fn default() -> Self {
        Self::ALL
    }
}

param!(struct ChannelMap([u8; 5]));

impl ChannelMap {
    pub fn is_channel_bad(&self, channel: u8) -> bool {
        let byte = usize::from(channel / 8);
        let bit = channel % 8;
        (self.0[byte] & (1 << bit)) != 0
    }

    pub fn set_channel_bad(&mut self, channel: u8, bad: bool) {
        let byte = usize::from(channel / 8);
        let bit = channel % 8;
        self.0[byte] = (self.0[byte] & !(1 << bit)) | (u8::from(bad) << bit);
    }
}
