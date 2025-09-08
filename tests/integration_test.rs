use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn run_ock(args: Vec<&str>) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(&args)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run_ock_with_stdin(stdin_data: &str, args: Vec<&str>) -> String {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(stdin_data.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait for child");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_basic_row_selection() {
    let input = "line1
line2
line3
line4
line5";
    let output = run_ock_with_stdin(input, vec!["-r", "2"]);
    assert!(output.contains("line2"));
    assert!(!output.contains("line1"));
    assert!(!output.contains("line3"));
}

#[test]
fn test_row_range_selection() {
    let input = "line1
line2
line3
line4
line5";
    let output = run_ock_with_stdin(input, vec!["-r", "2:4"]);
    assert!(!output.contains("line1"));
    assert!(output.contains("line2"));
    assert!(output.contains("line3"));
    assert!(output.contains("line4"));
    assert!(!output.contains("line5"));
}

#[test]
fn test_row_range_with_step() {
    // BUG: Step values are incorrectly decremented by 1 in selector.rs line 96
    // This test expects CORRECT behavior (step 2 means every 2nd row)
    // It currently FAILS due to the bug where step 2 becomes step 1
    let input = "line1
line2
line3
line4
line5
line6";
    let output = run_ock_with_stdin(input, vec!["-r", "1:6:2"]);
    // With step=2, should select lines 1, 3, 5 (indices 0, 2, 4)
    assert!(output.contains("line1"));
    assert!(!output.contains("line2"), "Step 2 should skip line2");
    assert!(output.contains("line3"));
    assert!(!output.contains("line4"), "Step 2 should skip line4");
    assert!(output.contains("line5"));
    assert!(!output.contains("line6"), "Step 2 should skip line6");
}

#[test]
fn test_regex_start_never_matches() {
    use std::process::{Command, Stdio};

    let input = "line1\nline2\nline3";
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(["-r", "foo:2"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to wait for child");

    assert!(
        output.status.success(),
        "Process failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).trim().is_empty());
}

#[test]
fn test_start_index_greater_than_end() {
    use std::process::{Command, Stdio};

    let input = "line1\nline2\nline3\nline4";
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(["-r", "5:3"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to wait for child");

    assert!(
        output.status.success(),
        "Process failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).trim().is_empty());
}

#[test]
fn test_column_selection() {
    let input = "col1 col2 col3
data1 data2 data3";
    let output = run_ock_with_stdin(input, vec!["-c", "2"]);
    assert!(output.contains("col2"));
    assert!(output.contains("data2"));
    assert!(!output.contains("col1"));
    assert!(!output.contains("col3"));
}

#[test]
fn test_column_multiple_selection() {
    let input = "A B C D
1 2 3 4";
    let output = run_ock_with_stdin(input, vec!["-c", "1,3"]);
    assert!(output.contains("A"));
    assert!(output.contains("C"));
    assert!(output.contains("1"));
    assert!(output.contains("3"));
    assert!(!output.contains("B"));
    assert!(!output.contains("D"));
}

#[test]
fn test_row_and_column_selection() {
    let input = "H1 H2 H3 H4
R1C1 R1C2 R1C3 R1C4
R2C1 R2C2 R2C3 R2C4
R3C1 R3C2 R3C3 R3C4";
    let output = run_ock_with_stdin(input, vec!["-r", "2:3", "-c", "2,4"]);
    assert!(output.contains("R1C2"));
    assert!(output.contains("R1C4"));
    assert!(output.contains("R2C2"));
    assert!(output.contains("R2C4"));
    assert!(!output.contains("H1"));
    assert!(!output.contains("R3C1"));
}

#[test]
fn test_regex_row_selection() {
    let input = "header
python process
java process
python script
rust program";
    let output = run_ock_with_stdin(input, vec!["-r", "python"]);
    // Regex "python" matches both lines containing "python"
    assert!(output.contains("python")); // Will match both lines
    assert!(!output.contains("java"));
    assert!(!output.contains("rust"));
    assert!(!output.contains("header"));
}

#[test]
fn test_regex_column_selection() {
    let input = "USER PID COMMAND %CPU %MEM
root 1 init 0.1 0.2
user 123 firefox 5.2 3.1";
    let output = run_ock_with_stdin(input, vec!["-c", "pid,%cpu"]);
    assert!(output.contains("PID"));
    assert!(output.contains("%CPU"));
    assert!(output.contains("1"));
    assert!(output.contains("123"));
    assert!(output.contains("0.1"));
    assert!(output.contains("5.2"));
    assert!(!output.contains("USER"));
    assert!(!output.contains("COMMAND"));
}

#[test]
fn test_custom_column_delimiter() {
    let input = "a,b,c,d
1,2,3,4";
    let output = run_ock_with_stdin(input, vec!["-c", "2,4", "--column-delimiter", ","]);
    assert!(output.contains("b"));
    assert!(output.contains("d"));
    assert!(output.contains("2"));
    assert!(output.contains("4"));
    assert!(!output.contains("a"));
    assert!(!output.contains("c"));
}

#[test]
fn test_custom_row_delimiter() {
    let input = "row1;row2;row3;row4";
    let output = run_ock_with_stdin(input, vec!["-r", "2:3", "--row-delimiter", ";"]);
    assert!(output.contains("row2"));
    assert!(output.contains("row3"));
    assert!(!output.contains("row1"));
    assert!(!output.contains("row4"));
}

#[test]
fn test_tab_delimiter() {
    let input = "f1\tf2\tf3
v1\tv2\tv3";
    let output = run_ock_with_stdin(input, vec!["-c", "2", "--column-delimiter", r"\t"]);
    assert!(output.contains("f2"));
    assert!(output.contains("v2"));
    assert!(!output.contains("f1"));
    assert!(!output.contains("f3"));
}

#[test]
fn test_file_input() {
    let mut temp_file = NamedTempFile::new().unwrap();
    let content = "file_line1
file_line2
file_line3";
    writeln!(temp_file, "{}", content).unwrap();

    let file_path = temp_file.path().to_str().unwrap();
    let output = run_ock(vec!["-r", "2", file_path]);
    assert!(output.contains("file_line2"));
    assert!(!output.contains("file_line1"));
    assert!(!output.contains("file_line3"));
}

#[test]
fn test_direct_text_input() {
    let output = run_ock(vec!["-r", "1", "direct text input"]);
    // Check that we got some output with the expected words
    assert!(!output.is_empty());
    // The words should be in the output, possibly with formatting
    let output_lower = output.to_lowercase();
    assert!(output_lower.contains("direct") || output.contains("direct"));
}

#[test]
fn test_empty_selection() {
    let input = "line1
line2
line3";
    let output = run_ock_with_stdin(input, vec![]);
    // With no selectors, should output everything
    assert!(output.contains("line1"));
    assert!(output.contains("line2"));
    assert!(output.contains("line3"));
}

#[test]
fn test_complex_regex_patterns() {
    let input = "USER_ID USER_NAME
123 john_doe
456 jane_smith
789 bob_jones";

    // Test case-insensitive matching
    let output = run_ock_with_stdin(input, vec!["-r", "JANE"]);
    assert!(output.contains("jane_smith"));
    assert!(!output.contains("john_doe"));

    // Test partial matching
    let output2 = run_ock_with_stdin(input, vec!["-c", "name"]);
    assert!(output2.contains("USER_NAME"));
    assert!(output2.contains("john_doe"));
    assert!(output2.contains("jane_smith"));
    assert!(!output2.contains("123"));
}

#[test]
fn test_regex_range_selection() {
    let input = "START_MARKER
data1
data2
data3
END_MARKER
extra_data";

    let output = run_ock_with_stdin(input, vec!["-r", "start:end"]);
    assert!(output.contains("START_MARKER"));
    assert!(output.contains("data1"));
    assert!(output.contains("data2"));
    assert!(output.contains("data3"));
    assert!(output.contains("END_MARKER"));
    assert!(!output.contains("extra_data"));
}

#[test]
fn test_multiple_row_selectors() {
    let input = "line1
line2
line3
line4
line5";

    let output = run_ock_with_stdin(input, vec!["-r", "1,3,5"]);
    assert!(output.contains("line1"));
    assert!(!output.contains("line2"));
    assert!(output.contains("line3"));
    assert!(!output.contains("line4"));
    assert!(output.contains("line5"));
}

#[test]
fn test_large_dataset() {
    let input: String = (1..=100)
        .map(|i| format!("row{} col1 col2 col3", i))
        .collect::<Vec<_>>()
        .join("\n");

    let output = run_ock_with_stdin(&input, vec!["-r", "10:20", "-c", "1,3"]);
    assert!(output.contains("row10"));
    assert!(output.contains("row20"));
    assert!(!output.contains("row9"));
    assert!(!output.contains("row21"));
    assert!(output.contains("col2"));
    assert!(!output.contains("col1"));
    assert!(!output.contains("col3"));
}

#[test]
fn test_empty_input() {
    let output = run_ock_with_stdin("", vec!["-r", "1"]);
    assert_eq!(output.trim(), "");
}

#[test]
fn test_whitespace_handling() {
    let input = "  col1   col2    col3  
  data1   data2    data3  ";

    let output = run_ock_with_stdin(input, vec!["-c", "2"]);
    assert!(output.contains("col2"));
    assert!(output.contains("data2"));
}

#[test]
fn test_mixed_delimiters() {
    let input = "a b c,d e
1 2 3,4 5";

    // Should split on whitespace by default
    let output = run_ock_with_stdin(input, vec!["-c", "3"]);
    assert!(output.contains("c,d"));
    assert!(output.contains("3,4"));
}

#[test]
fn test_unicode_support() {
    let input = "英文 中文 日本語
hello 你好 こんにちは";

    let output = run_ock_with_stdin(input, vec!["-c", "2"]);
    assert!(output.contains("中文"));
    assert!(output.contains("你好"));
}

#[test]
fn test_special_characters() {
    let input = "col@1 col#2 col$3
val!1 val%2 val^3";

    let output = run_ock_with_stdin(input, vec!["-c", "2"]);
    assert!(output.contains("col#2"));
    assert!(output.contains("val%2"));
}

#[test]
fn test_ps_aux_simulation() {
    // Simulate ps aux output
    let input = "USER       PID  %CPU  %MEM     VSZ    RSS TTY      STAT START   TIME COMMAND
root         1   0.0   0.0  168936  11408 ?        Ss   Oct30   0:48 /sbin/init
root        42   0.0   0.0   41796   3992 ?        S<s  Oct30   0:00 /lib/systemd/systemd-journald
www-data   847   0.2   1.3  342456  52788 ?        S    Nov01  12:34 apache2 -k start
mysql      923   0.5   3.2  892344 129876 ?        Ssl  Nov01  23:45 /usr/sbin/mysqld";

    // Get PID and COMMAND columns
    let output = run_ock_with_stdin(input, vec!["-c", "pid,command"]);
    assert!(output.contains("PID"));
    assert!(output.contains("COMMAND"));
    assert!(output.contains("1"));
    assert!(output.contains("/sbin/init"));
    assert!(output.contains("847"));
    assert!(output.contains("apache2"));

    // Get processes containing "systemd"
    let output2 = run_ock_with_stdin(input, vec!["-r", "systemd"]);
    assert!(output2.contains("systemd-journald"));
    assert!(!output2.contains("apache2"));
    assert!(!output2.contains("mysqld"));
}

#[test]
fn test_csv_processing() {
    let input = "Name,Age,City,Country
John,25,NewYork,USA
Jane,30,London,UK
Bob,35,Tokyo,Japan";

    // Select Age and Country columns
    let output = run_ock_with_stdin(input, vec!["-c", "2,4", "--column-delimiter", ","]);
    assert!(output.contains("Age"));
    assert!(output.contains("Country"));
    assert!(output.contains("25"));
    assert!(output.contains("USA"));
    assert!(output.contains("30"));
    assert!(output.contains("UK"));
    assert!(!output.contains("Name"));
    assert!(!output.contains("City"));
}

#[test]
fn test_edge_case_single_row() {
    let input = "only_one_row";
    let output = run_ock_with_stdin(input, vec!["-r", "1"]);
    assert!(output.contains("only_one_row"));
}

#[test]
fn test_edge_case_single_column() {
    let input = "col1
col2
col3";
    let output = run_ock_with_stdin(input, vec!["-c", "1"]);
    assert!(output.contains("col1"));
    assert!(output.contains("col2"));
    assert!(output.contains("col3"));
}

#[test]
fn test_out_of_bounds_indices() {
    let input = "a b c\n1 2 3";

    // Requesting a non-existent column should produce no output
    let output = run_ock_with_stdin(input, vec!["-c", "10"]);
    assert!(output.trim().is_empty());

    // Request row 10 (doesn't exist) - correctly returns empty
    let output2 = run_ock_with_stdin(input, vec!["-r", "10"]);
    assert_eq!(output2.trim(), "");
}

// STDIN-FOCUSED TESTS: Testing stdin input functionality specifically
// These tests focus on the stdin reading behavior and edge cases

#[test]
fn test_stdin_basic_functionality() {
    // Test that stdin input works with basic data
    let input = "line1\nline2\nline3";
    let output = run_ock_with_stdin(input, vec![]);

    assert!(output.contains("line1"));
    assert!(output.contains("line2"));
    assert!(output.contains("line3"));
}

#[test]
fn test_stdin_empty_input() {
    // Test stdin with completely empty input
    let output = run_ock_with_stdin("", vec![]);
    assert_eq!(output.trim(), "");
}

#[test]
fn test_stdin_single_line() {
    // Test stdin with just one line
    let output = run_ock_with_stdin("single_line", vec![]);
    assert!(output.contains("single_line"));
}

#[test]
fn test_stdin_with_trailing_newlines() {
    // Test stdin preserves trailing newlines correctly
    let input = "line1\nline2\n";
    let output = run_ock_with_stdin(input, vec![]);

    assert!(output.contains("line1"));
    assert!(output.contains("line2"));
}

#[test]
fn test_stdin_with_multiple_newlines() {
    // Test stdin handles multiple consecutive newlines
    // split() filters out empty strings, so blank lines are ignored
    let input = "line1\n\n\nline2\n\nline3";
    let output = run_ock_with_stdin(input, vec![]);

    assert!(output.contains("line1"));
    assert!(output.contains("line2"));
    assert!(output.contains("line3"));
    // Expect only the non-empty lines to remain
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines, vec!["line1", "line2", "line3"]);
}

#[test]
fn test_stdin_with_whitespace_only_lines() {
    // Test stdin with lines containing only whitespace
    let input = "line1\n   \n\t\nline2";
    let output = run_ock_with_stdin(input, vec![]);

    assert!(output.contains("line1"));
    assert!(output.contains("line2"));
}

#[test]
fn test_stdin_large_input() {
    // Test stdin with large amount of data
    let large_input: String = (1..=1000)
        .map(|i| format!("data_line_{}", i))
        .collect::<Vec<_>>()
        .join("\n");

    let output = run_ock_with_stdin(&large_input, vec!["-r", "1:10"]);

    assert!(output.contains("data_line_1"));
    assert!(output.contains("data_line_10"));
    assert!(!output.contains("data_line_11"));
    assert!(!output.contains("data_line_1000"));
}

#[test]
fn test_stdin_utf8_safety() {
    // Test stdin with UTF8 text including special characters - should handle gracefully
    let input = "normal_text\nmore_text_with_émojis_🦀";
    let output = run_ock_with_stdin(input, vec![]);

    assert!(output.contains("normal_text"));
    assert!(output.contains("more_text_with_émojis_🦀"));
}

#[test]
fn test_stdin_vs_parse_input_consistency() {
    // Ensure stdin input behaves consistently with direct text input
    let test_data = "test_line1\ntest_line2\ntest_line3";

    // Test via stdin
    let stdin_output = run_ock_with_stdin(test_data, vec!["-r", "2"]);

    // Test via direct text (should be treated as literal if not a file)
    let direct_output = run_ock(vec!["-r", "2", test_data]);

    // Both should contain the same selected row
    assert!(stdin_output.contains("test_line2"));
    assert!(direct_output.contains("test_line2"));
}

#[test]
fn test_stdin_with_complex_selectors() {
    // Test stdin input with complex row and column selectors
    let input = "col1 col2 col3 col4
row1_c1 row1_c2 row1_c3 row1_c4
row2_c1 row2_c2 row2_c3 row2_c4
row3_c1 row3_c2 row3_c3 row3_c4";

    let output = run_ock_with_stdin(input, vec!["-r", "2:3", "-c", "2,4"]);

    assert!(output.contains("row1_c2"));
    assert!(output.contains("row1_c4"));
    assert!(output.contains("row2_c2"));
    assert!(output.contains("row2_c4"));
    assert!(!output.contains("row3_c1")); // Row 3 should be excluded
    assert!(!output.contains("row1_c1")); // Column 1 should be excluded
}

#[test]
fn test_stdin_performance_benchmark() {
    // Performance test - ensure stdin can handle moderately large datasets efficiently
    let input: String = (1..=10000)
        .map(|i| format!("row{} data{} info{} value{}", i, i * 2, i * 3, i * 4))
        .collect::<Vec<_>>()
        .join("\n");

    // This should complete in reasonable time
    let output = run_ock_with_stdin(&input, vec!["-r", "5000:5010", "-c", "1,3"]);

    assert!(output.contains("row5000"));
    assert!(output.contains("row5010"));
    assert!(output.contains("info15000")); // 5000 * 3
    assert!(output.contains("info15030")); // 5010 * 3
    assert!(!output.contains("row4999"));
    assert!(!output.contains("row5011"));
}
