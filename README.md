Simple tool to categorize operations from PKO BP statement. Categorized transactions are grouping by month. Maybe in the future there will be option to define custom grouping rules. 

You can define rules in  `rules.json`, eg:

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
Defining `match_rules`, you can use Regex patterns (case-insensitive) in all fields.

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
