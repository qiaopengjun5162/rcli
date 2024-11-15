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

 rcli on  main [⇡?] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base took 1m 38.0s
➜ gp
Enumerating objects: 15, done.
Counting objects: 100% (15/15), done.
Delta compression using up to 12 threads
Compressing objects: 100% (5/5), done.
Writing objects: 100% (8/8), 2.85 KiB | 1.42 MiB/s, done.
Total 8 (delta 3), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (3/3), completed with 3 local objects.
To github.com:qiaopengjun5162/rcli.git
   39856f8..b09621b  main -> main

rcli on  main [?] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base took 4.4s
➜ git status
On branch main
Your branch is up to date with 'origin/main'.

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        .vscode/
        StudyNotes.md

nothing added to commit but untracked files present (use "git add" to track)

rcli on  main [?] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
➜ ga

rcli on  main [+] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git diff

rcli on  main [!+] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base took 48.6s
➜ git checkout -b chore/fix-cliff-config
Switched to a new branch 'chore/fix-cliff-config'

rcli on  chore/fix-cliff-config [!+] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git status
On branch chore/fix-cliff-config
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        new file:   .vscode/settings.json
        new file:   StudyNotes.md

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   cliff.toml


rcli on  chore/fix-cliff-config [!+] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git add .

rcli on  chore/fix-cliff-config [+] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git commit -a
check BOM - deprecated: use fix-byte-order-marker........................Passed
check for case conflicts.................................................Passed
check for merge conflicts................................................Passed
check for broken symlinks............................(no files to check)Skipped
check yaml...........................................(no files to check)Skipped
fix end of files.........................................................Passed
mixed line ending........................................................Passed
trim trailing whitespace.................................................Failed
- hook id: trailing-whitespace
- exit code: 1
- files were modified by this hook

Fixing StudyNotes.md

black................................................(no files to check)Skipped
cargo fmt............................................(no files to check)Skipped
cargo deny check.....................................(no files to check)Skipped
typos....................................................................Passed
cargo check..........................................(no files to check)Skipped
cargo clippy.........................................(no files to check)Skipped
cargo test...........................................(no files to check)Skipped

rcli on  chore/fix-cliff-config [!+] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
➜ ga

rcli on  chore/fix-cliff-config [+] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
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
cargo fmt............................................(no files to check)Skipped
cargo deny check.....................................(no files to check)Skipped
typos....................................................................Passed
cargo check..........................................(no files to check)Skipped
cargo clippy.........................................(no files to check)Skipped
cargo test...........................................(no files to check)Skipped
[chore/fix-cliff-config d245afb] chore: correct the project url in cliff config
 4 files changed, 142 insertions(+), 2 deletions(-)
 create mode 100644 .vscode/settings.json
 create mode 100644 StudyNotes.md

rcli on  chore/fix-cliff-config is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base took 1m 33.1s
➜ git push origin chore/fix-cliff-config
Enumerating objects: 10, done.
Counting objects: 100% (10/10), done.
Delta compression using up to 12 threads
Compressing objects: 100% (5/5), done.
Writing objects: 100% (7/7), 1.89 KiB | 1.89 MiB/s, done.
Total 7 (delta 3), reused 0 (delta 0), pack-reused 0 (from 0)
remote: Resolving deltas: 100% (3/3), completed with 3 local objects.
remote:
remote: Create a pull request for 'chore/fix-cliff-config' on GitHub by visiting:
remote:      https://github.com/qiaopengjun5162/rcli/pull/new/chore/fix-cliff-config
remote:
To github.com:qiaopengjun5162/rcli.git
 * [new branch]      chore/fix-cliff-config -> chore/fix-cliff-config

rcli on  chore/fix-cliff-config is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base took 4.4s
➜ git checkout main
Switched to branch 'main'
Your branch is behind 'origin/main' by 1 commit, and can be fast-forwarded.
  (use "git pull" to update your local branch)

rcli on  main [⇣] is 📦 0.1.0 via 🦀 1.76.0 via 🅒 base
➜ git pull
Updating b09621b..281a565
Fast-forward
 .vscode/settings.json |   5 ++++
 StudyNotes.md         | 135 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 _typos.toml           |   2 +-
 cliff.toml            |   2 +-
 4 files changed, 142 insertions(+), 2 deletions(-)
 create mode 100644 .vscode/settings.json
 create mode 100644 StudyNotes.md

cargo add anyhow
cargo add clap --features derive
cargo add csv
cargo add serde --features derive
cargo add serde_json

brew install duckdb

rcli on  dev [!] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base
➜ duckdb
v1.1.3 19864453f7
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
D select * from read_csv('assets/juventus.csv', auto_detect=true);

cargo run -- csv -i assets/juventus.csv
brew install tokei
tokei .
cargo add toml
cargo add serde_yaml
```

## 参考

- <https://github.com/tyr-rust-bootcamp/template/issues/3>
- <https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html>
- <https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_3/index.html>
- <https://duckdb.org/>
- <https://docs.rs/csv/latest/csv/>
- <https://learnxinyminutes.com/docs/rust/>
- <https://github.com/XAMPPRocky/tokei>
- <https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html>
- <https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_3/index.html>
- <https://duckdb.org/>
- <https://docs.rs/csv/latest/csv/>
- <https://learnxinyminutes.com/docs/rust/>
- <https://github.com/XAMPPRocky/tokei>
