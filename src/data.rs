use crate::param::{param, ConnHandle};
use crate::{FromHciBytes, FromHciBytesError, HostToControllerPacket, PacketKind, ReadHci, ReadHciError, WriteHci};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AclPacketBoundary {
    FirstNonFlushable,
    Continuing,
    FirstFlushable,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AclBroadcastFlag {
    PointToPoint,
    BrEdrBroadcast,
    Reserved,
}

param! {
    struct AclPacketHeader {
        handle: u16,
        data_len: u16,
    }
}

impl AclPacketHeader {
    pub fn handle(&self) -> ConnHandle {
        ConnHandle::new(self.handle & 0xeff)
    }

    pub fn boundary_flag(&self) -> AclPacketBoundary {
        match (self.handle >> 12) & 0x03 {
            0 => AclPacketBoundary::FirstNonFlushable,
            1 => AclPacketBoundary::Continuing,
            2 => AclPacketBoundary::FirstFlushable,
            3 => AclPacketBoundary::Complete,
            _ => unreachable!(),
        }
    }

    pub fn broadcast_flag(&self) -> AclBroadcastFlag {
        match (self.handle >> 14) & 0x03 {
            0 => AclBroadcastFlag::PointToPoint,
            1 => AclBroadcastFlag::BrEdrBroadcast,
            2 => AclBroadcastFlag::Reserved,
            3 => AclBroadcastFlag::Reserved,
            _ => unreachable!(),
        }
    }

    pub fn data_len(&self) -> usize {
        usize::from(self.data_len)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AclPacket<'a> {
    handle: u16,
    data: &'a [u8],
}

impl<'a> AclPacket<'a> {
    pub fn from_header_hci_bytes(header: AclPacketHeader, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
        let data_len = usize::from(header.data_len);
        if data.len() != data_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            Ok(Self {
                handle: header.handle,
                data,
            })
        }
    }

    pub fn handle(&self) -> ConnHandle {
        ConnHandle::new(self.handle & 0xeff)
    }

    pub fn boundary_flag(&self) -> AclPacketBoundary {
        match (self.handle >> 12) & 0x03 {
            0 => AclPacketBoundary::FirstNonFlushable,
            1 => AclPacketBoundary::Continuing,
            2 => AclPacketBoundary::FirstFlushable,
            3 => AclPacketBoundary::Complete,
            _ => unreachable!(),
        }
    }

    pub fn broadcast_flag(&self) -> AclBroadcastFlag {
        match (self.handle >> 14) & 0x03 {
            0 => AclBroadcastFlag::PointToPoint,
            1 => AclBroadcastFlag::BrEdrBroadcast,
            2 => AclBroadcastFlag::Reserved,
            3 => AclBroadcastFlag::Reserved,
            _ => unreachable!(),
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}

impl<'de> FromHciBytes<'de> for AclPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = AclPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data).map(|x| (x, &[] as &[u8]))
    }
}

impl<'de> ReadHci<'de> for AclPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header)?;
        let (header, _) = AclPacketHeader::from_hci_bytes(&header)?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(data_len);
            reader.read_exact(buf)?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header).await?;
        let (header, _) = AclPacketHeader::from_hci_bytes(&header)?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(data_len);
            reader.read_exact(buf).await?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }
}

impl<'a> WriteHci for AclPacket<'a> {
    fn size(&self) -> usize {
        4 + self.data.len()
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = AclPacketHeader {
            handle: self.handle,
            data_len: self.data.len() as u16,
        };
        header.write_hci(&mut writer)?;
        writer.write_all(self.data)
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = AclPacketHeader {
            handle: self.handle,
            data_len: self.data.len() as u16,
        };
        header.write_hci_async(&mut writer).await?;
        writer.write_all(self.data).await
    }
}

impl<'a> HostToControllerPacket for AclPacket<'a> {
    const KIND: PacketKind = PacketKind::AclData;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncPacketStatus {
    Correct,
    PossiblyInvalid,
    NoData,
    PartiallyLost,
}

param! {
    struct SyncPacketHeader {
        handle: u16,
        data_len: u8,
    }
}

impl SyncPacketHeader {
    pub fn handle(&self) -> ConnHandle {
        ConnHandle::new(self.handle & 0xeff)
    }

