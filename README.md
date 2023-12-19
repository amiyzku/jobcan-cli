# Jobcan-cli

A command line tool to operate Jobcan.

## Usage

```bash
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

```bash
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
