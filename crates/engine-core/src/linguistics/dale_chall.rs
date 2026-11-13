//! Deterministic, zero-cloud linguistic simplification and readability analysis.
//! Based on Dale-Chall 3,000 vocabulary and Flesch-Kincaid Grade Level formulas.

#![allow(dead_code)]

/// Foundational sample of the Dale-Chall familiar vocabulary list.
/// Always kept alphabetically sorted for O(log N) binary search.
pub static DALE_CHALL_CORE: &[&str] = &[
    "a", "able", "about", "above", "accept", "accident", "across", "act", "action", "add",
    "address", "admit", "adult", "advice", "affect", "after", "again", "against", "age", "ago",
    "agree", "ahead", "aid", "aim", "air", "alarm", "all", "allow", "almost", "alone",
    "along", "already", "also", "always", "among", "amount", "and", "anger", "angle", "angry",
    "animal", "another", "answer", "any", "apart", "appear", "apple", "apply", "area", "argue",
    "arm", "army", "around", "arrive", "art", "as", "ask", "at", "attack", "attempt",
    "attend", "attention", "author", "auto", "avoid", "away", "baby", "back", "bad", "bag",
    "ball", "band", "bank", "bar", "base", "basic", "bath", "be", "beach", "bear",
    "beat", "beauty", "because", "become", "bed", "before", "begin", "behavior", "behind", "believe",
    "belong", "below", "beside", "best", "better", "between", "beyond", "big", "bill", "bird",
    "birth", "bit", "black", "blade", "block", "blood", "blow", "blue", "board", "boat",
    "body", "bone", "book", "boot", "border", "born", "both", "bother", "bottle", "bottom",
    "branch", "brave", "bread", "break", "breakfast", "breath", "bridge", "bright", "bring", "brother",
    "brown", "brush", "build", "burn", "bus", "business", "busy", "but", "buy", "by",
    "cake", "call", "calm", "camera", "camp", "can", "cancel", "candy", "cannot", "car",
    "card", "care", "careful", "carry", "case", "cat", "catch", "cause", "cell", "center",
    "chair", "chance", "change", "charge", "chart", "check", "child", "children", "choose", "church",
    "circle", "city", "claim", "class", "clean", "clear", "climb", "clock", "close", "cloth",
    "clothes", "cloud", "coach", "coast", "coat", "cold", "collect", "college", "color", "come",
    "common", "company", "compare", "complete", "computer", "condition", "connect", "consider", "contain", "continue",
    "control", "cook", "cool", "copy", "corner", "correct", "cost", "count", "country", "course",
    "court", "cover", "crazy", "create", "credit", "crime", "cross", "crowd", "cry", "cup",
    "cure", "cut", "daily", "damage", "dance", "danger", "dark", "data", "date", "daughter",
    "day", "dead", "deal", "dear", "death", "decide", "decision", "deep", "degree", "delay",
    "deliver", "demand", "dentist", "deny", "depend", "describe", "design", "desk", "destroy", "detail",
    "develop", "device", "die", "diet", "difference", "different", "difficult", "dinner", "direct", "direction",
    "dirt", "disagree", "discover", "discuss", "disease", "dish", "distance", "divide", "do", "doctor",
    "dog", "dollar", "door", "double", "doubt", "down", "draw", "dream", "dress", "drink",
    "drive", "drop", "dry", "due", "during", "duty", "each", "ear", "early", "earn",
    "earth", "ease", "east", "easy", "eat", "edge", "educate", "effect", "effort", "egg",
    "eight", "either", "electric", "element", "else", "empty", "end", "enemy", "energy", "engine",
    "enjoy", "enough", "enter", "entire", "equal", "escape", "especially", "establish", "even", "evening",
    "event", "ever", "every", "evidence", "exact", "examine", "example", "except", "exchange", "excite",
    "excuse", "exercise", "exist", "expect", "expensive", "experience", "expert", "explain", "extend", "extra",
    "eye", "face", "fact", "factory", "fail", "fair", "fall", "false", "family", "famous",
    "far", "farm", "farmer", "fast", "fat", "father", "fault", "fear", "feed", "feel",
    "female", "few", "field", "fight", "figure", "fill", "film", "final", "find", "fine",
    "finger", "finish", "fire", "first", "fish", "fit", "five", "fix", "flag", "flat",
    "floor", "flow", "flower", "fly", "focus", "folk", "follow", "food", "foot", "for",
    "force", "foreign", "forest", "forever", "forget", "form", "forward", "four", "free", "fresh",
    "friend", "from", "front", "fruit", "full", "fun", "funny", "future", "gain", "game",
    "garden", "gas", "gate", "gather", "general", "gift", "girl", "give", "glad", "glass",
    "go", "goal", "god", "gold", "good", "government", "grab", "grade", "grain", "grandfather",
    "grass", "gray", "great", "green", "ground", "group", "grow", "growth", "guard", "guess",
    "guest", "guide", "gun", "guy", "habit", "hair", "half", "hall", "hand", "handle",
    "hang", "happen", "happy", "hard", "hat", "hate", "have", "he", "head", "health",
    "hear", "heart", "heat", "heavy", "help", "her", "here", "hero", "herself", "hide",
    "high", "hill", "him", "himself", "his", "history", "hit", "hold", "hole", "holiday",
    "home", "honest", "hope", "horse", "hospital", "hot", "hotel", "hour", "house", "how",
    "huge", "human", "hungry", "hunt", "hurry", "hurt", "husband", "i", "ice", "idea",
    "identify", "if", "ignore", "ill", "image", "imagine", "important", "improve", "in", "include",
    "income", "increase", "indeed", "indicate", "industry", "inform", "information", "inside", "instead", "intend",
    "interest", "into", "introduce", "invite", "iron", "island", "issue", "it", "item", "itself",
    "jacket", "job", "join", "joke", "journey", "joy", "judge", "jump", "just", "keep",
    "key", "kick", "kid", "kill", "kind", "king", "kiss", "kitchen", "knee", "knife",
    "knock", "know", "knowledge", "lady", "lake", "land", "large", "last", "late", "laugh",
    "law", "lay", "lead", "leader", "leaf", "learn", "least", "leave", "left", "leg",
    "legal", "less", "lesson", "let", "letter", "level", "lie", "life", "lift", "light",
    "like", "limit", "line", "lip", "liquid", "list", "listen", "little", "live", "load",
    "local", "lock", "long", "look", "lose", "loss", "lot", "loud", "love", "low",
    "luck", "lunch", "machine", "mad", "magazine", "main", "major", "make", "male", "man",
    "manage", "many", "map", "mark", "market", "marry", "match", "matter", "may", "maybe",
    "me", "meal", "mean", "measure", "meat", "meet", "meeting", "member", "memory", "mention",
    "message", "metal", "middle", "might", "mile", "military", "milk", "mind", "mine", "minute",
    "mirror", "miss", "mistake", "mix", "model", "modern", "moment", "money", "month", "moon",
    "more", "morning", "most", "mother", "motion", "mountain", "mouth", "move", "much", "music",
    "must", "my", "myself", "name", "narrow", "nation", "nature", "near", "neat", "necessary",
    "neck", "need", "neighbor", "neither", "nerve", "never", "new", "news", "next", "nice",
    "night", "no", "nobody", "nod", "noise", "none", "noon", "nor", "north", "nose",
    "not", "note", "nothing", "notice", "now", "number", "nurse", "object", "observe", "obtain",
    "obvious", "occur", "ocean", "odd", "of", "off", "offer", "office", "officer", "official",
    "often", "oil", "old", "on", "once", "one", "only", "onto", "open", "operate",
    "opinion", "opportunity", "opposite", "option", "or", "order", "ordinary", "organize", "original", "other",
    "ought", "our", "out", "outside", "over", "own", "owner", "page", "pain", "paint",
    "pair", "paper", "parent", "part", "partner", "party", "pass", "past", "path", "patient",
    "pattern", "pay", "payment", "peace", "pen", "people", "per", "perform", "perhaps", "period",
    "permit", "person", "phone", "photograph", "pick", "picture", "piece", "pig", "pile", "pilot",
    "pin", "pipe", "place", "plan", "plant", "plastic", "plate", "play", "player", "pleasant",
    "please", "plenty", "pocket", "point", "police", "policy", "pool", "poor", "popular", "population",
    "position", "positive", "possible", "post", "pot", "pound", "pour", "power", "practice", "prepare",
    "present", "president", "press", "pretty", "prevent", "price", "pride", "primary", "prince", "princess",
    "print", "prison", "private", "prize", "probably", "problem", "process", "produce", "product", "profession",
    "program", "project", "promise", "proof", "proper", "property", "protect", "proud", "prove", "provide",
    "public", "pull", "punish", "pure", "purple", "purpose", "push", "put", "quality", "quarter",
    "question", "quick", "quiet", "quite", "race", "radio", "rail", "rain", "raise", "range",
    "rare", "rate", "rather", "raw", "reach", "read", "ready", "real", "realize", "really",
    "reason", "receive", "recent", "recognize", "record", "red", "reduce", "refuse", "regard", "regular",
    "relation", "relax", "religion", "remain", "remember", "remove", "repeat", "replace", "reply", "report",
    "represent", "require", "rescue", "research", "respect", "respond", "rest", "result", "return", "reveal",
    "rich", "ride", "right", "ring", "rise", "risk", "river", "road", "rock", "role",
    "roll", "roof", "room", "root", "rope", "rough", "round", "route", "row", "rub",
    "rule", "run", "rush", "sad", "safe", "sail", "salary", "sale", "salt", "same",
    "sample", "sand", "save", "say", "scale", "scene", "school", "science", "score", "screen",
    "sea", "search", "season", "seat", "second", "secret", "section", "see", "seed", "seek",
    "seem", "sell", "send", "sense", "sentence", "separate", "series", "serious", "serve", "service",
    "set", "settle", "seven", "several", "severe", "sew", "shade", "shadow", "shake", "shall",
    "shame", "shape", "share", "sharp", "she", "sheet", "shelf", "shell", "shine", "ship",
    "shirt", "shock", "shoe", "shoot", "shop", "short", "should", "shoulder", "shout", "show",
    "shut", "sick", "side", "sight", "sign", "signal", "silence", "silent", "silver", "similar",
    "simple", "since", "sing", "single", "sink", "sister", "sit", "site", "situation", "six",
    "size", "skill", "skin", "skirt", "sky", "sleep", "slide", "slight", "slip", "slow",
    "small", "smart", "smell", "smile", "smoke", "smooth", "snake", "snow", "so", "soap",
    "society", "sock", "soft", "soil", "soldier", "solid", "some", "someone", "something", "sometimes",
    "son", "song", "soon", "sorry", "sort", "sound", "soup", "south", "space", "speak",
    "special", "speed", "spell", "spend", "spirit", "spite", "split", "spoon", "sport", "spot",
    "spread", "spring", "square", "staff", "stage", "stair", "stamp", "stand", "standard", "star",
    "stare", "start", "state", "station", "stay", "steady", "steal", "steam", "steel", "steep",
    "step", "stick", "still", "stomach", "stone", "stop", "store", "storm", "story", "straight",
    "strange", "stream", "street", "strength", "strike", "string", "strip", "strong", "structure", "struggle",
    "student", "study", "stuff", "stupid", "subject", "substance", "succeed", "such", "sudden", "suffer",
    "sugar", "suggest", "suit", "summer", "sun", "supply", "support", "suppose", "sure", "surface",
    "surprise", "surround", "sweet", "swim", "system", "table", "tail", "take", "talk", "tall",
    "tank", "taste", "tax", "tea", "teach", "teacher", "team", "tear", "telephone", "tell",
    "temperature", "term", "test", "than", "thank", "that", "the", "theater", "their", "them",
    "then", "theory", "there", "therefore", "these", "they", "thick", "thief", "thin", "thing",
    "think", "third", "this", "threat", "three", "throat", "through", "throw", "ticket", "tide",
    "tie", "tight", "till", "time", "tin", "tiny", "tip", "tire", "tired", "title",
    "to", "today", "toe", "together", "tomorrow", "tone", "tongue", "tonight", "too", "tool",
    "tooth", "top", "total", "touch", "tough", "toward", "town", "track", "trade", "traffic",
    "train", "travel", "treat", "tree", "trip", "trouble", "truck", "true", "trust", "truth",
    "try", "tube", "turn", "twelve", "twenty", "twice", "two", "type", "ugly", "uncle",
    "under", "understand", "unit", "until", "up", "upon", "upper", "urgent", "use", "usual",
    "valley", "valuable", "value", "various", "vegetable", "vehicle", "version", "very", "victim", "view",
    "village", "violence", "visit", "voice", "vote", "wage", "wait", "wake", "walk", "wall",
    "want", "war", "warm", "warn", "wash", "waste", "watch", "water", "wave", "way",
    "we", "weak", "wealth", "wear", "weather", "wedding", "week", "weigh", "weight", "welcome",
    "well", "west", "wet", "what", "whatever", "wheel", "when", "whenever", "where", "whether",
    "which", "while", "white", "who", "whole", "whom", "whose", "why", "wide", "wife",
    "wild", "will", "win", "wind", "window", "wine", "wing", "winter", "wire", "wise",
    "wish", "with", "within", "without", "woman", "wonder", "wood", "wooden", "word", "work",
    "worker", "world", "worry", "worse", "worth", "would", "wound", "wrap", "write", "wrong",
    "yard", "year", "yellow", "yes", "yesterday", "yet", "you", "young", "your", "yourself",
];