    pub fn status(&self) -> SyncPacketStatus {
        match (self.handle >> 12) & 0x03 {
            0 => SyncPacketStatus::Correct,
            1 => SyncPacketStatus::PossiblyInvalid,
            2 => SyncPacketStatus::NoData,
            3 => SyncPacketStatus::PartiallyLost,
            _ => unreachable!(),
        }
    }

    pub fn data_len(&self) -> usize {
        usize::from(self.data_len)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SyncPacket<'a> {
    handle: u16,
    data: &'a [u8],
}

impl<'a> SyncPacket<'a> {
    pub fn from_header_hci_bytes(header: SyncPacketHeader, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
        let data_len = usize::from(header.data_len);
        if data.len() != data_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            Ok(Self {
                handle: header.handle,
                data,
            })
        }
    }

    pub fn handle(&self) -> ConnHandle {
        ConnHandle::new(self.handle & 0xeff)
    }

    pub fn status(&self) -> SyncPacketStatus {
        match (self.handle >> 12) & 0x03 {
            0 => SyncPacketStatus::Correct,
            1 => SyncPacketStatus::PossiblyInvalid,
            2 => SyncPacketStatus::NoData,
            3 => SyncPacketStatus::PartiallyLost,
            _ => unreachable!(),
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}

impl<'de> FromHciBytes<'de> for SyncPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = SyncPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data).map(|x| (x, &[] as &[u8]))
    }
}

impl<'de> ReadHci<'de> for SyncPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 3];
        reader.read_exact(&mut header)?;
        let (header, _) = SyncPacketHeader::from_hci_bytes(&header)?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(data_len);
            reader.read_exact(buf)?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 3];
        reader.read_exact(&mut header).await?;
        let (header, _) = SyncPacketHeader::from_hci_bytes(&header)?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(data_len);
            reader.read_exact(buf).await?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }
}

impl<'a> WriteHci for SyncPacket<'a> {
    fn size(&self) -> usize {
        4 + self.data.len()
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = SyncPacketHeader {
            handle: self.handle,
            data_len: self.data.len() as u8,
        };
        header.write_hci(&mut writer)?;
        writer.write_all(self.data)
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = SyncPacketHeader {
            handle: self.handle,
            data_len: self.data.len() as u8,
        };
        header.write_hci_async(&mut writer).await?;
        writer.write_all(self.data).await
    }
}

