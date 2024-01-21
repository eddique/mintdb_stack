from lib import confluence, mintdb
from tqdm import tqdm

def scrape_upsert(idx: str, parent: str):
    articles = []
    url = f"https://confluence.gustocorp.com/rest/api/content/search?cql=ancestor={parent}"
    confluence.scrape_geekbot_knowledge_base(url=url, article_list=articles)
    for article in tqdm(articles, desc="Upserting to mintdb..."):
        mintdb.insert(idx, article["id"], article["title"], article["content"], article["link"])

def query(idx: str, text: str):
    mintdb.query(idx, text)