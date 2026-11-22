//! How far a driven client may go beyond a player (#1095). Every MCP tool and `state` field has a grade, and
//! `RUSTNAROK_CLIENT_PLAY` (fair, assist or all, all when unset) is the highest grade a session may use, and the packet
//! trace (`RUSTNAROK_CLIENT_TRACE`) is for all only.

use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Grade {
    /// What a player has: the same input, and what the screen shows.
    Fair,
    /// Nothing a player couldn't see or do, handed over in an easier form, or making up for a mouse driven over a
    /// socket (the round trip, no eyes on a moving sprite).
    Assist,
    /// Beyond a player: for tests and debugging.
    Test,
}

impl Grade {
    pub fn parse(mode: &str) -> Option<Self> {
        match mode {
            "fair" => Some(Self::Fair),
            "assist" => Some(Self::Assist),
            "all" => Some(Self::Test),
            _ => None,
        }
    }

    /// The play mode that allows up to this grade.
    pub fn mode(self) -> &'static str {
        ["fair", "assist", "all"][self as usize]
    }

    /// The tag a tool's description starts with.
    pub fn tag(self) -> &'static str {
        ["[fair]", "[assist]", "[test]"][self as usize]
    }
}

/// Keep full driver control for debug builds, but never default a shipped
/// release executable to the test-only command/packet-access grade.
fn default_mode(debug_build: bool) -> Grade {
    if debug_build { Grade::Test } else { Grade::Fair }
}

/// The session's play mode. Explicit RUSTNAROK_CLIENT_PLAY=all is still
/// available to controlled development sessions; regular release launches
/// default to the same access a player has.
pub fn mode() -> Grade {
    static MODE: OnceLock<Grade> = OnceLock::new();
    *MODE.get_or_init(|| {
        std::env::var("RUSTNAROK_CLIENT_PLAY")
            .ok()
            .and_then(|mode| Grade::parse(&mode))
            .unwrap_or_else(|| default_mode(cfg!(debug_assertions)))
    })
}

/// How the packets the servers send, and those the client sends the map server, are printed (`RUSTNAROK_CLIENT_TRACE`):
/// None for not at all, else whether whole (`full`) or just their first bytes. A trace holds everything the server
/// sends, far more than the screen shows, so it is for play mode all: below that it stays off (#1226).
pub fn trace() -> Option<bool> {
    static TRACE: OnceLock<Option<bool>> = OnceLock::new();
    *TRACE.get_or_init(|| {
        let trace = traced(mode(), std::env::var("RUSTNAROK_CLIENT_TRACE").ok().as_deref());
        if trace.is_none() && std::env::var_os("RUSTNAROK_CLIENT_TRACE").is_some() {
            eprintln!("RUSTNAROK_CLIENT_TRACE is off: a trace shows more than the screen, and the play mode is {}", mode().mode());
        }
        trace
    })
}

/// `trace` for a play mode and the switch's value.
fn traced(mode: Grade, value: Option<&str>) -> Option<bool> {
    value.filter(|_| mode == Grade::Test).map(|value| value == "full")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_defaults_to_player_permissions() {
        assert_eq!(default_mode(false), Grade::Fair);
        assert_eq!(default_mode(true), Grade::Test);
        assert_eq!(Grade::parse("all"), Some(Grade::Test));
        assert_eq!(Grade::parse("fair"), Some(Grade::Fair));
    }

    #[test]
    fn traces_only_in_play_mode_all() {
        assert_eq!(traced(Grade::Test, Some("1")), Some(false));
        assert_eq!(traced(Grade::Test, Some("full")), Some(true));
        assert_eq!(traced(Grade::Test, None), None);
        for mode in [Grade::Fair, Grade::Assist] {
            assert_eq!([traced(mode, Some("1")), traced(mode, Some("full"))], [None, None]);
        }
    }
}
