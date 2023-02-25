use crate::param::ConnHandle;
use crate::{FromHciBytes, FromHciBytesError, ReadHci, ReadHciError, WriteHci};

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
pub enum BroadcastFlag {
    PointToPoint,
    BrEdrBroadcast,
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AclPacketHeader {
    handle: u16,
    data_len: u16,
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

    pub fn broadcast_flag(&self) -> BroadcastFlag {
        match (self.handle >> 14) & 0x03 {
            0 => BroadcastFlag::PointToPoint,
            1 => BroadcastFlag::BrEdrBroadcast,
            2 => BroadcastFlag::Reserved,
            3 => BroadcastFlag::Reserved,
            _ => unreachable!(),
        }
    }

    pub fn data_len(&self) -> usize {
        usize::from(self.data_len)
    }
}

impl<'de> FromHciBytes<'de> for AclPacketHeader {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        if data.len() < 4 {
            Err(FromHciBytesError::InvalidSize)
        } else {
            let handle = u16::from_le_bytes(unsafe { data[0..2].try_into().unwrap_unchecked() });
            let data_len = u16::from_le_bytes(unsafe { data[2..4].try_into().unwrap_unchecked() });
            Ok((Self { handle, data_len }, 4))
        }
    }
}

impl<'de> ReadHci<'de> for AclPacketHeader {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf).await?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
    }
}

impl WriteHci for AclPacketHeader {
    fn size(&self) -> usize {
        4
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.handle.write_hci(&mut writer)?;
        self.data_len.write_hci(writer)
    }

    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.handle.write_hci_async(&mut writer).await?;
        self.data_len.write_hci_async(writer).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AclPacket<'a> {
    handle: u16,
    data: &'a [u8],
}

impl<'a> AclPacket<'a> {
    pub fn from_header_hci_bytes(header: AclPacketHeader, data: &'a [u8]) -> Result<(Self, usize), FromHciBytesError> {
        let data_len = usize::from(header.data_len);
        if data.len() < data_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            Ok((
                Self {
                    handle: header.handle,
                    data: &data[..data_len],
                },
                data_len,
            ))
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

    pub fn broadcast_flag(&self) -> BroadcastFlag {
        match (self.handle >> 14) & 0x03 {
            0 => BroadcastFlag::PointToPoint,
            1 => BroadcastFlag::BrEdrBroadcast,
            2 => BroadcastFlag::Reserved,
            3 => BroadcastFlag::Reserved,
            _ => unreachable!(),
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}

impl<'de> FromHciBytes<'de> for AclPacket<'de> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        let (header, header_len) = AclPacketHeader::from_hci_bytes(data)?;
        let data = &data[header_len..];
        Self::from_header_hci_bytes(header, data).map(|(x, y)| (x, header_len + y))
    }
}

impl<'de> ReadHci<'de> for AclPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let header = AclPacketHeader::read_hci(&mut reader, buf)?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufTooSmall)
        } else {
            reader.read_exact(&mut buf[..data_len])?;
            Self::from_header_hci_bytes(header, &buf[..data_len])
                .map(|(x, _)| x)
                .map_err(Into::into)
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let header = AclPacketHeader::read_hci_async(&mut reader, buf).await?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufTooSmall)
        } else {
            reader.read_exact(&mut buf[..data_len]).await?;
            Self::from_header_hci_bytes(header, &buf[..data_len])
                .map(|(x, _)| x)
                .map_err(Into::into)
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

    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = AclPacketHeader {
            handle: self.handle,
            data_len: self.data.len() as u16,
        };
        header.write_hci_async(&mut writer).await?;
        writer.write_all(self.data).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncPacketStatus {
    Correct,
    PossiblyInvalid,
    NoData,
    PartiallyLost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SyncPacketHeader {
    handle: u16,
    data_len: u8,
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

impl<'de> FromHciBytes<'de> for SyncPacketHeader {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        if data.len() < 3 {
            Err(FromHciBytesError::InvalidSize)
        } else {
            let handle = u16::from_le_bytes(unsafe { data[0..2].try_into().unwrap_unchecked() });
            let data_len = data[2];
            Ok((Self { handle, data_len }, 4))
        }
    }
}

impl<'de> ReadHci<'de> for SyncPacketHeader {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0; 3];
        reader.read_exact(&mut buf)?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0; 3];
        reader.read_exact(&mut buf).await?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
    }
}

impl WriteHci for SyncPacketHeader {
    fn size(&self) -> usize {
        3
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.handle.write_hci(&mut writer)?;
        self.data_len.write_hci(writer)
    }

    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.handle.write_hci_async(&mut writer).await?;
        self.data_len.write_hci_async(writer).await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SyncPacket<'a> {
    handle: u16,
    data: &'a [u8],
}

impl<'a> SyncPacket<'a> {
    pub fn from_header_hci_bytes(header: SyncPacketHeader, data: &'a [u8]) -> Result<(Self, usize), FromHciBytesError> {
        let data_len = usize::from(header.data_len);
        if data.len() < data_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            Ok((
                Self {
                    handle: header.handle,
                    data: &data[..data_len],
                },
                data_len,
            ))
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
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        let (header, header_len) = SyncPacketHeader::from_hci_bytes(data)?;
        let data = &data[header_len..];
        Self::from_header_hci_bytes(header, data).map(|(x, y)| (x, header_len + y))
    }
}

