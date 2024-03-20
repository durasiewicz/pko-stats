Simple tool to categorize operations from PKO BP statement. Categorized transactions are grouping by month. Maybe in the future there will be option to define custom grouping rules. 

You can set rules in  `rules.json`, eg:

```json
{
  "rules": [
    {
      "category_name": "Eating out",
      "match_rules": [
        {
          "transaction_type": "",
          "transaction_description": "McDonalds"
        },
        {
          "transaction_type": "",
          "transaction_description": "KFC"
        }
      ]
    },
    {
      "category_name": "Groceries",
      "match_rules": [
        {
          "transaction_type": "",
          "transaction_description": "Auchan"
        },
        {
          "transaction_type": "",
          "transaction_description": "Carrefour"
        },
        {
          "transaction_type": "",
          "transaction_description": "Lidl"
        }
      ]
    }
  ]
}
```
By default, application is looking for `rules.json` file on the same path as execetable. 

You can provide different path and/or filename, using `--category-rules` cmd line option. 

Defining `match_rules`, one can use Regex patterns (case-insensitive) in all fields.

As for now, only statements in XML format are supported.

```
Usage: pko-stats.exe [OPTIONS] <ACCOUNT_HISTORY>

Arguments:
  <ACCOUNT_HISTORY>

Options:
  -c, --category-rules <FILE>
  -s, --show-uncategorized
  -h, --help                   Print help
  -V, --version                Print version
  ```
