//! The codetrace session event vocabulary. See SPEC.md — that document is
//! normative, this is an implementation of it.

use serde::{Deserialize, Serialize};

/// Milliseconds since session start. `t = 0` is the moment audio capture
/// begins, or `session.start` when there is no audio. Holding this invariant
/// even when a stream is absent is what makes speech and code comparable.
pub type Millis = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub t: Millis,
    #[serde(flatten)]
    pub kind: EventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventKind {
    #[serde(rename = "session.start")]
    SessionStart { problem: String, lang: String },

    #[serde(rename = "session.end")]
    SessionEnd { reason: EndReason },

    /// Segment-level, not word-level: word timestamps are noise here.
    Speech { dur: Millis, text: String },

    /// Code is never inlined — `sha` points into the session's `repo/`.
    Edit { sha: String, files: u32, added: u32, removed: u32 },

    #[serde(rename = "compile.ok")]
    CompileOk { sha: String, exit: i32, ms: Millis },

    #[serde(rename = "compile.error")]
    CompileError { sha: String, exit: i32, diags: Vec<Diagnostic> },

    #[serde(rename = "test.run")]
    TestRun {
        sha: String,
        passed: u32,
        failed: u32,
        total: u32,
        #[serde(default)]
        failing: Vec<FailingCase>,
    },

    Run { sha: String, exit: i32, ms: Millis },

    /// Recorded at the *end* of the gap, with its duration. Silence is signal:
    /// an idle gap during debugging is the strongest stuck-indicator we have.
    Idle { dur: Millis },

    Marker {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndReason { User, Timeout, Crash }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub file: String,
    pub line: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col: Option<u32>,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailingCase {
    pub case: String,
    pub expected: String,
    pub actual: String,
}

/// Readers must ignore unknown event types rather than reject them — that is
/// how the format stays extensible across spec versions.
pub fn parse_lenient(jsonl: &str) -> Vec<Event> {
    jsonl.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str::<Event>(l).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips_a_compile_error() {
        let line = r#"{"t":190100,"type":"compile.error","sha":"a3f9c21","exit":1,"diags":[{"file":"Solution.java","line":42,"msg":"bad types"}]}"#;
        let evs = parse_lenient(line);
        assert_eq!(evs.len(), 1);
        assert_eq!(evs[0].t, 190100);
    }

    #[test]
    fn skips_unknown_types_without_failing() {
        let jsonl = "{\"t\":1,\"type\":\"future.thing\",\"x\":9}\n{\"t\":2,\"type\":\"idle\",\"dur\":71000}";
        assert_eq!(parse_lenient(jsonl).len(), 1);
    }
}
