use std::process::{Child, Command};

// Uses RAII to ensure child process is killed upon going out of scope
struct ScopedCmd {
    child: Child,
}

impl ScopedCmd {
    fn new(cmd: &str, args: &[&str]) -> Self {
        let child = Command::new(cmd)
            .args(args)
            .spawn()
            .expect("Error starting subproc");

        Self { child }
    }
}

impl Drop for ScopedCmd {
    fn drop(&mut self) {
        self.child.kill().expect("Failed to killed child process")
    }
}

struct MongodProc {
    db_proc: ScopedCmd,
    db_name: Option<String>,
}

impl MongodProc {
    fn new(db_name: &str) -> Self {
        // db proc is now running
        let db_proc = ScopedCmd::new("mongod", &["--dbpath=db"]);

        Self {
            db_proc,
            db_name: Some("db_test".to_string()),
        }
    }
}

impl Drop for MongodProc {
    fn drop(&mut self) {
        // RAII to clean up child resources but first clean up test_db
        let _ = ScopedCmd::new(
            "mongo",
            &["--eval", "db.getSiblingDB('test_db').dropDatabase();"],
        );
    }
}

#[test]
fn test_user_api() {
    let _ = MongodProc::new("db_test");
    // insert something into the db using the add_user endpoint
}

#[test]
fn test_post_api() {}

#[test]
fn test_safe_delete() {}
