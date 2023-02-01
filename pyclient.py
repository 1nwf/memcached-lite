from pymemcache.client.base import Client

server_addr = "127.0.0.1:9889"
client = Client(server_addr)


def main():
    key, val = "test_key", "test_val"
    set(key, val)
    get(key, val)
    append()
    prepend()
    replace(key, val)
    delete(key, val)
    flush()


def set(k, v):
    res = client.set(k, v, noreply=False)
    assert res is True


def get(k, v):
    res = client.get(k, v)
    print(f"{k}: {res}")
    assert res == bytes(v, "ascii")


def append():
    k, v = "append", "append_val"
    set(k, v)
    res = client.append(k, v, noreply=False)
    print(f"{k}: {res}")
    assert res is True


def prepend():
    k, v = "prepend", "prepend_val"
    set(k, v)
    res = client.prepend(k, "p ", noreply=False)
    print(f"{k}: {res}")
    assert res is True
    val = client.get(k)
    print("val: ", val)
    assert val == bytes(f"p {v}", "ascii")


def replace(k, v):
    res = client.replace(k, "value_replaced", noreply=False)
    assert res is True
    val = client.get(k)
    print(f"{k}: {res}")
    assert val == b"value_replaced"


def delete(k, v):
    res = client.delete(k, noreply=False)
    print(f"{k}: {res}")
    res = client.delete("invalid_key", noreply=False)
    assert res is False


def flush():
    res = client.flush_all()
    assert res is True


if __name__ == "__main__":
    main()
