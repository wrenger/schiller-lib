use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

pub struct Fuzzy {
    matcher: Matcher,
    pattern: Pattern,
    buffer: Vec<char>,
}

impl Fuzzy {
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
    pub fn score(&mut self, candidate: &str) -> Option<u32> {
        self.pattern.score(
            Utf32Str::new(candidate, &mut self.buffer),
            &mut self.matcher,
        )
    }
    pub fn score_many(&mut self, candidates: &[(&str, u32)]) -> Option<u32> {
        let mut result = None;
        for (candidate, multiplyer) in candidates {
            if !candidate.is_empty()
                && let Some(score) = self.score(candidate)
            {
                result = Some(result.unwrap_or_default() + score * multiplyer);
            }
        }
        result
    }
}
