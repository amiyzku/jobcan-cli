# Jobcan cli

A tool to operate Jobcan from the command line.

## Usage

```bash
$ jobcan --help
Usage: jobcan <COMMAND>

Commands:
  work-start  Start work
  work-end    End work
  rest-start  Start rest
  rest-end    End rest
  status      Working status
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Run 'jobcan \<COMMAND> --help' for more information on a command.

E.g. 'jobcan work-start --help'

```bash
$ jobcan work-start --help
Start work

Usage: jobcan work-start [OPTIONS]

Options:
  -e, --email <EMAIL>        Account email. Default to $JOBCAN_EMAIL if not set.
  -p, --password <PASSWORD>  Account password. Default to $JOBCAN_PASSWORD if not set.
  -g, --group-id <GROUP_ID>  Group ID. Required if you belong to multiple groups. Default to $JOBCAN_GROUP_ID if not set.
  -n, --night-shift          Night-Shift mode.
  -h, --help                 Print help
```
