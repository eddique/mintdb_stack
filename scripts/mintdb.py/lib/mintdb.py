from lib import embeddings
import requests

def insert(idx: str, id: str, title: str, content: str, link: str):
    document = embeddings.create_document(id, title, content, link)
    data = {
        "idx": idx,
        "data": document
    }
    res = requests.post("http://localhost:3000/dev/insert", json=data).json()
    print(res)

def query(idx: str, content: str):
    embedding = embeddings.get_vector(content)
    print("got embedding")
    data = {
        "idx": idx,
        "embedding": embedding
    }
    res = requests.post("http://localhost:3000/dev/query", json=data).json()
    print(res)