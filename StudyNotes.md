# rcli

## 实操

```shell
cargo generate tyr-rust-bootcamp/template
⚠️   Favorite `tyr-rust-bootcamp/template` not found in config, using it as a git repository: https://github.com/tyr-rust-bootcamp/template.git
🤷   Project Name: rcli
🔧   Destination: /Users/qiaopengjun/Code/rust/rcli ...
🔧   project-name: rcli ...
🔧   Generating template ...
🔧   Moving generated files into: `/Users/qiaopengjun/Code/rust/rcli`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /Users/qiaopengjun/Code/rust/rcli

~/Code/rust via 🅒 base took 42.7s
➜
cd rcli/

rcli on  master [?] is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base
➜
c


rcli on  master [?] is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git status
On branch master

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        .github/
        .gitignore
        .pre-commit-config.yaml
        CHANGELOG.md
        Cargo.lock
        Cargo.toml
        README.md
        _typos.toml
        assets/
        cliff.toml
        deny.toml
        src/

nothing added to commit but untracked files present (use "git add" to track)

rcli on  master [?] is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git commit -a
On branch master

Initial commit

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        .github/
        .gitignore
        .pre-commit-config.yaml
        CHANGELOG.md
        Cargo.lock
        Cargo.toml
        README.md
        _typos.toml
        assets/
        cliff.toml
        deny.toml
        src/

nothing added to commit but untracked files present (use "git add" to track)

rcli on  master [?] is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git add .

rcli on  master [+] is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git commit -a
[master (root-commit) 39856f8] init the project
 13 files changed, 572 insertions(+)
 create mode 100644 .github/workflows/build.yml
 create mode 100644 .gitignore
 create mode 100644 .pre-commit-config.yaml
 create mode 100644 CHANGELOG.md
 create mode 100644 Cargo.lock
 create mode 100644 Cargo.toml
 create mode 100644 README.md
 create mode 100644 _typos.toml
 create mode 100644 assets/README.md
 create mode 100644 assets/juventus.csv
 create mode 100644 cliff.toml
 create mode 100644 deny.toml
 create mode 100644 src/main.rs

rcli on  master is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base took 42.9s
➜ git remote add origin git@github.com:qiaopengjun5162/rcli.git
git branch -M main
git push -u origin main
Enumerating objects: 19, done.
Counting objects: 100% (19/19), done.
Delta compression using up to 12 threads
Compressing objects: 100% (14/14), done.
Writing objects: 100% (19/19), 8.90 KiB | 4.45 MiB/s, done.
Total 19 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
To github.com:qiaopengjun5162/rcli.git
 * [new branch]      main -> main
branch 'main' set up to track 'origin/main'.

rcli on  main is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base took 5.8s
➜ touch StudyNotes.md

rcli on  main [?] is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base
➜

rcli on  main [!?] is 📦 template@0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git commit -a
check BOM - deprecated: use fix-byte-order-marker........................Passed
check for case conflicts.................................................Passed
check for merge conflicts................................................Passed
check for broken symlinks............................(no files to check)Skipped
check yaml...........................................(no files to check)Skipped
fix end of files.........................................................Passed
mixed line ending........................................................Passed
trim trailing whitespace.................................................Passed
black................................................(no files to check)Skipped
cargo fmt................................................................Passed
cargo deny check.........................................................Passed
typos....................................................................Passed
cargo check..............................................................Passed
cargo clippy.............................................................Passed
cargo test...............................................................Passed
[main b09621b] first commit
 5 files changed, 123 insertions(+), 169 deletions(-)
```

## 参考

- <https://github.com/tyr-rust-bootcamp/template/issues/3>
