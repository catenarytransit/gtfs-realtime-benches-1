// Automatically generated rust module for 'gtfs-realtime.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeedMessage<'a> {
    pub header: transit_realtime::FeedHeader<'a>,
    pub entity: Vec<transit_realtime::FeedEntity<'a>>,
}

impl<'a> MessageRead<'a> for FeedMessage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.header = r.read_message::<transit_realtime::FeedHeader>(bytes)?,
                Ok(18) => msg.entity.push(r.read_message::<transit_realtime::FeedEntity>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FeedMessage<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.header).get_size())
        + self.entity.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.header))?;
        for s in &self.entity { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeedHeader<'a> {
    pub gtfs_realtime_version: Cow<'a, str>,
    pub incrementality: transit_realtime::mod_FeedHeader::Incrementality,
    pub timestamp: Option<u64>,
}

impl<'a> MessageRead<'a> for FeedHeader<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.gtfs_realtime_version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.incrementality = r.read_enum(bytes)?,
                Ok(24) => msg.timestamp = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FeedHeader<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.gtfs_realtime_version).len())
        + if self.incrementality == transit_realtime::mod_FeedHeader::Incrementality::FULL_DATASET { 0 } else { 1 + sizeof_varint(*(&self.incrementality) as u64) }
        + self.timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.gtfs_realtime_version))?;
        if self.incrementality != transit_realtime::mod_FeedHeader::Incrementality::FULL_DATASET { w.write_with_tag(16, |w| w.write_enum(*&self.incrementality as i32))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(24, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

pub mod mod_FeedHeader {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Incrementality {
    FULL_DATASET = 0,
    DIFFERENTIAL = 1,
}

impl Default for Incrementality {
    fn default() -> Self {
        Incrementality::FULL_DATASET
    }
}

impl From<i32> for Incrementality {
    fn from(i: i32) -> Self {
        match i {
            0 => Incrementality::FULL_DATASET,
            1 => Incrementality::DIFFERENTIAL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Incrementality {
    fn from(s: &'a str) -> Self {
        match s {
            "FULL_DATASET" => Incrementality::FULL_DATASET,
            "DIFFERENTIAL" => Incrementality::DIFFERENTIAL,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeedEntity<'a> {
    pub id: Cow<'a, str>,
    pub is_deleted: bool,
    pub trip_update: Option<transit_realtime::TripUpdate<'a>>,
    pub vehicle: Option<transit_realtime::VehiclePosition<'a>>,
    pub alert: Option<transit_realtime::Alert<'a>>,
    pub shape: Option<transit_realtime::Shape<'a>>,
    pub stop: Option<transit_realtime::Stop<'a>>,
    pub trip_modifications: Option<transit_realtime::TripModifications<'a>>,
}

impl<'a> MessageRead<'a> for FeedEntity<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.is_deleted = r.read_bool(bytes)?,
                Ok(26) => msg.trip_update = Some(r.read_message::<transit_realtime::TripUpdate>(bytes)?),
                Ok(34) => msg.vehicle = Some(r.read_message::<transit_realtime::VehiclePosition>(bytes)?),
                Ok(42) => msg.alert = Some(r.read_message::<transit_realtime::Alert>(bytes)?),
                Ok(50) => msg.shape = Some(r.read_message::<transit_realtime::Shape>(bytes)?),
                Ok(58) => msg.stop = Some(r.read_message::<transit_realtime::Stop>(bytes)?),
                Ok(66) => msg.trip_modifications = Some(r.read_message::<transit_realtime::TripModifications>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FeedEntity<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + if self.is_deleted == false { 0 } else { 1 + sizeof_varint(*(&self.is_deleted) as u64) }
        + self.trip_update.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.vehicle.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.alert.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.shape.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stop.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.trip_modifications.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        if self.is_deleted != false { w.write_with_tag(16, |w| w.write_bool(*&self.is_deleted))?; }
        if let Some(ref s) = self.trip_update { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.vehicle { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.alert { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.shape { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stop { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.trip_modifications { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TripUpdate<'a> {
    pub trip: transit_realtime::TripDescriptor<'a>,
    pub vehicle: Option<transit_realtime::VehicleDescriptor<'a>>,
    pub stop_time_update: Vec<transit_realtime::mod_TripUpdate::StopTimeUpdate<'a>>,
    pub timestamp: Option<u64>,
    pub delay: Option<i32>,
    pub trip_properties: Option<transit_realtime::mod_TripUpdate::TripProperties<'a>>,
}

impl<'a> MessageRead<'a> for TripUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.trip = r.read_message::<transit_realtime::TripDescriptor>(bytes)?,
                Ok(26) => msg.vehicle = Some(r.read_message::<transit_realtime::VehicleDescriptor>(bytes)?),
                Ok(18) => msg.stop_time_update.push(r.read_message::<transit_realtime::mod_TripUpdate::StopTimeUpdate>(bytes)?),
                Ok(32) => msg.timestamp = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.delay = Some(r.read_int32(bytes)?),
                Ok(50) => msg.trip_properties = Some(r.read_message::<transit_realtime::mod_TripUpdate::TripProperties>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TripUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.trip).get_size())
        + self.vehicle.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stop_time_update.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.delay.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.trip_properties.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.trip))?;
        if let Some(ref s) = self.vehicle { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.stop_time_update { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(32, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.delay { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.trip_properties { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TripUpdate {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StopTimeEvent {
    pub delay: Option<i32>,
    pub time: Option<i64>,
    pub uncertainty: Option<i32>,
}

impl<'a> MessageRead<'a> for StopTimeEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.delay = Some(r.read_int32(bytes)?),
                Ok(16) => msg.time = Some(r.read_int64(bytes)?),
                Ok(24) => msg.uncertainty = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for StopTimeEvent {
    fn get_size(&self) -> usize {
        0
        + self.delay.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.uncertainty.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.delay { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.time { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.uncertainty { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StopTimeUpdate<'a> {
    pub stop_sequence: Option<u32>,
    pub stop_id: Option<Cow<'a, str>>,
    pub arrival: Option<transit_realtime::mod_TripUpdate::StopTimeEvent>,
    pub departure: Option<transit_realtime::mod_TripUpdate::StopTimeEvent>,
    pub departure_occupancy_status: Option<transit_realtime::mod_VehiclePosition::OccupancyStatus>,
    pub schedule_relationship: transit_realtime::mod_TripUpdate::mod_StopTimeUpdate::ScheduleRelationship,
    pub stop_time_properties: Option<transit_realtime::mod_TripUpdate::mod_StopTimeUpdate::StopTimeProperties<'a>>,
}

impl<'a> MessageRead<'a> for StopTimeUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.stop_sequence = Some(r.read_uint32(bytes)?),
                Ok(34) => msg.stop_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.arrival = Some(r.read_message::<transit_realtime::mod_TripUpdate::StopTimeEvent>(bytes)?),
                Ok(26) => msg.departure = Some(r.read_message::<transit_realtime::mod_TripUpdate::StopTimeEvent>(bytes)?),
                Ok(56) => msg.departure_occupancy_status = Some(r.read_enum(bytes)?),
                Ok(40) => msg.schedule_relationship = r.read_enum(bytes)?,
                Ok(50) => msg.stop_time_properties = Some(r.read_message::<transit_realtime::mod_TripUpdate::mod_StopTimeUpdate::StopTimeProperties>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StopTimeUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + self.stop_sequence.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stop_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.arrival.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.departure.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.departure_occupancy_status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + if self.schedule_relationship == transit_realtime::mod_TripUpdate::mod_StopTimeUpdate::ScheduleRelationship::SCHEDULED { 0 } else { 1 + sizeof_varint(*(&self.schedule_relationship) as u64) }
        + self.stop_time_properties.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.stop_sequence { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.stop_id { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.arrival { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.departure { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.departure_occupancy_status { w.write_with_tag(56, |w| w.write_enum(*s as i32))?; }
        if self.schedule_relationship != transit_realtime::mod_TripUpdate::mod_StopTimeUpdate::ScheduleRelationship::SCHEDULED { w.write_with_tag(40, |w| w.write_enum(*&self.schedule_relationship as i32))?; }
        if let Some(ref s) = self.stop_time_properties { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_StopTimeUpdate {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StopTimeProperties<'a> {
    pub assigned_stop_id: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for StopTimeProperties<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.assigned_stop_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StopTimeProperties<'a> {
    fn get_size(&self) -> usize {
        0
        + self.assigned_stop_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.assigned_stop_id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScheduleRelationship {
    SCHEDULED = 0,
    SKIPPED = 1,
    NO_DATA = 2,
    UNSCHEDULED = 3,
}

impl Default for ScheduleRelationship {
    fn default() -> Self {
        ScheduleRelationship::SCHEDULED
    }
}

impl From<i32> for ScheduleRelationship {
    fn from(i: i32) -> Self {
        match i {
            0 => ScheduleRelationship::SCHEDULED,
            1 => ScheduleRelationship::SKIPPED,
            2 => ScheduleRelationship::NO_DATA,
            3 => ScheduleRelationship::UNSCHEDULED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ScheduleRelationship {
    fn from(s: &'a str) -> Self {
        match s {
            "SCHEDULED" => ScheduleRelationship::SCHEDULED,
            "SKIPPED" => ScheduleRelationship::SKIPPED,
            "NO_DATA" => ScheduleRelationship::NO_DATA,
            "UNSCHEDULED" => ScheduleRelationship::UNSCHEDULED,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TripProperties<'a> {
    pub trip_id: Option<Cow<'a, str>>,
    pub start_date: Option<Cow<'a, str>>,
    pub start_time: Option<Cow<'a, str>>,
    pub shape_id: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for TripProperties<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.trip_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.start_date = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.start_time = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.shape_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TripProperties<'a> {
    fn get_size(&self) -> usize {
        0
        + self.trip_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.start_date.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.start_time.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.shape_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.trip_id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.start_date { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.start_time { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.shape_id { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct VehiclePosition<'a> {
    pub trip: Option<transit_realtime::TripDescriptor<'a>>,
    pub vehicle: Option<transit_realtime::VehicleDescriptor<'a>>,
    pub position: Option<transit_realtime::Position>,
    pub current_stop_sequence: Option<u32>,
    pub stop_id: Option<Cow<'a, str>>,
    pub current_status: transit_realtime::mod_VehiclePosition::VehicleStopStatus,
    pub timestamp: Option<u64>,
    pub congestion_level: Option<transit_realtime::mod_VehiclePosition::CongestionLevel>,
    pub occupancy_status: Option<transit_realtime::mod_VehiclePosition::OccupancyStatus>,
    pub occupancy_percentage: Option<u32>,
    pub multi_carriage_details: Vec<transit_realtime::mod_VehiclePosition::CarriageDetails<'a>>,
}

impl<'a> MessageRead<'a> for VehiclePosition<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = VehiclePosition {
            current_status: transit_realtime::mod_VehiclePosition::VehicleStopStatus::IN_TRANSIT_TO,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.trip = Some(r.read_message::<transit_realtime::TripDescriptor>(bytes)?),
                Ok(66) => msg.vehicle = Some(r.read_message::<transit_realtime::VehicleDescriptor>(bytes)?),
                Ok(18) => msg.position = Some(r.read_message::<transit_realtime::Position>(bytes)?),
                Ok(24) => msg.current_stop_sequence = Some(r.read_uint32(bytes)?),
                Ok(58) => msg.stop_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.current_status = r.read_enum(bytes)?,
                Ok(40) => msg.timestamp = Some(r.read_uint64(bytes)?),
                Ok(48) => msg.congestion_level = Some(r.read_enum(bytes)?),
                Ok(72) => msg.occupancy_status = Some(r.read_enum(bytes)?),
                Ok(80) => msg.occupancy_percentage = Some(r.read_uint32(bytes)?),
                Ok(90) => msg.multi_carriage_details.push(r.read_message::<transit_realtime::mod_VehiclePosition::CarriageDetails>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for VehiclePosition<'a> {
    fn get_size(&self) -> usize {
        0
        + self.trip.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.vehicle.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.position.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.current_stop_sequence.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stop_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.current_status == transit_realtime::mod_VehiclePosition::VehicleStopStatus::IN_TRANSIT_TO { 0 } else { 1 + sizeof_varint(*(&self.current_status) as u64) }
        + self.timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.congestion_level.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.occupancy_status.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.occupancy_percentage.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.multi_carriage_details.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.trip { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.vehicle { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.position { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.current_stop_sequence { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.stop_id { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if self.current_status != transit_realtime::mod_VehiclePosition::VehicleStopStatus::IN_TRANSIT_TO { w.write_with_tag(32, |w| w.write_enum(*&self.current_status as i32))?; }
        if let Some(ref s) = self.timestamp { w.write_with_tag(40, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.congestion_level { w.write_with_tag(48, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.occupancy_status { w.write_with_tag(72, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.occupancy_percentage { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        for s in &self.multi_carriage_details { w.write_with_tag(90, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_VehiclePosition {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CarriageDetails<'a> {
    pub id: Option<Cow<'a, str>>,
    pub label: Option<Cow<'a, str>>,
    pub occupancy_status: transit_realtime::mod_VehiclePosition::OccupancyStatus,
    pub occupancy_percentage: i32,
    pub carriage_sequence: Option<u32>,
}

impl<'a> MessageRead<'a> for CarriageDetails<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = CarriageDetails {
            occupancy_status: transit_realtime::mod_VehiclePosition::OccupancyStatus::NO_DATA_AVAILABLE,
            occupancy_percentage: -1i32,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.label = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.occupancy_status = r.read_enum(bytes)?,
                Ok(32) => msg.occupancy_percentage = r.read_int32(bytes)?,
                Ok(40) => msg.carriage_sequence = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CarriageDetails<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.label.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.occupancy_status == transit_realtime::mod_VehiclePosition::OccupancyStatus::NO_DATA_AVAILABLE { 0 } else { 1 + sizeof_varint(*(&self.occupancy_status) as u64) }
        + if self.occupancy_percentage == -1i32 { 0 } else { 1 + sizeof_varint(*(&self.occupancy_percentage) as u64) }
        + self.carriage_sequence.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.label { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if self.occupancy_status != transit_realtime::mod_VehiclePosition::OccupancyStatus::NO_DATA_AVAILABLE { w.write_with_tag(24, |w| w.write_enum(*&self.occupancy_status as i32))?; }
        if self.occupancy_percentage != -1i32 { w.write_with_tag(32, |w| w.write_int32(*&self.occupancy_percentage))?; }
        if let Some(ref s) = self.carriage_sequence { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VehicleStopStatus {
    INCOMING_AT = 0,
    STOPPED_AT = 1,
    IN_TRANSIT_TO = 2,
}

impl Default for VehicleStopStatus {
    fn default() -> Self {
        VehicleStopStatus::INCOMING_AT
    }
}

impl From<i32> for VehicleStopStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => VehicleStopStatus::INCOMING_AT,
            1 => VehicleStopStatus::STOPPED_AT,
            2 => VehicleStopStatus::IN_TRANSIT_TO,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for VehicleStopStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "INCOMING_AT" => VehicleStopStatus::INCOMING_AT,
            "STOPPED_AT" => VehicleStopStatus::STOPPED_AT,
            "IN_TRANSIT_TO" => VehicleStopStatus::IN_TRANSIT_TO,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CongestionLevel {
    UNKNOWN_CONGESTION_LEVEL = 0,
    RUNNING_SMOOTHLY = 1,
    STOP_AND_GO = 2,
    CONGESTION = 3,
    SEVERE_CONGESTION = 4,
}

impl Default for CongestionLevel {
    fn default() -> Self {
        CongestionLevel::UNKNOWN_CONGESTION_LEVEL
    }
}

impl From<i32> for CongestionLevel {
    fn from(i: i32) -> Self {
        match i {
            0 => CongestionLevel::UNKNOWN_CONGESTION_LEVEL,
            1 => CongestionLevel::RUNNING_SMOOTHLY,
            2 => CongestionLevel::STOP_AND_GO,
            3 => CongestionLevel::CONGESTION,
            4 => CongestionLevel::SEVERE_CONGESTION,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CongestionLevel {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_CONGESTION_LEVEL" => CongestionLevel::UNKNOWN_CONGESTION_LEVEL,
            "RUNNING_SMOOTHLY" => CongestionLevel::RUNNING_SMOOTHLY,
            "STOP_AND_GO" => CongestionLevel::STOP_AND_GO,
            "CONGESTION" => CongestionLevel::CONGESTION,
            "SEVERE_CONGESTION" => CongestionLevel::SEVERE_CONGESTION,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OccupancyStatus {
    EMPTY = 0,
    MANY_SEATS_AVAILABLE = 1,
    FEW_SEATS_AVAILABLE = 2,
    STANDING_ROOM_ONLY = 3,
    CRUSHED_STANDING_ROOM_ONLY = 4,
    FULL = 5,
    NOT_ACCEPTING_PASSENGERS = 6,
    NO_DATA_AVAILABLE = 7,
    NOT_BOARDABLE = 8,
}

impl Default for OccupancyStatus {
    fn default() -> Self {
        OccupancyStatus::EMPTY
    }
}

impl From<i32> for OccupancyStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => OccupancyStatus::EMPTY,
            1 => OccupancyStatus::MANY_SEATS_AVAILABLE,
            2 => OccupancyStatus::FEW_SEATS_AVAILABLE,
            3 => OccupancyStatus::STANDING_ROOM_ONLY,
            4 => OccupancyStatus::CRUSHED_STANDING_ROOM_ONLY,
            5 => OccupancyStatus::FULL,
            6 => OccupancyStatus::NOT_ACCEPTING_PASSENGERS,
            7 => OccupancyStatus::NO_DATA_AVAILABLE,
            8 => OccupancyStatus::NOT_BOARDABLE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for OccupancyStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "EMPTY" => OccupancyStatus::EMPTY,
            "MANY_SEATS_AVAILABLE" => OccupancyStatus::MANY_SEATS_AVAILABLE,
            "FEW_SEATS_AVAILABLE" => OccupancyStatus::FEW_SEATS_AVAILABLE,
            "STANDING_ROOM_ONLY" => OccupancyStatus::STANDING_ROOM_ONLY,
            "CRUSHED_STANDING_ROOM_ONLY" => OccupancyStatus::CRUSHED_STANDING_ROOM_ONLY,
            "FULL" => OccupancyStatus::FULL,
            "NOT_ACCEPTING_PASSENGERS" => OccupancyStatus::NOT_ACCEPTING_PASSENGERS,
            "NO_DATA_AVAILABLE" => OccupancyStatus::NO_DATA_AVAILABLE,
            "NOT_BOARDABLE" => OccupancyStatus::NOT_BOARDABLE,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Alert<'a> {
    pub active_period: Vec<transit_realtime::TimeRange>,
    pub informed_entity: Vec<transit_realtime::EntitySelector<'a>>,
    pub cause: transit_realtime::mod_Alert::Cause,
    pub effect: transit_realtime::mod_Alert::Effect,
    pub url: Option<transit_realtime::TranslatedString<'a>>,
    pub header_text: Option<transit_realtime::TranslatedString<'a>>,
    pub description_text: Option<transit_realtime::TranslatedString<'a>>,
    pub tts_header_text: Option<transit_realtime::TranslatedString<'a>>,
    pub tts_description_text: Option<transit_realtime::TranslatedString<'a>>,
    pub severity_level: transit_realtime::mod_Alert::SeverityLevel,
    pub image: Option<transit_realtime::TranslatedImage<'a>>,
    pub image_alternative_text: Option<transit_realtime::TranslatedString<'a>>,
    pub cause_detail: Option<transit_realtime::TranslatedString<'a>>,
    pub effect_detail: Option<transit_realtime::TranslatedString<'a>>,
}

impl<'a> MessageRead<'a> for Alert<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Alert {
            effect: transit_realtime::mod_Alert::Effect::UNKNOWN_EFFECT,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.active_period.push(r.read_message::<transit_realtime::TimeRange>(bytes)?),
                Ok(42) => msg.informed_entity.push(r.read_message::<transit_realtime::EntitySelector>(bytes)?),
                Ok(48) => msg.cause = r.read_enum(bytes)?,
                Ok(56) => msg.effect = r.read_enum(bytes)?,
                Ok(66) => msg.url = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(82) => msg.header_text = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(90) => msg.description_text = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(98) => msg.tts_header_text = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(106) => msg.tts_description_text = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(112) => msg.severity_level = r.read_enum(bytes)?,
                Ok(122) => msg.image = Some(r.read_message::<transit_realtime::TranslatedImage>(bytes)?),
                Ok(130) => msg.image_alternative_text = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(138) => msg.cause_detail = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(146) => msg.effect_detail = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Alert<'a> {
    fn get_size(&self) -> usize {
        0
        + self.active_period.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.informed_entity.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.cause == transit_realtime::mod_Alert::Cause::UNKNOWN_CAUSE { 0 } else { 1 + sizeof_varint(*(&self.cause) as u64) }
        + if self.effect == transit_realtime::mod_Alert::Effect::UNKNOWN_EFFECT { 0 } else { 1 + sizeof_varint(*(&self.effect) as u64) }
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.header_text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.description_text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tts_header_text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tts_description_text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.severity_level == transit_realtime::mod_Alert::SeverityLevel::UNKNOWN_SEVERITY { 0 } else { 1 + sizeof_varint(*(&self.severity_level) as u64) }
        + self.image.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.image_alternative_text.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.cause_detail.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.effect_detail.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.active_period { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.informed_entity { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.cause != transit_realtime::mod_Alert::Cause::UNKNOWN_CAUSE { w.write_with_tag(48, |w| w.write_enum(*&self.cause as i32))?; }
        if self.effect != transit_realtime::mod_Alert::Effect::UNKNOWN_EFFECT { w.write_with_tag(56, |w| w.write_enum(*&self.effect as i32))?; }
        if let Some(ref s) = self.url { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.header_text { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.description_text { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tts_header_text { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tts_description_text { w.write_with_tag(106, |w| w.write_message(s))?; }
        if self.severity_level != transit_realtime::mod_Alert::SeverityLevel::UNKNOWN_SEVERITY { w.write_with_tag(112, |w| w.write_enum(*&self.severity_level as i32))?; }
        if let Some(ref s) = self.image { w.write_with_tag(122, |w| w.write_message(s))?; }
        if let Some(ref s) = self.image_alternative_text { w.write_with_tag(130, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cause_detail { w.write_with_tag(138, |w| w.write_message(s))?; }
        if let Some(ref s) = self.effect_detail { w.write_with_tag(146, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Alert {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cause {
    UNKNOWN_CAUSE = 1,
    OTHER_CAUSE = 2,
    TECHNICAL_PROBLEM = 3,
    STRIKE = 4,
    DEMONSTRATION = 5,
    ACCIDENT = 6,
    HOLIDAY = 7,
    WEATHER = 8,
    MAINTENANCE = 9,
    CONSTRUCTION = 10,
    POLICE_ACTIVITY = 11,
    MEDICAL_EMERGENCY = 12,
}

impl Default for Cause {
    fn default() -> Self {
        Cause::UNKNOWN_CAUSE
    }
}

impl From<i32> for Cause {
    fn from(i: i32) -> Self {
        match i {
            1 => Cause::UNKNOWN_CAUSE,
            2 => Cause::OTHER_CAUSE,
            3 => Cause::TECHNICAL_PROBLEM,
            4 => Cause::STRIKE,
            5 => Cause::DEMONSTRATION,
            6 => Cause::ACCIDENT,
            7 => Cause::HOLIDAY,
            8 => Cause::WEATHER,
            9 => Cause::MAINTENANCE,
            10 => Cause::CONSTRUCTION,
            11 => Cause::POLICE_ACTIVITY,
            12 => Cause::MEDICAL_EMERGENCY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Cause {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_CAUSE" => Cause::UNKNOWN_CAUSE,
            "OTHER_CAUSE" => Cause::OTHER_CAUSE,
            "TECHNICAL_PROBLEM" => Cause::TECHNICAL_PROBLEM,
            "STRIKE" => Cause::STRIKE,
            "DEMONSTRATION" => Cause::DEMONSTRATION,
            "ACCIDENT" => Cause::ACCIDENT,
            "HOLIDAY" => Cause::HOLIDAY,
            "WEATHER" => Cause::WEATHER,
            "MAINTENANCE" => Cause::MAINTENANCE,
            "CONSTRUCTION" => Cause::CONSTRUCTION,
            "POLICE_ACTIVITY" => Cause::POLICE_ACTIVITY,
            "MEDICAL_EMERGENCY" => Cause::MEDICAL_EMERGENCY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Effect {
    NO_SERVICE = 1,
    REDUCED_SERVICE = 2,
    SIGNIFICANT_DELAYS = 3,
    DETOUR = 4,
    ADDITIONAL_SERVICE = 5,
    MODIFIED_SERVICE = 6,
    OTHER_EFFECT = 7,
    UNKNOWN_EFFECT = 8,
    STOP_MOVED = 9,
    NO_EFFECT = 10,
    ACCESSIBILITY_ISSUE = 11,
}

impl Default for Effect {
    fn default() -> Self {
        Effect::NO_SERVICE
    }
}

impl From<i32> for Effect {
    fn from(i: i32) -> Self {
        match i {
            1 => Effect::NO_SERVICE,
            2 => Effect::REDUCED_SERVICE,
            3 => Effect::SIGNIFICANT_DELAYS,
            4 => Effect::DETOUR,
            5 => Effect::ADDITIONAL_SERVICE,
            6 => Effect::MODIFIED_SERVICE,
            7 => Effect::OTHER_EFFECT,
            8 => Effect::UNKNOWN_EFFECT,
            9 => Effect::STOP_MOVED,
            10 => Effect::NO_EFFECT,
            11 => Effect::ACCESSIBILITY_ISSUE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Effect {
    fn from(s: &'a str) -> Self {
        match s {
            "NO_SERVICE" => Effect::NO_SERVICE,
            "REDUCED_SERVICE" => Effect::REDUCED_SERVICE,
            "SIGNIFICANT_DELAYS" => Effect::SIGNIFICANT_DELAYS,
            "DETOUR" => Effect::DETOUR,
            "ADDITIONAL_SERVICE" => Effect::ADDITIONAL_SERVICE,
            "MODIFIED_SERVICE" => Effect::MODIFIED_SERVICE,
            "OTHER_EFFECT" => Effect::OTHER_EFFECT,
            "UNKNOWN_EFFECT" => Effect::UNKNOWN_EFFECT,
            "STOP_MOVED" => Effect::STOP_MOVED,
            "NO_EFFECT" => Effect::NO_EFFECT,
            "ACCESSIBILITY_ISSUE" => Effect::ACCESSIBILITY_ISSUE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SeverityLevel {
    UNKNOWN_SEVERITY = 1,
    INFO = 2,
    WARNING = 3,
    SEVERE = 4,
}

impl Default for SeverityLevel {
    fn default() -> Self {
        SeverityLevel::UNKNOWN_SEVERITY
    }
}

impl From<i32> for SeverityLevel {
    fn from(i: i32) -> Self {
        match i {
            1 => SeverityLevel::UNKNOWN_SEVERITY,
            2 => SeverityLevel::INFO,
            3 => SeverityLevel::WARNING,
            4 => SeverityLevel::SEVERE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SeverityLevel {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_SEVERITY" => SeverityLevel::UNKNOWN_SEVERITY,
            "INFO" => SeverityLevel::INFO,
            "WARNING" => SeverityLevel::WARNING,
            "SEVERE" => SeverityLevel::SEVERE,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TimeRange {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

impl<'a> MessageRead<'a> for TimeRange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.start = Some(r.read_uint64(bytes)?),
                Ok(16) => msg.end = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TimeRange {
    fn get_size(&self) -> usize {
        0
        + self.start.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.end.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.start { w.write_with_tag(8, |w| w.write_uint64(*s))?; }
        if let Some(ref s) = self.end { w.write_with_tag(16, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
    pub bearing: Option<f32>,
    pub odometer: Option<f64>,
    pub speed: Option<f32>,
}

impl<'a> MessageRead<'a> for Position {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.latitude = r.read_float(bytes)?,
                Ok(21) => msg.longitude = r.read_float(bytes)?,
                Ok(29) => msg.bearing = Some(r.read_float(bytes)?),
                Ok(33) => msg.odometer = Some(r.read_double(bytes)?),
                Ok(45) => msg.speed = Some(r.read_float(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Position {
    fn get_size(&self) -> usize {
        0
        + 1 + 4
        + 1 + 4
        + self.bearing.as_ref().map_or(0, |_| 1 + 4)
        + self.odometer.as_ref().map_or(0, |_| 1 + 8)
        + self.speed.as_ref().map_or(0, |_| 1 + 4)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(13, |w| w.write_float(*&self.latitude))?;
        w.write_with_tag(21, |w| w.write_float(*&self.longitude))?;
        if let Some(ref s) = self.bearing { w.write_with_tag(29, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.odometer { w.write_with_tag(33, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.speed { w.write_with_tag(45, |w| w.write_float(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TripDescriptor<'a> {
    pub trip_id: Option<Cow<'a, str>>,
    pub route_id: Option<Cow<'a, str>>,
    pub direction_id: Option<u32>,
    pub start_time: Option<Cow<'a, str>>,
    pub start_date: Option<Cow<'a, str>>,
    pub schedule_relationship: Option<transit_realtime::mod_TripDescriptor::ScheduleRelationship>,
    pub modified_trip: Option<transit_realtime::mod_TripDescriptor::ModifiedTripSelector<'a>>,
}

impl<'a> MessageRead<'a> for TripDescriptor<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.trip_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.route_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.direction_id = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.start_time = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.start_date = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.schedule_relationship = Some(r.read_enum(bytes)?),
                Ok(58) => msg.modified_trip = Some(r.read_message::<transit_realtime::mod_TripDescriptor::ModifiedTripSelector>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TripDescriptor<'a> {
    fn get_size(&self) -> usize {
        0
        + self.trip_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.route_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.direction_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.start_time.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.start_date.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.schedule_relationship.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.modified_trip.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.trip_id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.route_id { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.direction_id { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.start_time { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.start_date { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.schedule_relationship { w.write_with_tag(32, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.modified_trip { w.write_with_tag(58, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TripDescriptor {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ModifiedTripSelector<'a> {
    pub modifications_id: Option<Cow<'a, str>>,
    pub affected_trip_id: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ModifiedTripSelector<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.modifications_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.affected_trip_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModifiedTripSelector<'a> {
    fn get_size(&self) -> usize {
        0
        + self.modifications_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.affected_trip_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.modifications_id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.affected_trip_id { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScheduleRelationship {
    SCHEDULED = 0,
    ADDED = 1,
    UNSCHEDULED = 2,
    CANCELED = 3,
    REPLACEMENT = 5,
    DUPLICATED = 6,
    DELETED = 7,
}

impl Default for ScheduleRelationship {
    fn default() -> Self {
        ScheduleRelationship::SCHEDULED
    }
}

impl From<i32> for ScheduleRelationship {
    fn from(i: i32) -> Self {
        match i {
            0 => ScheduleRelationship::SCHEDULED,
            1 => ScheduleRelationship::ADDED,
            2 => ScheduleRelationship::UNSCHEDULED,
            3 => ScheduleRelationship::CANCELED,
            5 => ScheduleRelationship::REPLACEMENT,
            6 => ScheduleRelationship::DUPLICATED,
            7 => ScheduleRelationship::DELETED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ScheduleRelationship {
    fn from(s: &'a str) -> Self {
        match s {
            "SCHEDULED" => ScheduleRelationship::SCHEDULED,
            "ADDED" => ScheduleRelationship::ADDED,
            "UNSCHEDULED" => ScheduleRelationship::UNSCHEDULED,
            "CANCELED" => ScheduleRelationship::CANCELED,
            "REPLACEMENT" => ScheduleRelationship::REPLACEMENT,
            "DUPLICATED" => ScheduleRelationship::DUPLICATED,
            "DELETED" => ScheduleRelationship::DELETED,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct VehicleDescriptor<'a> {
    pub id: Option<Cow<'a, str>>,
    pub label: Option<Cow<'a, str>>,
    pub license_plate: Option<Cow<'a, str>>,
    pub wheelchair_accessible: transit_realtime::mod_VehicleDescriptor::WheelchairAccessible,
}

impl<'a> MessageRead<'a> for VehicleDescriptor<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.label = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.license_plate = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.wheelchair_accessible = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for VehicleDescriptor<'a> {
    fn get_size(&self) -> usize {
        0
        + self.id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.label.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.license_plate.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.wheelchair_accessible == transit_realtime::mod_VehicleDescriptor::WheelchairAccessible::NO_VALUE { 0 } else { 1 + sizeof_varint(*(&self.wheelchair_accessible) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.label { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.license_plate { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if self.wheelchair_accessible != transit_realtime::mod_VehicleDescriptor::WheelchairAccessible::NO_VALUE { w.write_with_tag(32, |w| w.write_enum(*&self.wheelchair_accessible as i32))?; }
        Ok(())
    }
}

pub mod mod_VehicleDescriptor {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WheelchairAccessible {
    NO_VALUE = 0,
    UNKNOWN = 1,
    WHEELCHAIR_ACCESSIBLE = 2,
    WHEELCHAIR_INACCESSIBLE = 3,
}

impl Default for WheelchairAccessible {
    fn default() -> Self {
        WheelchairAccessible::NO_VALUE
    }
}

impl From<i32> for WheelchairAccessible {
    fn from(i: i32) -> Self {
        match i {
            0 => WheelchairAccessible::NO_VALUE,
            1 => WheelchairAccessible::UNKNOWN,
            2 => WheelchairAccessible::WHEELCHAIR_ACCESSIBLE,
            3 => WheelchairAccessible::WHEELCHAIR_INACCESSIBLE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for WheelchairAccessible {
    fn from(s: &'a str) -> Self {
        match s {
            "NO_VALUE" => WheelchairAccessible::NO_VALUE,
            "UNKNOWN" => WheelchairAccessible::UNKNOWN,
            "WHEELCHAIR_ACCESSIBLE" => WheelchairAccessible::WHEELCHAIR_ACCESSIBLE,
            "WHEELCHAIR_INACCESSIBLE" => WheelchairAccessible::WHEELCHAIR_INACCESSIBLE,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EntitySelector<'a> {
    pub agency_id: Option<Cow<'a, str>>,
    pub route_id: Option<Cow<'a, str>>,
    pub route_type: Option<i32>,
    pub trip: Option<transit_realtime::TripDescriptor<'a>>,
    pub stop_id: Option<Cow<'a, str>>,
    pub direction_id: Option<u32>,
}

impl<'a> MessageRead<'a> for EntitySelector<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.agency_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.route_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.route_type = Some(r.read_int32(bytes)?),
                Ok(34) => msg.trip = Some(r.read_message::<transit_realtime::TripDescriptor>(bytes)?),
                Ok(42) => msg.stop_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.direction_id = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for EntitySelector<'a> {
    fn get_size(&self) -> usize {
        0
        + self.agency_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.route_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.route_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.trip.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stop_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.direction_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.agency_id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.route_id { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.route_type { w.write_with_tag(24, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.trip { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stop_id { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.direction_id { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TranslatedString<'a> {
    pub translation: Vec<transit_realtime::mod_TranslatedString::Translation<'a>>,
}

impl<'a> MessageRead<'a> for TranslatedString<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.translation.push(r.read_message::<transit_realtime::mod_TranslatedString::Translation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TranslatedString<'a> {
    fn get_size(&self) -> usize {
        0
        + self.translation.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.translation { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TranslatedString {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Translation<'a> {
    pub text: Cow<'a, str>,
    pub language: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Translation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.language = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Translation<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.text).len())
        + self.language.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.text))?;
        if let Some(ref s) = self.language { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TranslatedImage<'a> {
    pub localized_image: Vec<transit_realtime::mod_TranslatedImage::LocalizedImage<'a>>,
}

impl<'a> MessageRead<'a> for TranslatedImage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.localized_image.push(r.read_message::<transit_realtime::mod_TranslatedImage::LocalizedImage>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TranslatedImage<'a> {
    fn get_size(&self) -> usize {
        0
        + self.localized_image.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.localized_image { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TranslatedImage {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LocalizedImage<'a> {
    pub url: Cow<'a, str>,
    pub media_type: Cow<'a, str>,
    pub language: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for LocalizedImage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.media_type = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.language = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LocalizedImage<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.url).len())
        + 1 + sizeof_len((&self.media_type).len())
        + self.language.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.url))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.media_type))?;
        if let Some(ref s) = self.language { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Shape<'a> {
    pub shape_id: Option<Cow<'a, str>>,
    pub encoded_polyline: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Shape<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.shape_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.encoded_polyline = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Shape<'a> {
    fn get_size(&self) -> usize {
        0
        + self.shape_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.encoded_polyline.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.shape_id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.encoded_polyline { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Stop<'a> {
    pub stop_id: Option<Cow<'a, str>>,
    pub stop_code: Option<transit_realtime::TranslatedString<'a>>,
    pub stop_name: Option<transit_realtime::TranslatedString<'a>>,
    pub tts_stop_name: Option<transit_realtime::TranslatedString<'a>>,
    pub stop_desc: Option<transit_realtime::TranslatedString<'a>>,
    pub stop_lat: Option<f32>,
    pub stop_lon: Option<f32>,
    pub zone_id: Option<Cow<'a, str>>,
    pub stop_url: Option<transit_realtime::TranslatedString<'a>>,
    pub parent_station: Option<Cow<'a, str>>,
    pub stop_timezone: Option<Cow<'a, str>>,
    pub wheelchair_boarding: transit_realtime::mod_Stop::WheelchairBoarding,
    pub level_id: Option<Cow<'a, str>>,
    pub platform_code: Option<transit_realtime::TranslatedString<'a>>,
}

impl<'a> MessageRead<'a> for Stop<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.stop_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.stop_code = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(26) => msg.stop_name = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(34) => msg.tts_stop_name = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(42) => msg.stop_desc = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(53) => msg.stop_lat = Some(r.read_float(bytes)?),
                Ok(61) => msg.stop_lon = Some(r.read_float(bytes)?),
                Ok(66) => msg.zone_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.stop_url = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(90) => msg.parent_station = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.stop_timezone = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(104) => msg.wheelchair_boarding = r.read_enum(bytes)?,
                Ok(114) => msg.level_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(122) => msg.platform_code = Some(r.read_message::<transit_realtime::TranslatedString>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Stop<'a> {
    fn get_size(&self) -> usize {
        0
        + self.stop_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.stop_code.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stop_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.tts_stop_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stop_desc.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.stop_lat.as_ref().map_or(0, |_| 1 + 4)
        + self.stop_lon.as_ref().map_or(0, |_| 1 + 4)
        + self.zone_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.stop_url.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.parent_station.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.stop_timezone.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + if self.wheelchair_boarding == transit_realtime::mod_Stop::WheelchairBoarding::UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.wheelchair_boarding) as u64) }
        + self.level_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.platform_code.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.stop_id { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.stop_code { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stop_name { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.tts_stop_name { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stop_desc { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.stop_lat { w.write_with_tag(53, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.stop_lon { w.write_with_tag(61, |w| w.write_float(*s))?; }
        if let Some(ref s) = self.zone_id { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.stop_url { w.write_with_tag(74, |w| w.write_message(s))?; }
        if let Some(ref s) = self.parent_station { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.stop_timezone { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if self.wheelchair_boarding != transit_realtime::mod_Stop::WheelchairBoarding::UNKNOWN { w.write_with_tag(104, |w| w.write_enum(*&self.wheelchair_boarding as i32))?; }
        if let Some(ref s) = self.level_id { w.write_with_tag(114, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.platform_code { w.write_with_tag(122, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Stop {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WheelchairBoarding {
    UNKNOWN = 0,
    AVAILABLE = 1,
    NOT_AVAILABLE = 2,
}

impl Default for WheelchairBoarding {
    fn default() -> Self {
        WheelchairBoarding::UNKNOWN
    }
}

impl From<i32> for WheelchairBoarding {
    fn from(i: i32) -> Self {
        match i {
            0 => WheelchairBoarding::UNKNOWN,
            1 => WheelchairBoarding::AVAILABLE,
            2 => WheelchairBoarding::NOT_AVAILABLE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for WheelchairBoarding {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => WheelchairBoarding::UNKNOWN,
            "AVAILABLE" => WheelchairBoarding::AVAILABLE,
            "NOT_AVAILABLE" => WheelchairBoarding::NOT_AVAILABLE,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TripModifications<'a> {
    pub selected_trips: Vec<transit_realtime::mod_TripModifications::SelectedTrips<'a>>,
    pub start_times: Vec<Cow<'a, str>>,
    pub service_dates: Vec<Cow<'a, str>>,
    pub modifications: Vec<transit_realtime::mod_TripModifications::Modification<'a>>,
}

impl<'a> MessageRead<'a> for TripModifications<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.selected_trips.push(r.read_message::<transit_realtime::mod_TripModifications::SelectedTrips>(bytes)?),
                Ok(18) => msg.start_times.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.service_dates.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.modifications.push(r.read_message::<transit_realtime::mod_TripModifications::Modification>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TripModifications<'a> {
    fn get_size(&self) -> usize {
        0
        + self.selected_trips.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.start_times.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.service_dates.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.modifications.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.selected_trips { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.start_times { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.service_dates { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        for s in &self.modifications { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TripModifications {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Modification<'a> {
    pub start_stop_selector: Option<transit_realtime::StopSelector<'a>>,
    pub end_stop_selector: Option<transit_realtime::StopSelector<'a>>,
    pub propagated_modification_delay: i32,
    pub replacement_stops: Vec<transit_realtime::ReplacementStop<'a>>,
    pub service_alert_id: Option<Cow<'a, str>>,
    pub last_modified_time: Option<u64>,
}

impl<'a> MessageRead<'a> for Modification<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.start_stop_selector = Some(r.read_message::<transit_realtime::StopSelector>(bytes)?),
                Ok(18) => msg.end_stop_selector = Some(r.read_message::<transit_realtime::StopSelector>(bytes)?),
                Ok(24) => msg.propagated_modification_delay = r.read_int32(bytes)?,
                Ok(34) => msg.replacement_stops.push(r.read_message::<transit_realtime::ReplacementStop>(bytes)?),
                Ok(42) => msg.service_alert_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.last_modified_time = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Modification<'a> {
    fn get_size(&self) -> usize {
        0
        + self.start_stop_selector.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.end_stop_selector.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.propagated_modification_delay == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.propagated_modification_delay) as u64) }
        + self.replacement_stops.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.service_alert_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.last_modified_time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.start_stop_selector { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.end_stop_selector { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.propagated_modification_delay != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.propagated_modification_delay))?; }
        for s in &self.replacement_stops { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.service_alert_id { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.last_modified_time { w.write_with_tag(48, |w| w.write_uint64(*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SelectedTrips<'a> {
    pub trip_ids: Vec<Cow<'a, str>>,
    pub shape_id: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for SelectedTrips<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.trip_ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.shape_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SelectedTrips<'a> {
    fn get_size(&self) -> usize {
        0
        + self.trip_ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.shape_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.trip_ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.shape_id { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StopSelector<'a> {
    pub stop_sequence: Option<u32>,
    pub stop_id: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for StopSelector<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.stop_sequence = Some(r.read_uint32(bytes)?),
                Ok(18) => msg.stop_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StopSelector<'a> {
    fn get_size(&self) -> usize {
        0
        + self.stop_sequence.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stop_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.stop_sequence { w.write_with_tag(8, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.stop_id { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplacementStop<'a> {
    pub travel_time_to_stop: Option<i32>,
    pub stop_id: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ReplacementStop<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.travel_time_to_stop = Some(r.read_int32(bytes)?),
                Ok(18) => msg.stop_id = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ReplacementStop<'a> {
    fn get_size(&self) -> usize {
        0
        + self.travel_time_to_stop.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stop_id.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.travel_time_to_stop { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.stop_id { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

