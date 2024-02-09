from app import app
from lib import confluence
import csv
def main():
    try:
        articles = []
        url = f"https://confluence.gustocorp.com/rest/api/content/search?cql=ancestor=293014027"
        confluence.scrape_geekbot_knowledge_base(url=url, article_list=articles)
        with open('geekbot.csv', mode='w', newline="", encoding="utf-8") as f:
            writer = csv.writer(f)
            writer.writerow(["id", "title", "content", "url"])
            for article in articles:
                title = article["title"].replace("\n", "")
                content = article["content"].replace("\n", "")
                writer.writerow([article["id"], title, content, article["url"]])
    except Exception as e:
        print(e)

if __name__ == '__main__':
    main()