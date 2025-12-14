use core::fmt;

pub type Channel = u8; // 0..15
pub type Note = u8; // 0..127
pub type Velocity = u8; // 0..127
pub type VoiceId = u8; // 0..N-1 (N <= 256)

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StolenVoice {
    pub voice_id: VoiceId,
    pub channel: Channel,
    pub note: Note,
    pub velocity: Velocity,
}

impl fmt::Debug for StolenVoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StolenVoice")
            .field("voice_id", &self.voice_id)
            .field("channel", &self.channel)
            .field("note", &self.note)
            .field("velocity", &self.velocity)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteOnAlloc {
    pub voice_id: VoiceId,
    pub stolen: Option<StolenVoice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Voice {
    channel: Channel,
    note: Note,
    velocity: Velocity,
    age: u64, // monotónico (para “oldest stealing”)
}

#[derive(Debug, Clone, Copy)]
pub struct VoiceAllocator<const N: usize> {
    voices: [Option<Voice>; N],
    free: [VoiceId; N],
    free_len: usize,
    age: u64,
}

impl<const N: usize> VoiceAllocator<N> {
    pub fn new() -> Self {
        debug_assert!(N > 0, "N debe ser > 0");
        debug_assert!(N <= 256, "N debe ser <= 256 (VoiceId es u8)");

        let mut free = [0u8; N];
        for (i, slot) in free.iter_mut().enumerate() {
            *slot = (N - 1 - i) as u8;
        }

        Self {
            voices: [None; N],
            free,
            free_len: N,
            age: 0,
        }
    }

    #[inline]
    pub fn note_on(&mut self, channel: Channel, note: Note, velocity: Velocity) -> NoteOnAlloc {
        debug_assert!(channel < 16);
        debug_assert!(note < 128);

        let (voice_id, stolen) = match self.pop_free() {
            Some(id) => (id, None),
            None => {
                let (id, st) = self.steal_oldest();
                (id, Some(st))
            }
        };

        self.age = self.age.wrapping_add(1);
        self.voices[voice_id as usize] = Some(Voice {
            channel,
            note,
            velocity,
            age: self.age,
        });

        NoteOnAlloc { voice_id, stolen }
    }

    #[inline]
    pub fn note_off(&mut self, channel: Channel, note: Note) -> Option<VoiceId> {
        debug_assert!(channel < 16);
        debug_assert!(note < 128);

        let mut best: Option<(VoiceId, u64)> = None;

        for (i, slot) in self.voices.iter().enumerate() {
            let Some(v) = slot else { continue };
            if v.channel != channel || v.note != note {
                continue;
            }

            let id = i as VoiceId;
            best = match best {
                None => Some((id, v.age)),
                Some((best_id, best_age)) => {
                    if v.age > best_age {
                        Some((id, v.age))
                    } else {
                        Some((best_id, best_age))
                    }
                }
            };
        }

        let (id, _) = best?;
        self.voices[id as usize] = None;
        self.push_free(id);
        Some(id)
    }

    #[inline]
    fn pop_free(&mut self) -> Option<VoiceId> {
        if self.free_len == 0 {
            None
        } else {
            self.free_len -= 1;
            Some(self.free[self.free_len])
        }
    }

    #[inline]
    fn push_free(&mut self, id: VoiceId) {
        debug_assert!(self.free_len < N);
        self.free[self.free_len] = id;
        self.free_len += 1;
    }

    fn steal_oldest(&mut self) -> (VoiceId, StolenVoice) {
        let mut best: Option<(VoiceId, u64, Voice)> = None;

        for (i, slot) in self.voices.iter().enumerate() {
            let Some(v) = slot else { continue };
            let id = i as VoiceId;

            best = match best {
                None => Some((id, v.age, *v)),
                Some((best_id, best_age, best_v)) => {
                    if v.age < best_age {
                        Some((id, v.age, *v))
                    } else {
                        Some((best_id, best_age, best_v))
                    }
                }
            };
        }

        let (id, _age, v) =
            best.expect("Invariante: sin libres => debe existir al menos una voz activa.");

        self.voices[id as usize] = None;

        (
            id,
            StolenVoice {
                voice_id: id,
                channel: v.channel,
                note: v.note,
                velocity: v.velocity,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_sequential_voice_ids_initially() {
        let mut a: VoiceAllocator<4> = VoiceAllocator::new();

        let v0 = a.note_on(0, 60, 100).voice_id;
        let v1 = a.note_on(0, 61, 100).voice_id;
        let v2 = a.note_on(0, 62, 100).voice_id;
        let v3 = a.note_on(0, 63, 100).voice_id;

        assert_eq!([v0, v1, v2, v3], [0, 1, 2, 3]);
    }

    #[test]
    fn note_off_releases_and_reuses_voice_id() {
        let mut a: VoiceAllocator<2> = VoiceAllocator::new();

        let v0 = a.note_on(0, 60, 100).voice_id;
        let v1 = a.note_on(0, 61, 100).voice_id;
        assert_eq!([v0, v1], [0, 1]);

        let off = a.note_off(0, 60);
        assert_eq!(off, Some(0));

        // al liberar 0, el siguiente note_on puede reutilizarlo
        let v2 = a.note_on(0, 62, 100).voice_id;
        assert_eq!(v2, 0);
    }

    #[test]
    fn steals_oldest_when_full() {
        let mut a: VoiceAllocator<2> = VoiceAllocator::new();

        let _ = a.note_on(0, 60, 100); // usa voz 0 (oldest)
        let _ = a.note_on(0, 61, 100); // usa voz 1

        let alloc = a.note_on(0, 62, 100); // debe robar la voz 0 (la más antigua)
        assert_eq!(alloc.voice_id, 0);
        let stolen = alloc.stolen.expect("debe haber robado una voz");
        assert_eq!(stolen.voice_id, 0);
        assert_eq!(stolen.note, 60);
    }

    #[test]
    fn note_off_releases_most_recent_when_note_is_stacked() {
        let mut a: VoiceAllocator<4> = VoiceAllocator::new();

        let v0 = a.note_on(0, 60, 10).voice_id;
        let v1 = a.note_on(0, 60, 20).voice_id;
        let v2 = a.note_on(0, 60, 30).voice_id;

        // Debe liberar la más reciente: v2
        let off1 = a.note_off(0, 60);
        assert_eq!(off1, Some(v2));

        let off2 = a.note_off(0, 60);
        assert_eq!(off2, Some(v1));

        let off3 = a.note_off(0, 60);
        assert_eq!(off3, Some(v0));

        let off4 = a.note_off(0, 60);
        assert_eq!(off4, None);
    }
}
