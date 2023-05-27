//! # Dialog
//! Structs for the `dlg` file format

use serde::{Deserialize, Serialize};

use super::*;

/// Represents a dialog in the `dlg` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Dlg {
    /// Field representing the delay for the entry in milliseconds
    #[serde(skip_serializing_if = "Option::is_none", rename = "DelayEntry")]
    pub delay_entry: Option<NwValue<u32>>,
    /// Field representing the delay for the reply in milliseconds
    #[serde(skip_serializing_if = "Option::is_none", rename = "DelayReply")]
    pub delay_reply: Option<NwValue<u32>>,
    /// Field representing the script to execute when the conversation is aborted
    #[serde(skip_serializing_if = "Option::is_none", rename = "EndConverAbort")]
    pub end_conver_abort: Option<NwValue<String>>,
    /// Field representing the script to execute when the conversation ends
    #[serde(skip_serializing_if = "Option::is_none", rename = "EndConversation")]
    pub end_conversation: Option<NwValue<String>>,
    /// Field representing the list of entries in the conversation
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntryList")]
    pub entry_list: Option<NwValue<Vec<Entry>>>,
    /// Field representing the number of words in the conversation
    #[serde(skip_serializing_if = "Option::is_none", rename = "NumWords")]
    pub num_words: Option<NwValue<u32>>,
    /// Field indicating whether zooming in is prevented during the conversation
    #[serde(skip_serializing_if = "Option::is_none", rename = "PreventZoomIn")]
    pub prevent_zoom_in: Option<NwValue<u8>>,
    /// Field representing the list of reply entries in the conversation
    #[serde(skip_serializing_if = "Option::is_none", rename = "ReplyList")]
    pub reply_list: Option<NwValue<Vec<ReplyEntry>>>,
    /// Field representing the list of starting entries in the conversation
    #[serde(skip_serializing_if = "Option::is_none", rename = "StartingList")]
    pub starting_list: Option<NwValue<Vec<StartingList>>>,
}

/// Represents a entry in the `dlg` file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    /// Field representing the struct ID of the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    /// Field representing the animation of the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "Animation")]
    pub animation: Option<NwValue<u32>>,
    /// Field representing the animation loop of the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimLoop")]
    pub anim_loop: Option<NwValue<u8>>,
    /// Field representing the comment of the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    /// Field representing the delay of the entry in milliseconds
    #[serde(skip_serializing_if = "Option::is_none", rename = "Delay")]
    pub delay: Option<NwValue<u32>>,
    /// Field representing the quest associated with the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "Quest")]
    pub quest: Option<NwValue<String>>,
    /// Field representing the quest entry of the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "QuestEntry")]
    pub quest_entry: Option<NwValue<u32>>,
    /// Field representing the list of replies associated with the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "RepliesList")]
    pub replies_list: Option<NwValue<Vec<Reply>>>,
    /// Field representing the list of entries associated with the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntriesList")]
    pub entries_list: Option<NwValue<Vec<EntryReply>>>,
    /// Field representing the script to execute for the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "Script")]
    pub script: Option<NwValue<String>>,
    /// Field representing the sound associated with the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sound")]
    pub sound: Option<NwValue<String>>,
    /// Field representing the speaker of the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "Speaker")]
    pub speaker: Option<NwValue<String>>,
    /// Field representing the text of the entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "Text")]
    pub text: Option<NwValue<super::LocalizedText>>,
}

/// Represents a response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyEntry {
    /// Field representing the struct ID
    #[serde(skip_serializing_if = "Option::is_none", rename = "__struct_id")]
    pub struct_id: Option<u32>,
    /// Field representing the animation
    #[serde(skip_serializing_if = "Option::is_none", rename = "Animation")]
    pub animation: Option<NwValue<u32>>,
    /// Field representing the animation loop
    #[serde(skip_serializing_if = "Option::is_none", rename = "AnimLoop")]
    pub anim_loop: Option<NwValue<u8>>,
    /// Field representing the comment
    #[serde(skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: Option<NwValue<String>>,
    /// Field representing the delay
    #[serde(skip_serializing_if = "Option::is_none", rename = "Delay")]
    pub delay: Option<NwValue<u32>>,
    /// Field representing the list of entry replies
    #[serde(skip_serializing_if = "Option::is_none", rename = "EntriesList")]
    pub entries_list: Option<NwValue<Vec<EntryReply>>>,
    /// Field representing the quest
    #[serde(skip_serializing_if = "Option::is_none", rename = "Quest")]
    pub quest: Option<NwValue<String>>,
    /// Field representing the quest entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "QuestEntry")]
    pub quest_entry: Option<NwValue<u32>>,
    /// Field representing the list of replies
    #[serde(skip_serializing_if = "Option::is_none", rename = "RepliesList")]
    pub replies_list: Option<NwValue<Vec<Reply>>>,
    /// Field representing the script
    #[serde(skip_serializing_if = "Option::is_none", rename = "Script")]
    pub script: Option<NwValue<String>>,
    /// Field representing the sound
    #[serde(skip_serializing_if = "Option::is_none", rename = "Sound")]
    pub sound: Option<NwValue<String>>,
    /// Field representing the speaker
    #[serde(skip_serializing_if = "Option::is_none", rename = "Speaker")]
    pub speaker: Option<NwValue<String>>,
    /// Field representing the text
    #[serde(skip_serializing_if = "Option::is_none", rename = "Text")]
    pub text: Option<NwValue<super::LocalizedText>>,
    /// Field representing the link comment
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}

/// An entry reply.
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryReply {
    /// Field representing the struct ID
    #[serde(rename = "__struct_id")]
    pub struct_id: u8,
    /// Field representing the active state
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<String>>,
    /// Field representing the index
    #[serde(skip_serializing_if = "Option::is_none", rename = "Index")]
    pub index: Option<NwValue<u32>>,
    /// Field indicating if it is a child entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsChild")]
    pub is_child: Option<NwValue<u8>>,
    /// Field representing the link comment
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}

/// A response.
#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    /// Field representing the struct ID
    #[serde(rename = "__struct_id")]
    pub struct_id: u8,
    /// Field representing the active state
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<String>>,
    /// Field representing the index
    #[serde(skip_serializing_if = "Option::is_none", rename = "Index")]
    pub index: Option<NwValue<u32>>,
    /// Field indicating if it is a child entry
    #[serde(skip_serializing_if = "Option::is_none", rename = "IsChild")]
    pub is_child: Option<NwValue<u8>>,
    /// Field representing the link comment
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}

/// The starting list in a conversation.
#[derive(Debug, Serialize, Deserialize)]
pub struct StartingList {
    /// Field representing the struct ID
    #[serde(rename = "__struct_id")]
    pub struct_id: u32,
    /// Field representing the active state
    #[serde(skip_serializing_if = "Option::is_none", rename = "Active")]
    pub active: Option<NwValue<String>>,
    /// Field representing the index
    #[serde(skip_serializing_if = "Option::is_none", rename = "Index")]
    pub index: Option<NwValue<u32>>,
    /// Field representing the link comment
    #[serde(skip_serializing_if = "Option::is_none", rename = "LinkComment")]
    pub link_comment: Option<NwValue<String>>,
}
