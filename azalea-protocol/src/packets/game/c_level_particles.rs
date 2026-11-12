use azalea_buf::AzBuf;
use azalea_core::position::Vec3;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundLevelParticles {
    pub particle: Particle,
    pub override_limiter: bool,
    pub always_show: bool,
    pub pos: Vec3,
    pub x_dist: f32,
    pub y_dist: f32,
    pub z_dist: f32,
    pub x_max_speed: f32,
    pub y_max_speed: f32,
    pub z_max_speed: f32,
    #[var]
    pub count: u32,
    pub randomization_type: RandomizationType,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, PartialEq)]
pub enum RandomizationType {
    #[default]
    Default,
    Alternative,
    AlternativeWithSpeed,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use super::*;

    #[test]
    fn test_read_write_bubble_level_particles() {
        // A bubble particle as a 26.3 server sends it (packet id removed): the
        // particle comes first, the speed is set per axis, the count is a VarInt
        // and the randomization type follows it.
        let contents = [
            0x03, // particle: bubble
            0x00, // override_limiter
            0x00, // always_show
            0xc0, 0x2b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // x: -13.5
            0x40, 0x4d, 0x18, 0x9b, 0x0e, 0xe4, 0x9f, 0x51, // y
            0xc0, 0x5c, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x00, // z: -114.5
            0x3e, 0xcc, 0xcc, 0xcd, // x_dist: 0.4
            0x3e, 0xcc, 0xcc, 0xcd, // y_dist: 0.4
            0x3e, 0xcc, 0xcc, 0xcd, // z_dist: 0.4
            0x3f, 0x00, 0x00, 0x00, // x_max_speed: 0.5
            0x3f, 0x00, 0x00, 0x00, // y_max_speed: 0.5
            0x3f, 0x00, 0x00, 0x00, // z_max_speed: 0.5
            0x02, // count: 2
            0x00, // randomization_type: default
        ];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundLevelParticles::azalea_read(&mut buf).unwrap();
        assert_eq!(buf.position(), contents.len() as u64);

        assert_eq!(packet.particle, Particle::Bubble);
        assert!(!packet.override_limiter);
        assert!(!packet.always_show);
        assert_eq!(packet.pos.x, -13.5);
        assert_eq!(packet.pos.z, -114.5);
        assert_eq!(packet.x_dist, 0.4);
        assert_eq!(packet.y_max_speed, 0.5);
        assert_eq!(packet.count, 2);
        assert_eq!(packet.randomization_type, RandomizationType::Default);

        let mut written = Vec::new();
        packet.azalea_write(&mut written).unwrap();
        assert_eq!(written, contents);
    }
}
