import re
import csv
from pathlib import Path
from jproperties import Properties

file_re = re.compile("translations_(\\w+).properties")

languages = dict()

for file in Path(".").glob("translations_*.properties"):
    search = file_re.match(str(file))
    if search:
        lang = search.group(1)

        config = Properties()
        with file.open() as f:
            config.load(f.read())
        languages[lang] = config


keys = list(list(languages.values())[0].keys())
langs = list(languages.keys())

with open("translations_new.csv", "w", newline="") as f:
    writer = csv.writer(f)
    writer.writerow(["", *langs])

    for key in keys:
        row = ["." + key]
        for lang in langs:
            value = languages[lang][key][0]
            value = value.replace("\n", "\\n")
            row.append(value)
        writer.writerow(row)
