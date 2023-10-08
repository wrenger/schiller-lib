import csv
import json
from pathlib import Path

# Define a dictionary to store translations
translations = {}

current_dir = Path(__file__).parent
csv_file_path = current_dir / "translations.csv"

# Read the CSV file and populate the translations dictionary
with open(csv_file_path, newline='', encoding='utf-8') as csvfile:
    csvreader = csv.reader(csvfile)
    for row in csvreader:
        if len(row) == 3:
            key, en, de = row
            en = en.replace('\\n', '\n')
            de = de.replace('\\n', '\n')
            if key:
                translations[key] = {'en': en, 'de': de}

# Create en.json with English translations
with open('en.json', 'w', encoding='utf-8') as en_json_file:
    en_translations = {key: lang['en'] for key, lang in translations.items()}
    json.dump(en_translations, en_json_file, indent=2, ensure_ascii=False)

# Create de.json with German translations
with open('de.json', 'w', encoding='utf-8') as de_json_file:
    de_translations = {key: lang['de'] for key, lang in translations.items()}
    json.dump(de_translations, de_json_file, indent=2, ensure_ascii=False)

print('JSON files (en.json and de.json) have been created.')
