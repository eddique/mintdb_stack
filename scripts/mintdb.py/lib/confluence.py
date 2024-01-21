import os
import requests
import re
from tqdm import tqdm
from dotenv import load_dotenv
load_dotenv()

BASE_URL = os.environ.get("KB_URI")

def scrape_geekbot_knowledge_base(url, article_list=[], count=0):
    headers = {
        "Authorization": "Bearer " + os.environ.get("CONFLUENCE_API_KEY"),
        "Accept": "application/json"
    }

    try:
        r = requests.get(url, headers=headers)
        payload  = r.json()
    except Exception as e:
        print(f"Error fetching articles 😕, Cause: {e}")
    articles = payload["results"]
    count += 1
    for article in tqdm(articles, desc=f"Articles, page: {count}"):
        try:
            id = article["id"]
            title = article["title"]
            print(title)
            web_url = BASE_URL + article["_links"]["webui"]
            page_url = article["_links"]["self"] + "?expand=body.storage"
            req = requests.get(page_url, headers=headers)
            html_content = req.json()["body"]["storage"]["value"]
        except Exception as e:
            print(f"Error fetching article content 😕, Cause: {e}")
        
        plain_text = re.sub(r"&[^;\s]+;|<.*?>", "", html_content)
        plain_text = re.sub(r"([a-z]?[a-z])([A-Z][a-z])", r"\1. \2", plain_text)
    
        data_object = {
            "id": id,
            "title": title,
            "content": plain_text,
            "link": web_url,
        }
        article_list.append(data_object)
    if payload["_links"].get("next"):
        url = BASE_URL + payload["_links"]["next"]
        scrape_geekbot_knowledge_base(url, article_list, count)