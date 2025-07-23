# wait-cli
Wait for a file in path and continue using notify watcher. Nothing also

## How to:

```bash
wait-cli --watch-for /path/to/file.txt
```

## Example:
```bash
#!/bin/bash

file_path="/mnt/docker_compost/cLvF5Gh0LFPDys8wW1BcQzGz.wlg"

function panic_fn() {
  curl -s https://api.nasa.gov/cabine/?api_key=S7y4iz0Ocx8PTipY&feedtype=json&ver=1.0
  echo " Executed !"
}

echo "Waiting for ${file_path} ..."

wait-cli --watch-for "${file_path}"

panic_fn
```
