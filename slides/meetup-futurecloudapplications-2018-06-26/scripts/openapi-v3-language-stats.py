#!/usr/bin/env python3

import urllib3
import re

http = urllib3.PoolManager()
r = http.request(
    'GET',
    'https://raw.githubusercontent.com/OAI/OpenAPI-Specification/master/IMPLEMENTATIONS.md'
)

resultsByHeader = {}
resultsByLanguage = {}
currentHeader = None

for line in r.data.decode('utf-8').splitlines():
    if line.startswith('#'):
        currentHeader = re.sub('^#+ +', '', line)

        if currentHeader == 'Implementations':
            continue

        if currentHeader not in resultsByHeader:
            resultsByHeader[currentHeader] = {}

    elif line.startswith('|-') or line.startswith('| Title'):
        continue

    elif line.startswith('| ') and currentHeader is not None:
        cols = line.split('|')
        title = cols[1].replace(' ', '')
        languages = cols[3].split(',')
        for language in languages:
            language = language.replace(' ', '')
            if language != '':
                if language not in resultsByHeader[currentHeader]:
                    resultsByHeader[currentHeader][language] = 0
                if language not in resultsByLanguage:
                    resultsByLanguage[language] = 0

                resultsByHeader[currentHeader][language] += 1
                resultsByLanguage[language] += 1

if __name__ == '__main__':
    print(
        'Tool type',
        '|',
        'Number of Tools ',
    )
    print(
        '-',
        ' | ',
        '-',
    )
    for header, langCounts in resultsByHeader.items():
        headerItems = 0
        for language, counts in langCounts.items():
            headerItems += 1
        print(
            header,
            ' | ',
            headerItems,
        )

    print(
        'Language',
        '|',
        'Number of Tools ',
    )
    print(
        '-',
        ' | ',
        '-',
    )

    for language, counts in resultsByLanguage.items():
        print(language, ' | ', counts)