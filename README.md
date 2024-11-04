**Cox Command Line Tool**

`cox` is a CLI tool designed to parse logs containing JSON objects as strings, providing an efficient way to filter and query data within log files. With `cox`, you can quickly search for specific key-value pairs in log files, making it a powerful tool for developers and system administrators who need to analyze logs in real-time or troubleshoot complex issues.

### Usage
```bash
cox "[path-to-file]" "[query]"
```

#### Example:
```bash
cox api-test.log level=10
```

In this example, `cox` searches through `api-test.log` for entries where the `level` field is set to `10`.**Cox Command Line Tool**

`cox` is a CLI tool designed to parse logs containing JSON objects as strings, providing an efficient way to filter and query data within log files. With `cox`, you can quickly search for specific key-value pairs in log files, making it a powerful tool for developers and system administrators who need to analyze logs in real-time or troubleshoot complex issues.

### Usage
```bash
cox "[path-to-file]" "[query]"
```

#### Example:
```bash
cox api-test.log level=10
```

In this example, `cox` searches through `api-test.log` for entries where the `level` field is set to `10`.


### TODO
- [x] Task 1: Basic: cox "[path-to-file]" "[query]" ; eg: cox api-test.log level=10
- [ ] Task 2: Support of fulltext search
- [ ] Task 3: Support of /regex/
