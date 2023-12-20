<h1 align="center">
  Jobcan-cli
</h1>

<h4 align="center">
  A command line tool to operate Jobcan.
</h1>

<div align="center">
<a href="https://github.com/amiyzku/jobcan-cli/releases/latest">
  <img alt="GitHub release (with filter)" src="https://img.shields.io/github/v/release/amiyzku/jobcan-cli?style=for-the-badge">
</a>
<img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/amiyzku/jobcan-cli?style=for-the-badge">
<img alt="GitHub top language" src="https://img.shields.io/github/languages/top/amiyzku/jobcan-cli?style=for-the-badge">
<a href="https://github.com/amiyzku/jobcan-cli/blob/master/LICENSE">
  <img alt="GitHub License" src="https://img.shields.io/github/license/amiyzku/jobcan-cli?style=for-the-badge">
</a>
</div>

## Usage

```plaintext
$ jobcan --help
Usage: jobcan <COMMAND>

Commands:
  clock-in     Login to Jobcan and clock in
  clock-out    Login to Jobcan and clock out
  start-break  Login to Jobcan and start break
  end-break    Login to Jobcan and end break
  status       Login to Jobcan and get current working status
  list-groups  Login to Jobcan and list groups which you belong to
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Run 'jobcan \<COMMAND> --help' for more information on a command.

E.g. 'jobcan clock-in --help'

```plaintext
$ jobcan clock-in --help
Login to Jobcan and clock in

Usage: jobcan clock-in [OPTIONS]

Options:
  -e, --email <EMAIL>        Account email. Default to $JOBCAN_EMAIL if not set.
  -p, --password <PASSWORD>  Account password. Default to $JOBCAN_PASSWORD if not set.
      --group-id <GROUP_ID>  Group ID. Default to $JOBCAN_GROUP_ID if not set.
      --night-shift          Night-Shift mode.
      --notes <NOTES>        Notes to be added to the stamp.
  -h, --help                 Print help
```
