#!/bin/bash

# sc-meta account --api https://devnet-gateway.multiversx.com --address erd1qqqqqqqqqqqqqpgqyxel2ed6lf4q0wdprc9g2e0g6f7hujxnwadsm0ejj2 > bytes4.json
# sc-meta account --api https://devnet-gateway.multiversx.com --address erd1qqqqqqqqqqqqqpgqkgj6xrrg3qejh30kxxw4h7cg6aecyy7vwadsl9988w > bytes8.json
# sc-meta account --api https://devnet-gateway.multiversx.com --address erd1qqqqqqqqqqqqqpgq9m6p365l2dhl0u8pqr8ag00e8lt68umrwadse5c9u7 > bytes16.json
# sc-meta account --api https://devnet-gateway.multiversx.com --address erd1qqqqqqqqqqqqqpgq0uahuqnu0afru3803ueg7eklruea633lwadsaz3rh4 > bytes32.json


sc-meta account --api http://localhost:8085 --address erd1k0tdvsvsf84j8xkkjgjhg2m0kw3ltcev8utk5jc6lzeaqmmkwadsvfqus4 > owner.json
sc-meta account --api http://localhost:8085 --address erd1qqqqqqqqqqqqqpgquecqjfz44kml4xkulha43xfghueuxvlmwads6m765p > bytes4.json
sc-meta account --api http://localhost:8085 --address erd1qqqqqqqqqqqqqpgqdz4rsucjvgggavmjyw742lm6eh4pfm9twadsax0f8h > bytes8.json
sc-meta account --api http://localhost:8085 --address erd1qqqqqqqqqqqqqpgqxakesav5v2znzdpdkwsx53n4lqt3dqmawads8t4sx5 > bytes16.json
sc-meta account --api http://localhost:8085 --address erd1qqqqqqqqqqqqqpgqfexagv6z25a34u5x6rcdezntsk7l09hrwadslvzet4 > bytes32.json
