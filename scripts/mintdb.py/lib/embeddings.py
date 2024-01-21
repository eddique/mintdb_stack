import openai
import os
from dotenv import load_dotenv

load_dotenv()
openai.api_key = os.getenv("OPENAI_API_KEY")

def get_vector(text, model="text-embedding-ada-002"):
    text = text.replace("\n", " ")
    return openai.embeddings.create(input=[text], model=model).data[0].embedding

def create_document(id: str, title: str, content: str, link: str) -> dict:
    document = {
        "id": id,
        "title": title,
        "content": content,
        "link": link,
        "embedding": get_vector(content)
    }
    return document