impl<'de> ReadHci<'de> for SyncPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let header = SyncPacketHeader::read_hci(&mut reader, buf)?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufTooSmall)
        } else {
            reader.read_exact(&mut buf[..data_len])?;
            Self::from_header_hci_bytes(header, &buf[..data_len])
                .map(|(x, _)| x)
                .map_err(Into::into)
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let header = SyncPacketHeader::read_hci_async(&mut reader, buf).await?;
        let data_len = header.data_len();
        if buf.len() < data_len {
            Err(ReadHciError::BufTooSmall)
        } else {
            reader.read_exact(&mut buf[..data_len]).await?;
            Self::from_header_hci_bytes(header, &buf[..data_len])
                .map(|(x, _)| x)
                .map_err(Into::into)
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

    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        let header = SyncPacketHeader {
            handle: self.handle,
            data_len: self.data.len() as u8,
        };
        header.write_hci_async(&mut writer).await?;
        writer.write_all(self.data).await
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IsoPacketHeader {
    pub handle: u16,
    pub data_load_len: u16,
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

impl<'de> FromHciBytes<'de> for IsoPacketHeader {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        if data.len() < 4 {
            Err(FromHciBytesError::InvalidSize)
        } else {
            let handle = u16::from_le_bytes(unsafe { data[0..2].try_into().unwrap_unchecked() });
            let data_load_len = u16::from_le_bytes(unsafe { data[2..4].try_into().unwrap_unchecked() });
            Ok((Self { handle, data_load_len }, 4))
        }
    }
}

impl<'de> ReadHci<'de> for IsoPacketHeader {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        _buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf).await?;
        Self::from_hci_bytes(&buf).map(|(x, _)| x).map_err(Into::into)
    }
}

impl WriteHci for IsoPacketHeader {
    fn size(&self) -> usize {
        4
    }

    fn write_hci<W: embedded_io::blocking::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.handle.write_hci(&mut writer)?;
        self.data_load_len.write_hci(writer)
    }

    async fn write_hci_async<W: embedded_io::asynch::Write>(&self, mut writer: W) -> Result<(), W::Error> {
        self.handle.write_hci_async(&mut writer).await?;
        self.data_load_len.write_hci_async(writer).await
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
    pub fn from_hci_bytes(timestamp: bool, data: &[u8]) -> Result<(Self, usize), FromHciBytesError> {
        let (timestamp, len) = if timestamp {
            u32::from_hci_bytes(data).map(|(x, y)| (Some(x), y))?
        } else {
            (None, 0)
        };
        let data = &data[len..];
        let total = len;

        let (sequence_num, len) = u16::from_hci_bytes(data)?;
        let data = &data[len..];
        let total = total + len;

        let (iso_sdu_len, len) = u16::from_hci_bytes(data)?;
        let total = total + len;

        Ok((
            Self {
                timestamp,
                sequence_num,
                iso_sdu_len,
            },
            total,
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
    pub fn from_header_hci_bytes(header: IsoPacketHeader, data: &'a [u8]) -> Result<(Self, usize), FromHciBytesError> {
        let data_load_len = usize::from(header.data_load_len);
        if data.len() < data_load_len {
            Err(FromHciBytesError::InvalidSize)
        } else {
            let (data_load_header, len) = match header.boundary_flag() {
                IsoPacketBoundary::FirstFragment | IsoPacketBoundary::Complete => {
                    IsoDataLoadHeader::from_hci_bytes(header.has_timestamp(), data).map(|(x, y)| (Some(x), y))?
                }
                IsoPacketBoundary::ContinuationFragment | IsoPacketBoundary::LastFragment => (None, 0),
            };
            let data = &data[len..];

            Ok((
                Self {
                    handle: header.handle,
                    data_load_header,
                    data: &data[..(data_load_len - len)],
                },
                data_load_len,
            ))
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
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, usize), FromHciBytesError> {
        let (header, header_len) = IsoPacketHeader::from_hci_bytes(data)?;
        let data = &data[header_len..];
        Self::from_header_hci_bytes(header, data).map(|(x, y)| (x, header_len + y))
    }
}

impl<'de> ReadHci<'de> for IsoPacket<'de> {
    fn read_hci<R: embedded_io::blocking::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let header = IsoPacketHeader::read_hci(&mut reader, buf)?;
        let data_load_len = header.data_load_len();
        if buf.len() < data_load_len {
            Err(ReadHciError::BufTooSmall)
        } else {
            reader.read_exact(&mut buf[..data_load_len])?;
            Self::from_header_hci_bytes(header, &buf[..data_load_len])
                .map(|(x, _)| x)
                .map_err(Into::into)
        }
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        mut reader: R,
        buf: &'de mut [u8],
    ) -> Result<Self, ReadHciError<R::Error>> {
        let header = IsoPacketHeader::read_hci_async(&mut reader, buf).await?;
        let data_load_len = header.data_load_len();
        if buf.len() < data_load_len {
            Err(ReadHciError::BufTooSmall)
        } else {
            reader.read_exact(&mut buf[..data_load_len]).await?;
            Self::from_header_hci_bytes(header, &buf[..data_load_len])
                .map(|(x, _)| x)
                .map_err(Into::into)
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
