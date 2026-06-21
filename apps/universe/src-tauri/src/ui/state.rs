use tench_ui::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniverseMode {
    Chat,
    Novel,
    Interactive,
    Script,
}

impl UniverseMode {
    pub const ALL: [Self; 4] = [Self::Chat, Self::Novel, Self::Interactive, Self::Script];

    pub fn label(self) -> &'static str {
        match self {
            Self::Chat => "Chat",
            Self::Novel => "Novel",
            Self::Interactive => "Interactive",
            Self::Script => "Script",
        }
    }

    pub fn accent(self) -> Color {
        match self {
            Self::Chat => NEUTRAL_100,
            Self::Novel => ACCENT_UNIVERSE,
            Self::Interactive => STATUS_WARNING,
            Self::Script => ACCENT_ENGINE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mood {
    Happy,
    Neutral,
    Surprised,
    Sad,
    Angry,
}

impl Mood {
    pub fn color(self) -> Color {
        match self {
            Self::Happy => STATUS_READY,
            Self::Neutral => ACCENT_ENGINE,
            Self::Surprised => STATUS_WARNING,
            Self::Sad => STATUS_RUNNING,
            Self::Angry => STATUS_ERROR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Character {
    pub id: &'static str,
    pub name: &'static str,
    pub mood: Mood,
    pub score: i32,
    pub memory_count: i32,
    pub greeting: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

#[derive(Clone, Debug)]
pub struct Persona {
    pub name: String,
    pub bio: String,
    pub tone: String,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub sender: String,
    pub text: String,
    pub time: String,
}

#[derive(Clone, Debug)]
pub struct InteractiveBlock {
    pub text: &'static str,
    pub choices: &'static [&'static str],
    pub selected: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniverseHit {
    Mode(UniverseMode),
    Search,
    Character(usize),
    Choice(usize),
    ComposerInput,
    Send,
    NewCharacter,
    TemplatePicker,
    Sessions,
    Settings,
    EditPersona,
    CloseModal,
    Memory(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UniverseInputFocus {
    Composer,
    CharacterSearch,
}

pub struct UniverseState {
    pub mode: UniverseMode,
    pub active_character_idx: usize,
    pub characters: Vec<Character>,
    pub persona: Persona,
    pub messages: Vec<ChatMessage>,
    pub interactive_blocks: Vec<InteractiveBlock>,
    pub input_text: String,
    pub search_query: String,
    pub input_focus: UniverseInputFocus,
    pub toast: String,
    pub show_character_editor: bool,
    pub show_persona_editor: bool,
    pub show_sessions: bool,
    pub show_template_picker: bool,
    pub show_settings: bool,
    pub prompt_preview_open: bool,
    pub selected_memory: Option<usize>,
}

impl Default for UniverseState {
    fn default() -> Self {
        Self {
            mode: UniverseMode::Chat,
            active_character_idx: 0,
            characters: default_characters(),
            persona: Persona {
                name: "User".into(),
                bio: "Default user persona".into(),
                tone: "neutral".into(),
            },
            messages: Vec::new(),
            interactive_blocks: vec![InteractiveBlock {
                text: "You stand before the gate. Its surface ripples with a faint blue light. Ari watches from the shadows, arms crossed.",
                choices: &["Open the gate", "Call Ari over", "Walk away", "Touch the seal"],
                selected: None,
            }],
            input_text: String::new(),
            search_query: String::new(),
            input_focus: UniverseInputFocus::Composer,
            toast: String::new(),
            show_character_editor: false,
            show_persona_editor: false,
            show_sessions: false,
            show_template_picker: false,
            show_settings: false,
            prompt_preview_open: false,
            selected_memory: None,
        }
    }
}

impl UniverseState {
    pub fn active_character(&self) -> Option<&Character> {
        self.characters.get(self.active_character_idx)
    }

    pub fn set_mode(&mut self, mode: UniverseMode) {
        self.mode = mode;
        self.toast = format!("Mode: {}", mode.label());
    }

    pub fn select_character(&mut self, index: usize) {
        if index < self.characters.len() {
            self.active_character_idx = index;
            self.input_focus = UniverseInputFocus::Composer;
            self.toast = format!("Selected {}", self.characters[index].name);
        }
    }

    pub fn send_input(&mut self) {
        let text = self.input_text.trim().to_string();
        if text.is_empty() {
            return;
        }
        self.input_text.clear();
        self.messages.push(ChatMessage {
            sender: "user".into(),
            text,
            time: "now".into(),
        });
        let character_name = self
            .active_character()
            .map(|character| character.name)
            .unwrap_or("Character");
        self.messages.push(ChatMessage {
            sender: character_name.into(),
            text: format!("[{character_name} would respond here. Engine integration pending.]"),
            time: "now".into(),
        });
    }

    pub fn push_input_text(&mut self, text: &str) {
        match self.input_focus {
            UniverseInputFocus::Composer => self.input_text.push_str(text),
            UniverseInputFocus::CharacterSearch => self.search_query.push_str(text),
        }
    }

    pub fn backspace_input(&mut self) {
        match self.input_focus {
            UniverseInputFocus::Composer => {
                self.input_text.pop();
            }
            UniverseInputFocus::CharacterSearch => {
                self.search_query.pop();
            }
        }
    }

    pub fn focus_composer(&mut self) {
        self.input_focus = UniverseInputFocus::Composer;
    }

    pub fn focus_search(&mut self) {
        self.input_focus = UniverseInputFocus::CharacterSearch;
    }

    pub fn choose_interactive(&mut self, index: usize) {
        if let Some(block) = self.interactive_blocks.first_mut() {
            if index < block.choices.len() {
                block.selected = Some(index);
                self.toast = format!("You chose: {}", block.choices[index]);
            }
        }
    }

    pub fn open_character_editor(&mut self) {
        self.close_modals();
        self.show_character_editor = true;
    }

    pub fn open_persona_editor(&mut self) {
        self.close_modals();
        self.show_persona_editor = true;
    }

    pub fn open_sessions(&mut self) {
        self.close_modals();
        self.show_sessions = true;
    }

    pub fn open_template_picker(&mut self) {
        self.close_modals();
        self.show_template_picker = true;
    }

    pub fn select_memory(&mut self, index: usize) {
        self.selected_memory = Some(index);
        self.toast = format!("Memory {index} selected");
    }

    pub fn open_settings(&mut self) {
        self.close_modals();
        self.show_settings = true;
    }

    pub fn close_modals(&mut self) {
        self.show_character_editor = false;
        self.show_persona_editor = false;
        self.show_sessions = false;
        self.show_template_picker = false;
        self.show_settings = false;
    }
}

pub const NEUTRAL_900: Color = Color::rgb8(0x0F, 0x0F, 0x0F);
pub const NEUTRAL_800: Color = Color::rgb8(0x1A, 0x1A, 0x1A);
pub const NEUTRAL_700: Color = Color::rgb8(0x2A, 0x2A, 0x2A);
pub const NEUTRAL_600: Color = Color::rgb8(0x3A, 0x3A, 0x3A);
pub const NEUTRAL_500: Color = Color::rgb8(0x4A, 0x4A, 0x4A);
pub const NEUTRAL_400: Color = Color::rgb8(0x6A, 0x6A, 0x6A);
pub const NEUTRAL_300: Color = Color::rgb8(0x8A, 0x8A, 0x8A);
pub const NEUTRAL_100: Color = Color::rgb8(0xD4, 0xD4, 0xD4);
pub const NEUTRAL_50: Color = Color::rgb8(0xF5, 0xF5, 0xF5);
pub const ACCENT_UNIVERSE: Color = Color::rgb8(0xC0, 0x84, 0xFC);
pub const ACCENT_ENGINE: Color = Color::rgb8(0x94, 0xA3, 0xB8);
pub const STATUS_READY: Color = Color::rgb8(0x22, 0xC5, 0x5E);
pub const STATUS_RUNNING: Color = Color::rgb8(0x3B, 0x82, 0xF6);
pub const STATUS_WARNING: Color = Color::rgb8(0xF5, 0x9E, 0x0B);
pub const STATUS_ERROR: Color = Color::rgb8(0xEF, 0x44, 0x44);

fn default_characters() -> Vec<Character> {
    vec![
        Character {
            id: "ari",
            name: "Ari",
            mood: Mood::Neutral,
            score: 72,
            memory_count: 3,
            greeting: "Welcome back. Something happened while you were gone.",
            description: "A mysterious gatekeeper who guards the ancient archive.",
            tags: &["gatekeeper", "archive"],
        },
        Character {
            id: "mira",
            name: "Mira",
            mood: Mood::Happy,
            score: 58,
            memory_count: 1,
            greeting: "Hey there! Long time no see!",
            description: "A cheerful merchant from the market district.",
            tags: &["merchant", "market"],
        },
        Character {
            id: "detective",
            name: "Detective",
            mood: Mood::Surprised,
            score: 64,
            memory_count: 0,
            greeting: "Another case? Let me get my coat.",
            description: "A sharp-witted private investigator.",
            tags: &["mystery", "detective"],
        },
        Character {
            id: "villain",
            name: "Villain",
            mood: Mood::Angry,
            score: 42,
            memory_count: 2,
            greeting: "So... you've finally arrived.",
            description: "A compelling antagonist with depth.",
            tags: &["villain", "antagonist"],
        },
        Character {
            id: "wanderer",
            name: "Wanderer",
            mood: Mood::Sad,
            score: 37,
            memory_count: 1,
            greeting: "The road has been quiet lately.",
            description: "A displaced traveler looking for old routes.",
            tags: &["traveler", "memory"],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_input_appends_user_and_character_messages() {
        let mut state = UniverseState::default();
        state.push_input_text("What happened at the archive?");

        state.send_input();

        assert!(state.input_text.is_empty());
        assert_eq!(state.messages.len(), 2);
        assert_eq!(state.messages[0].sender, "user");
        assert_eq!(state.messages[1].sender, "Ari");
    }

    #[test]
    fn selecting_character_changes_response_speaker() {
        let mut state = UniverseState::default();
        state.select_character(1);
        state.push_input_text("Hello");

        state.send_input();

        assert_eq!(state.messages[1].sender, "Mira");
        assert!(state.toast.contains("Mira"));
    }

    #[test]
    fn modal_openers_close_previous_modal() {
        let mut state = UniverseState::default();

        state.open_character_editor();
        state.open_settings();

        assert!(!state.show_character_editor);
        assert!(state.show_settings);
    }
}