impl<'a> HostToControllerPacket for SyncPacket<'a> {
    const KIND: PacketKind = PacketKind::SyncData;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IsoPacketBoundary {
    FirstFragment,
    ContinuationFragment,
    Complete,
    LastFragment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IsoPacketStatus {
    Correct,
    PossiblyInvalid,
    PartiallyLost,
}

param! {
    struct IsoPacketHeader {
        handle: u16,
        data_load_len: u16,
    }
}

impl IsoPacketHeader {
    pub fn handle(&self) -> ConnHandle {
        ConnHandle::new(self.handle & 0xeff)
    }

    pub fn boundary_flag(&self) -> IsoPacketBoundary {
        match (self.handle >> 12) & 0x03 {
            0 => IsoPacketBoundary::FirstFragment,
            1 => IsoPacketBoundary::ContinuationFragment,
            2 => IsoPacketBoundary::Complete,
            3 => IsoPacketBoundary::LastFragment,
            _ => unreachable!(),
        }
    }

    pub fn has_timestamp(&self) -> bool {
        ((self.handle >> 14) & 1) != 0
    }

    pub fn data_load_len(&self) -> usize {
        usize::from(self.data_load_len)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IsoDataLoadHeader {
    pub timestamp: Option<u32>,
    pub sequence_num: u16,
    pub iso_sdu_len: u16,
}

impl IsoDataLoadHeader {
    pub fn from_hci_bytes(timestamp: bool, data: &[u8]) -> Result<(Self, &[u8]), FromHciBytesError> {
        let (timestamp, data) = if timestamp {
            u32::from_hci_bytes(data).map(|(x, y)| (Some(x), y))?
        } else {
            (None, data)
        };

        let (sequence_num, data) = u16::from_hci_bytes(data)?;
        let (iso_sdu_len, data) = u16::from_hci_bytes(data)?;

        Ok((
            Self {
                timestamp,
                sequence_num,
                iso_sdu_len,
            },
            data,
        ))
    }
}

impl WriteHci for IsoDataLoadHeader {
    fn size(&self) -> usize {
        if self.timestamp.is_some() {
            8
        } else {
            4
        }
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        if let Some(timestamp) = self.timestamp {
            timestamp.write_hci(&mut writer)?;
        }
        self.sequence_num.write_hci(&mut writer)?;
        self.iso_sdu_len.write_hci(writer)
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        if let Some(timestamp) = self.timestamp {
            timestamp.write_hci_async(&mut writer).await?;
        }
        self.sequence_num.write_hci_async(&mut writer).await?;
        self.iso_sdu_len.write_hci_async(writer).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IsoPacket<'a> {
    handle: u16,
    data_load_header: Option<IsoDataLoadHeader>,
    data: &'a [u8],
}

impl<'a> IsoPacket<'a> {
    pub fn from_header_hci_bytes(header: IsoPacketHeader, data: &'a [u8]) -> Result<Self, FromHciBytesError> {
        let data_load_len = usize::from(header.data_load_len);
        if data.len() != data_load_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            let (data_load_header, data) = match header.boundary_flag() {
                IsoPacketBoundary::FirstFragment | IsoPacketBoundary::Complete => {
                    IsoDataLoadHeader::from_hci_bytes(header.has_timestamp(), data).map(|(x, y)| (Some(x), y))?
                }
                IsoPacketBoundary::ContinuationFragment | IsoPacketBoundary::LastFragment => (None, data),
            };

            Ok(Self {
                handle: header.handle,
                data_load_header,
                data,
            })
        }
    }

    pub fn handle(&self) -> ConnHandle {
        ConnHandle::new(self.handle & 0xeff)
    }

    pub fn boundary_flag(&self) -> IsoPacketBoundary {
        match (self.handle >> 12) & 0x03 {
            0 => IsoPacketBoundary::FirstFragment,
            1 => IsoPacketBoundary::ContinuationFragment,
            2 => IsoPacketBoundary::Complete,
            3 => IsoPacketBoundary::LastFragment,
            _ => unreachable!(),
        }
    }

    pub fn has_timestamp(&self) -> bool {
        ((self.handle >> 14) & 1) != 0
    }

    pub fn data_load_header(&self) -> Option<IsoDataLoadHeader> {
        self.data_load_header
    }

    pub fn data_load_len(&self) -> usize {
        self.data_load_header.as_ref().map(|x| x.size()).unwrap_or(0) + self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}

impl<'de> FromHciBytes<'de> for IsoPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        let (header, data) = IsoPacketHeader::from_hci_bytes(data)?;
        Self::from_header_hci_bytes(header, data).map(|x| (x, &[] as &[u8]))
    }
}

impl<'de> ReadHci<'de> for IsoPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header)?;
        let (header, _) = IsoPacketHeader::from_hci_bytes(&header)?;
        let data_load_len = header.data_load_len();
        if buf.len() < data_load_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(data_load_len);
            reader.read_exact(buf)?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut header = [0; 4];
        reader.read_exact(&mut header).await?;
        let (header, _) = IsoPacketHeader::from_hci_bytes(&header)?;
        let data_load_len = header.data_load_len();
        if buf.len() < data_load_len {
            Err(ReadHciError::BufferTooSmall)
        } else {
            let (buf, _) = buf.split_at_mut(data_load_len);
            reader.read_exact(buf).await?;
            Self::from_header_hci_bytes(header, buf).map_err(Into::into)
        }
    }
}

impl<'a> WriteHci for IsoPacket<'a> {
    fn size(&self) -> usize {
        4 + self.data_load_len()
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = IsoPacketHeader {
            handle: self.handle,
            data_load_len: self.data_load_len() as u16,
        };
        header.write_hci(&mut writer)?;
        if let Some(data_load_header) = &self.data_load_header {
            data_load_header.write_hci(&mut writer)?;
        }
        writer.write_all(self.data)
    }

    #[cfg(feature = "async")]
    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = IsoPacketHeader {
            handle: self.handle,
            data_load_len: self.data_load_len() as u16,
        };
        header.write_hci_async(&mut writer).await?;
        if let Some(data_load_header) = &self.data_load_header {
            data_load_header.write_hci_async(&mut writer).await?;
        }
        writer.write_all(self.data).await
    }
}

impl<'a> HostToControllerPacket for IsoPacket<'a> {
    const KIND: PacketKind = PacketKind::IsoData;
}
