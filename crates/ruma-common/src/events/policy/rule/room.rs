//! Types for the [`m.policy.rule.room`] event.
//!
//! [`m.policy.rule.room`]: https://spec.matrix.org/v1.2/client-server-api/#mpolicyruleroom

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use super::PolicyRuleEventContent;

/// The content of an `m.policy.rule.room` event.
///
/// This event type is used to apply rules to room entities.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[allow(clippy::exhaustive_structs)]
#[ruma_event(type = "m.policy.rule.room", kind = State, state_key_type = String)]
pub struct PolicyRuleRoomEventContent(pub PolicyRuleEventContent);

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use js_int::int;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::{OriginalPolicyRuleRoomEvent, PolicyRuleRoomEventContent};
    use crate::{
        event_id,
        events::{
            policy::rule::{PolicyRuleEventContent, Recommendation},
            StateUnsigned,
        },
        room_id,
        serde::Raw,
        user_id, MilliSecondsSinceUnixEpoch,
    };

    #[test]
    fn serialization() {
        let room_event = OriginalPolicyRuleRoomEvent {
            event_id: event_id!("$143273582443PhrSn:example.org").to_owned(),
            sender: user_id!("@example:example.org").to_owned(),
            origin_server_ts: MilliSecondsSinceUnixEpoch(1_432_735_824_653_u64.try_into().unwrap()),
            room_id: room_id!("!jEsUZKDJdhlrceRyVU:example.org").to_owned(),
            state_key: "rule:#*:example.org".into(),
            unsigned: StateUnsigned { age: Some(int!(1234)), ..StateUnsigned::default() },
            content: PolicyRuleRoomEventContent(PolicyRuleEventContent {
                entity: "#*:example.org".into(),
                reason: "undesirable content".into(),
                recommendation: Recommendation::Ban,
            }),
        };

        let json = json!({
            "content": {
                "entity": "#*:example.org",
                "reason": "undesirable content",
                "recommendation": "m.ban"
            },
            "event_id": "$143273582443PhrSn:example.org",
            "origin_server_ts": 1_432_735_824_653_u64,
            "room_id": "!jEsUZKDJdhlrceRyVU:example.org",
            "sender": "@example:example.org",
            "state_key": "rule:#*:example.org",
            "type": "m.policy.rule.room",
            "unsigned": {
                "age": 1234
            }
        });

        assert_eq!(to_json_value(room_event).unwrap(), json);
    }

    #[test]
    fn deserialization() {
        let json = json!({
            "content": {
                "entity": "#*:example.org",
                "reason": "undesirable content",
                "recommendation": "m.ban"
            },
            "event_id": "$143273582443PhrSn:example.org",
            "origin_server_ts": 1_432_735_824_653_u64,
            "room_id": "!jEsUZKDJdhlrceRyVU:example.org",
            "sender": "@example:example.org",
            "state_key": "rule:#*:example.org",
            "type": "m.policy.rule.room",
            "unsigned": {
                "age": 1234
            }
        });

        assert!(from_json_value::<Raw<OriginalPolicyRuleRoomEvent>>(json)
            .unwrap()
            .deserialize()
            .is_ok());
    }
}
