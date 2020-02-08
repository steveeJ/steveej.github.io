#!/usr/bin/env sh
printf '' > index.toml
echo "files = [" >> index.toml
for file in *.toml; do
  if [[ "$file" != "index.toml" ]]; then
    printf '    "%s",\n' $file >> index.toml
  fi
done
echo "]" >> index.toml
