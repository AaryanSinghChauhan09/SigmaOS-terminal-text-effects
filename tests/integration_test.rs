// Integration test for SigmaOS-terminal-text-effects
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_terminal_text_effects as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
