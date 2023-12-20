struct Setup {
    cmd: assert_cmd::Command,
    email: String,
    password: String,
}

fn setup() -> Setup {
    let cmd = assert_cmd::Command::cargo_bin("jobcan").expect("Failed to find jobcan binary");
    let email = std::env::var("JOBCAN_EMAIL").expect("JOBCAN_EMAIL is not set");
    let password = std::env::var("JOBCAN_PASSWORD").expect("JOBCAN_PASSWORD is not set");
    Setup {
        cmd,
        email,
        password,
    }
}

#[tokio::test]
async fn process_of_getting_work_status_correctly() {
    let Setup {
        mut cmd,
        email,
        password,
    } = setup();
    cmd.arg("status")
        .arg("--email")
        .arg(email)
        .arg("--password")
        .arg(password)
        .assert()
        .success();
}

#[tokio::test]
async fn process_of_getting_group_list_correctly() {
    let Setup {
        mut cmd,
        email,
        password,
    } = setup();
    cmd.arg("list-groups")
        .arg("--email")
        .arg(email)
        .arg("--password")
        .arg(password)
        .assert()
        .success();
}