/// Checks if a lowercase word is present in the core familiar vocabulary list using binary search.
pub fn is_familiar_word(word: &str) -> bool {
    DALE_CHALL_CORE.binary_search(&word).is_ok()
}

/// Approximate syllable counter for English words.
pub fn count_syllables(word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    let bytes = word.as_bytes();
    let mut count = 0;
    let mut prev_is_vowel = false;

    for &b in bytes {
        let is_vowel = matches!(b, b'a' | b'e' | b'i' | b'o' | b'u' | b'y' | b'A' | b'E' | b'I' | b'O' | b'U' | b'Y');
        if is_vowel && !prev_is_vowel {
            count += 1;
        }
        prev_is_vowel = is_vowel;
    }

    // Subtract silent 'e' at the end of word if count > 1
    if bytes.len() > 2 && (bytes[bytes.len() - 1] == b'e' || bytes[bytes.len() - 1] == b'E') {
        if count > 1 {
            count -= 1;
        }
    }

    if count == 0 {
        1
    } else {
        count
    }
}

/// Readability statistics for a body of text.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReadabilityStats {
    pub word_count: usize,
    pub sentence_count: usize,
    pub syllable_count: usize,
    pub unfamiliar_word_count: usize,
    /// Flesch-Kincaid Grade Level (e.g. 8.0 = 8th grade reading level).
    pub flesch_kincaid_grade: f64,
}

