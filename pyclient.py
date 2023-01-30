from pymemcache.client.base import Client


def main():
    client = Client("localhost:9889")
    key, val = "test_key", "test_val"
    res = client.set(key, val, noreply=False)
    assert res is True
    res = client.get(key)
    print(f"{key}: {res}")
    assert res == bytes(val, "ascii")


if __name__ == "__main__":
    main()
