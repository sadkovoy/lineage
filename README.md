**Linege**


Usage:
```
    lineage --host <GITLAB_HOST> --project <PROJECT_NAME> --tag <TAG> --token <PERSONAL_ACCESS_TOKEN>
```

Flags:
```
         --help       Prints help information
    -V, --version    Prints version information
```

Options:
```
    -h, --host <GITLAB_HOST>               Target gitlab host
    -p, --project <PROJECT_NAME>           Your project name
    -t, --tag <TAG>                        Last deployed tag name
    -a, --token <PERSONAL_ACCESS_TOKEN>    Your GitLab access-token
                                           (https://<YOUR_GITLAB_HOST>/profile/personal_access_tokens)
```

Example:

```
lineage --host https://gitlab.example.com --token "SECRET_TOKEN" --project my_awesome_project --tag v18.4.2
```