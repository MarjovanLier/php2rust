//! Problem lines for refused or invalid input (P2R-03, P2R-171).

use mago_database::file::{File, FileId};

/// Who found the problem. `Tool` sorts first, so on an equal span the php2rust problem
/// is the one printed (P2R-03).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    Tool,
    Mago,
}

#[derive(Clone, Debug)]
pub struct Problem {
    /// The input file the span is in (P2R-171).
    pub file: FileId,
    pub start: u32,
    pub end: u32,
    pub origin: Origin,
    pub message: String,
}

/// Renders the problems of every input file, each file named as shown: files in the
/// order given, which is the order of their paths, and the span rule within one file
/// only (P2R-171).
pub fn render_files(files: &[(&str, &File)], problems: Vec<Problem>) -> String {
    let mut text: String = files
        .iter()
        .map(|(path, file)| {
            let own = problems.iter().filter(|p| p.file == file.id).cloned();
            render(path, file, own.collect())
        })
        .collect();
    // A finding whose span is in no input file is still printed, so exit 1 never comes
    // with an empty stderr.
    for problem in &problems {
        if files.iter().all(|(_, file)| file.id != problem.file) {
            text.push_str(&format!("{}: {}\n", files[0].0, problem.message));
        }
    }
    text
}

/// Renders problems as `path:line:column: message` lines. Where one span contains or
/// equals another, only the outer problem is printed (P2R-03).
fn render(path: &str, file: &File, mut problems: Vec<Problem>) -> String {
    // Outer spans sort before the spans they contain.
    problems.sort_by(|a, b| {
        a.start
            .cmp(&b.start)
            .then(b.end.cmp(&a.end))
            .then(a.origin.cmp(&b.origin))
            .then(a.message.cmp(&b.message))
    });
    let mut kept: Vec<Problem> = Vec::new();
    for problem in problems {
        if !kept
            .iter()
            .any(|k| k.start <= problem.start && problem.end <= k.end)
        {
            kept.push(problem);
        }
    }
    let mut lines: Vec<(u32, u32, String)> = kept
        .into_iter()
        .map(|p| {
            (
                file.line_number(p.start) + 1,
                file.column_number(p.start) + 1,
                p.message,
            )
        })
        .collect();
    lines.sort();
    lines.dedup();
    lines
        .into_iter()
        .map(|(line, column, message)| format!("{path}:{line}:{column}: {message}\n"))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    fn file(source: &[u8]) -> File {
        File::ephemeral(Cow::Borrowed(&b"t.php"[..]), Cow::Owned(source.to_vec()))
    }

    fn tool(start: u32, end: u32, message: &str) -> Problem {
        Problem {
            file: file(b"").id,
            start,
            end,
            origin: Origin::Tool,
            message: message.into(),
        }
    }

    #[test]
    fn inner_problem_is_dropped() {
        let f = file(b"abcdefghij");
        let out = render("t.php", &f, vec![tool(2, 4, "inner"), tool(0, 9, "outer")]);
        assert_eq!(out, "t.php:1:1: outer\n");
    }

    #[test]
    fn equal_span_prefers_the_tool_problem() {
        let f = file(b"abcdefghij");
        let mago = Problem {
            file: f.id,
            start: 1,
            end: 5,
            origin: Origin::Mago,
            message: "a mago".into(),
        };
        let out = render("t.php", &f, vec![mago, tool(1, 5, "z tool")]);
        assert_eq!(out, "t.php:1:2: z tool\n");
    }

    #[test]
    fn lines_sort_by_position_then_message() {
        let f = file(b"one\ntwo\nthree\n");
        let out = render(
            "t.php",
            &f,
            vec![tool(8, 9, "c"), tool(4, 5, "b"), tool(0, 1, "a")],
        );
        assert_eq!(out, "t.php:1:1: a\nt.php:2:1: b\nt.php:3:1: c\n");
    }

    #[test]
    fn crlf_line_numbers() {
        let f = file(b"<?php\r\n\r\necho 1;\r\n");
        let out = render("t.php", &f, vec![tool(9, 13, "here")]);
        assert_eq!(out, "t.php:3:1: here\n");
    }

    #[test]
    fn columns_count_bytes() {
        // "\u{e9}" is two bytes in UTF-8, so the x after it is byte column 4.
        let f = file("a\u{e9}x".as_bytes());
        let out = render("t.php", &f, vec![tool(3, 4, "x")]);
        assert_eq!(out, "t.php:1:4: x\n");
    }

    #[test]
    fn path_is_printed_as_given() {
        let f = file(b"x");
        let out = render("my dir/in put.php", &f, vec![tool(0, 1, "m")]);
        assert_eq!(out, "my dir/in put.php:1:1: m\n");
    }

    #[test]
    fn each_file_keeps_its_own_problems_and_spans() {
        let a = File::ephemeral(Cow::Borrowed(&b"a.php"[..]), Cow::Borrowed(&b"abcdef"[..]));
        let b = File::ephemeral(Cow::Borrowed(&b"b.php"[..]), Cow::Borrowed(&b"abcdef"[..]));
        let on = |file: &File, start, end, message: &str| Problem {
            file: file.id,
            ..tool(start, end, message)
        };
        // The span in a.php would hide the one in b.php if files were not apart.
        let problems = vec![on(&b, 2, 3, "inner of b"), on(&a, 0, 5, "outer of a")];
        let out = render_files(&[("src/a.php", &a), ("src/b.php", &b)], problems);
        assert_eq!(
            out,
            "src/a.php:1:1: outer of a\nsrc/b.php:1:3: inner of b\n"
        );
    }
}
