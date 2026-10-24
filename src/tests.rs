use super::*;

struct SearchSample {
    text: &'static str,
    query: &'static str,
    expect_result_in_vec: Vec<&'static str>,
}

fn get_test_text_contents_and_lines() -> (&'static str, [&'static str; 4]) {
    let text = concat!(
        "I'm nobody! Who are you?\n",
        "Are you nobody, too?\n",
        "Then there's a pair of us - don't tell!\n",
        "They'd banish us, you know.\n",
    );

    let mut lines = text.lines();

    let lines_array = std::array::from_fn(|_| lines.next().unwrap());
    assert!(lines.next().is_none());

    (text, lines_array)
}

fn get_regular_search_test_samples() -> [SearchSample; 4] {
    let (text, lines) = get_test_text_contents_and_lines();

    [
        SearchSample {
            text,
            query: "nobody",
            expect_result_in_vec: vec![lines[0], lines[1]],
        },
        SearchSample {
            text,
            query: "Who are you",
            expect_result_in_vec: vec![lines[0]],
        },
        SearchSample {
            text,
            query: "The",
            expect_result_in_vec: vec![lines[2], lines[3]],
        },
        SearchSample {
            text,
            query: "are",
            expect_result_in_vec: vec![lines[0]],
        },
    ]
}

#[test]
fn test_regular_search() {
    let test_samples = get_regular_search_test_samples();

    for sample in test_samples {
        assert_eq!(
            search(sample.query, sample.text).collect::<Vec<_>>(),
            sample.expect_result_in_vec,
            "\n\nquery: {}\n\ncontent:\n{}",
            sample.query,
            sample.text
        );
    }
}