/// Evaluates readability of an English text snippet.
pub fn evaluate_readability(words: &[&str], sentence_count: usize) -> ReadabilityStats {
    let word_count = words.len();
    let effective_sentences = if sentence_count == 0 { 1 } else { sentence_count };
    let mut syllable_count = 0;
    let mut unfamiliar_word_count = 0;

    for &w in words {
        syllable_count += count_syllables(w);
        if !is_familiar_word(w) {
            unfamiliar_word_count += 1;
        }
    }

    // Flesch-Kincaid Grade Level formula:
    // 0.39 * (words / sentences) + 11.8 * (syllables / words) - 15.59
    let words_per_sent = (word_count as f64) / (effective_sentences as f64);
    let syllables_per_word = if word_count == 0 {
        1.0
    } else {
        (syllable_count as f64) / (word_count as f64)
    };

    let fk_grade = 0.39 * words_per_sent + 11.8 * syllables_per_word - 15.59;
    let clamped_grade = if fk_grade < 0.0 { 0.0 } else { fk_grade };

    ReadabilityStats {
        word_count,
        sentence_count: effective_sentences,
        syllable_count,
        unfamiliar_word_count,
        flesch_kincaid_grade: clamped_grade,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_order() {
        // Verify that DALE_CHALL_CORE is strictly sorted
        for i in 1..DALE_CHALL_CORE.len() {
            assert!(
                DALE_CHALL_CORE[i - 1] < DALE_CHALL_CORE[i],
                "List not sorted at index {}: '{}' >= '{}'",
                i,
                DALE_CHALL_CORE[i - 1],
                DALE_CHALL_CORE[i]
            );
        }
    }

    #[test]
    fn test_familiar_words() {
        assert!(is_familiar_word("water"));
        assert!(is_familiar_word("friend"));
        assert!(is_familiar_word("understand"));
        // Complex / archaic / jargon words should be marked unfamiliar
        assert!(!is_familiar_word("indemnification"));
        assert!(!is_familiar_word("hypercholesterolemia"));
        assert!(!is_familiar_word("jurisprudence"));
    }

    #[test]
    fn test_syllable_counter() {
        assert_eq!(count_syllables("cat"), 1);
        assert_eq!(count_syllables("water"), 2);
        assert_eq!(count_syllables("family"), 3);
        assert_eq!(count_syllables("understand"), 3);
    }
}
