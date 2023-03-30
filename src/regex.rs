use std::sync::Arc;

use pyo3::prelude::*;

use crate::{captures::Captures, error::RegexResult, match_struct::Match};

#[pyclass]
#[derive(Debug)]
pub struct Regex(Arc<regex::Regex>);

// constructor
#[pymethods]
impl Regex {
    // core methods
    #[new]
    #[pyo3(signature = (
        pattern,
        *,
        case_insensitive=None,
        dfa_size_limit=None,
        dot_matches_new_line=None,
        ignore_whitespace=None,
        multi_line=None,
        nest_limit=None,
        octal=None,
        size_limit=None,
        swap_greed=None,
        unicode=None,
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pattern: &str,
        case_insensitive: Option<bool>,
        dfa_size_limit: Option<usize>,
        dot_matches_new_line: Option<bool>,
        ignore_whitespace: Option<bool>,
        multi_line: Option<bool>,
        nest_limit: Option<u32>,
        octal: Option<bool>,
        size_limit: Option<usize>,
        swap_greed: Option<bool>,
        unicode: Option<bool>,
    ) -> RegexResult<Self> {
        let mut builder = regex::RegexBuilder::new(pattern);

        if let Some(val) = case_insensitive {
            builder.case_insensitive(val);
        }
        if let Some(val) = dfa_size_limit {
            builder.dfa_size_limit(val);
        }
        if let Some(val) = dot_matches_new_line {
            builder.dot_matches_new_line(val);
        }
        if let Some(val) = ignore_whitespace {
            builder.ignore_whitespace(val);
        }
        if let Some(val) = multi_line {
            builder.multi_line(val);
        }
        if let Some(val) = nest_limit {
            builder.nest_limit(val);
        }
        if let Some(val) = octal {
            builder.octal(val);
        }
        if let Some(val) = size_limit {
            builder.size_limit(val);
        }
        if let Some(val) = swap_greed {
            builder.swap_greed(val);
        }
        if let Some(val) = unicode {
            builder.unicode(val);
        }

        builder
            .build()
            .map(Arc::new)
            .map(Self)
            .map_err(|e| e.into())
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    #[pyo3(signature = (text, start=None))]
    pub fn find(&self, text: &str, start: Option<usize>) -> Option<Match> {
        let mat = if let Some(start) = start {
            self.0.find_at(text, start)
        } else {
            self.0.find(text)
        };

        mat.map(|m| m.into())
    }

    pub fn find_iter(&self, text: &str) -> Vec<Match> {
        self.0.find_iter(text).map(|m| m.into()).collect()
    }

    pub fn captures(&self, text: String) -> Option<Captures> {
        Captures::try_new(Arc::new(text), |text| self.0.captures(text).ok_or(())).ok()
    }

    pub fn captures_iter(&self, text: String) -> Vec<Captures> {
        let text = Arc::new(text);
        self.0.captures_iter(&text)
        .map(|caps| Captures::new(text.clone(), |text| caps.adopt(text)))
        .collect()
    }

    #[pyo3(signature = (text, limit=None))]
    pub fn split(&self, text: &str, limit: Option<usize>) -> Vec<String> {
        if let Some(limit) = limit {
            self.0.splitn(text, limit).map(|v| v.to_owned()).collect()
        } else {
            self.0.split(text).map(|v| v.to_owned()).collect()
        }
    }

    #[pyo3(signature = (text, rep, limit=None))]
    pub fn replace(&self, text: &str, rep: &str, limit: Option<usize>) -> String {
        if let Some(limit) = limit {
            self.0.replacen(text, limit, rep).into_owned()
        } else {
            self.0.replace_all(text, rep).into_owned()
        }
    }

    // magic
    pub fn __str__(&self) -> &str {
        self.0.as_str()
    }

    pub fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}
