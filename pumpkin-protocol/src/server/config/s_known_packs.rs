use pumpkin_macros::packet;

use crate::{bytebuf::ByteBuffer, Identifier, ServerPacket, VarInt};

#[derive(serde::Deserialize)]
#[packet(0x07)]
pub struct SKnownPacks {
    pub known_pack_count: VarInt,
    // known_packs: &'a [KnownPack]
}
