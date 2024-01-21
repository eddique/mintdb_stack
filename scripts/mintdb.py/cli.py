from app import app

def main():
    try:
        app.run()
    except Exception as e:
        print(e)

if __name__ == '__main__':
    main()