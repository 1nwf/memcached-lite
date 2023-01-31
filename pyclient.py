from pymemcache.client.base import Client


def main():
    client = Client("127.0.0.1:9889")
    key, val = "test_key", "test_val"
    res = client.set(key, val, noreply=False)
    assert res is True
    res = client.get(key)
    print(f"{key}: {res}")
    assert res == bytes(val, "ascii")
    res = client.delete(key, noreply=False)
    assert res is True
    res = client.delete("invalid_key", noreply=False)
    assert res is False


if __name__ == "__main__":
    main()
