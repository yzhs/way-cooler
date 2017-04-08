//! Benchmarks operations on the tree.
extern crate test;

use self::test::Bencher;
use ::layout::ContainerType;
use ::layout::core::tree::tests::basic_tree;

#[bench]
fn basic_get_active_container(b: &mut Bencher) {
    let tree = basic_tree();
    b.iter(|| {
        let _active = tree.get_active_container().unwrap();
    })
}

#[bench]
fn basic_switch_workspace(b: &mut Bencher) {
    let mut tree = basic_tree();
    b.iter(|| {
        tree.switch_to_workspace("2")
    })
}

#[bench]
fn basic_tile(b: &mut Bencher) {
    let mut tree = basic_tree();
    let root_ix = tree.tree.root_ix();
    b.iter(|| {
        tree.layout(root_ix)
    })
}
