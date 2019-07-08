set -ex;

(cd www; yarn build);
(cd crate; cargo build);
