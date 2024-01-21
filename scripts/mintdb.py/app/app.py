from lib import api
import argparse
def run():
    parser = argparse.ArgumentParser(description='A CLI to scrape confluence data, vectorize, and upsert to mintdb')
    parser.add_argument('command', choices=['scrape', 'migrate', 'drop', 'query'], help='Specify the action to perform')
    parser.add_argument('--idx', type=str, help='Specify the name of the index to upsert, migrate, query, or delete')
    parser.add_argument('--text', type=str, help='Text to query similarity against vectors')
    parser.add_argument('--parent', type=str, help='Specify the parent page for scraping child articles in Confluence')

    args = parser.parse_args()
    parent = args.parent if args.parent else "293014027"
    idx = args.idx if args.idx else "it"
    text = args.text if args.text else "What's the wifi password for SF?"
    cmd = args.command

    if cmd == "scrape":
        api.scrape_upsert(idx, parent)
    elif cmd == "query":
        api.query(idx, text)