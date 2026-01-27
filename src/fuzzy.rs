use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

pub struct Fuzzy {
    matcher: Matcher,
    pattern: Pattern,
    buffer: Vec<char>,
}

impl Fuzzy {
    /// Create a new fuzzy search instance for the given search string.
    pub fn new(search: &str) -> Self {
        let mut config = Config::DEFAULT;
        config.prefer_prefix = true;
        Self {
            matcher: Matcher::new(config),
            pattern: Pattern::new(
                search,
                CaseMatching::Ignore,
                Normalization::Smart,
                AtomKind::Fuzzy,
            ),
            buffer: Vec::new(),
        }
    }
    /// Compare the candidate string against the pattern and return a score.
    /// Returns 0 if there is no match or the pattern is empty.
    pub fn score(&mut self, candidate: &str) -> u32 {
        let res = self.pattern.score(
            Utf32Str::new(candidate, &mut self.buffer),
            &mut self.matcher,
        );
        assert!(!matches!(res, Some(0)));
        res.unwrap_or_default()
    }
    /// Compare multiple candidate strings against the pattern and return the total score.
    pub fn score_many(&mut self, candidates: &[(&str, u32)]) -> u32 {
        candidates
            .iter()
            .filter_map(|(candidate, multiplier)| {
                (!candidate.is_empty()).then(|| self.score(candidate) * multiplier)
            })
            .sum()
    }
}
